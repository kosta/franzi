#![forbid(unsafe_code)]
#![warn(clippy::all)]
//TODO: Re-enable once you got the time to fix this
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]

use bytes::{Bytes, BytesMut};
use debug_stub_derive::DebugStub;
use franzi_base::{types::KafkaString, Error as KafkaError, KafkaRequest};
use franzi_proto::messages::offset_fetch::OffsetFetchRequestV5;
use franzi_proto::messages::offset_fetch::OffsetFetchRequestV5_partitions;
use franzi_proto::messages::offset_fetch::OffsetFetchRequestV5_topics;
use franzi_proto::{
    exchange,
    exchange::Exchange,
    messages::{
        find_coordinator::FindCoordinatorRequestV2,
        join_group::{JoinGroupRequestV3, JoinGroupRequestV3_group_protocols},
        metadata::{MetadataRequestV7, MetadataResponseV7, MetadataResponseV7_brokers},
    },
};
use futures::{channel::mpsc, SinkExt, Stream};
use rand::seq::{IteratorRandom, SliceRandom};
use std::{collections::BTreeMap, convert::From, fmt, fmt::Debug, io, sync::Arc};
use tracing::{debug, event, span, trace, Level, Span};
use tracing_futures::Instrument;

pub mod broker;

#[derive(Clone, Debug, Default)]
pub struct BrokerInfo {
    pub node_id: i32,
    /// host:port
    pub host: String,
    pub rack: Option<String>,
}

#[derive(Clone)]
pub struct BrokerChannel {
    hostname: String,
    channel: mpsc::Sender<exchange::Exchange>,
}

impl Debug for BrokerChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BrokerChannel")
            .field("hostname", &self.hostname)
            .field("channel", if self.channel.is_closed() { &"closed" } else { &"open" })
            .finish()
    }
}

impl BrokerChannel {
    pub async fn send_one<Request: KafkaRequest>(
        &mut self,
        request: &Request,
        client_id: Bytes, //TODO: Atomic?
    ) -> Result<Request::Response, KafkaError> {
        let broker_host = &self.hostname;
        event!(Level::TRACE, ?broker_host, "Sending request {:?}", request);
        let (request, response) = exchange::make_exchange(request, client_id);
        self.channel.send(request).await.map_err(KafkaError::from)?;
        let response = response.await;
        event!(Level::TRACE, ?broker_host, "Received response {:?}", response);
        response
    }
}

#[derive(Debug, Default)]
pub struct ClusterConfig {
    pub bootstrap_addrs: Vec<String>,
    pub client_id: Bytes,
}

#[derive(Debug)]
struct LeaderEpoch {
    broker_id: i32,
    leader_epoch: i32,
}

// TODO: Better Debug formatting...
#[derive(DebugStub, Default)]
pub struct Cluster {
    config: ClusterConfig,
    brokers: BTreeMap<i32, BrokerInfo>,
    topic_leaders: BTreeMap<(String, i32), LeaderEpoch>,
    #[debug_stub = "conns_by_id"]
    conns_by_id: BTreeMap<i32, BrokerChannel>,
    #[debug_stub = "conns_by_host"]
    conns_by_host: BTreeMap<String, BrokerChannel>,
}

#[derive(Debug)]
pub enum ConnectError {
    EmptyBootstrapServers(),
    Io(io::Error),
    KafkaError(KafkaError),
}

impl From<KafkaError> for ConnectError {
    fn from(e: KafkaError) -> ConnectError {
        ConnectError::KafkaError(e)
    }
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectError::EmptyBootstrapServers() => write!(f, "no bootstrap servers specified"),
            ConnectError::Io(e) => write!(f, "Error connecting to server: {}", e),
            ConnectError::KafkaError(e) => write!(f, "Error trying to speak with server: {}", e),
        }
    }
}

impl std::error::Error for ConnectError {
    fn description(&self) -> &str {
        match self {
            ConnectError::EmptyBootstrapServers() => "no bootstrap servers specified",
            ConnectError::Io(_) => "Error connecting to server",
            ConnectError::KafkaError(_) => "Error trying to speak with server",
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

fn spawn_off_broker_responses<St>(responses: broker::BrokerResponses<St>, span: Span)
where
    St: Stream<Item = Result<BytesMut, io::Error>> + Unpin + Send + Sync + 'static,
{
    tokio::spawn(
        async {
            event!(Level::DEBUG, "handling broker responses");
            let result = responses.run().await;
            event!(Level::DEBUG, ?result, "broker response result");
        }
        .instrument(span),
    );
}

pub struct ConsumerGroupMembership(Arc<()>);

#[allow(clippy::cognitive_complexity)] // Ooops...
fn spawn_off_broker_sink(
    mut sink: Option<broker::BrokerTcpSink>,
    addr: String,
    rx: mpsc::Receiver<Exchange>,
    span: Span,
) {
    let span_inner = span.clone();
    use futures::StreamExt;
    let mut rx = rx.map(Result::Ok);
    tokio::spawn(
        async move {
            event!(Level::DEBUG, "handling broker requests");
            loop {
                if let Some(mut sink) = sink {
                    let result = sink.send_all(&mut rx).await;
                    event!(Level::DEBUG, ?result, "broker requests result");
                    // either all tx are dropped (and rx returned None) => result Ok
                    // or sink is broken => result Err
                    if result.is_ok() {
                        return;
                    }
                }
                loop {
                    // reconnect
                    match broker::connect(&addr).await {
                        Ok((client, responses)) => {
                            event!(Level::DEBUG, "(Re)connected");
                            sink = Some(client);
                            spawn_off_broker_responses(responses, span_inner.clone());
                            break; // reconnect loop
                        }
                        Err(e) => {
                            event!(Level::DEBUG, ?e, "Error reconnecting...");
                            // TODO: Configurable exponential backoff
                            let amount = std::time::Duration::from_secs(30);
                            tokio::time::sleep(amount).await
                        }
                    }
                }
            }
        }
        .instrument(span),
    );
}

impl Cluster {
    /// tries to connect to each given broker once, returning the last error or the cluster
    #[allow(clippy::cognitive_complexity)] // oops :)
    pub async fn connect(mut addrs: Vec<String>) -> Result<Cluster, ConnectError> {
        let span = span!(Level::INFO, "connect", ?addrs);
        let _enter = span.enter();

        addrs.shuffle(&mut rand::thread_rng());

        // TODO: Make connect() a method on ClusterConfig
        let mut client = Cluster {
            config: ClusterConfig {
                bootstrap_addrs: addrs,
                client_id: b"franzi_test_client"[..].into(),
            },
            ..Default::default()
        };

        let mut channel = None;
        let mut address = None;

        for addr in &client.config.bootstrap_addrs {
            let connection_span = span!(parent: &span, Level::INFO, "connection", ?addr);
            let _ = span.enter();
            match broker::connect(addr).await {
                // store last error
                Err(e) => {
                    event!(Level::DEBUG, ?addr, error = ?e, "Connection error");
                    channel = Some(Err(e));
                }
                Ok((sink, responses)) => {
                    event!(Level::DEBUG, ?addr, "Connected");
                    spawn_off_broker_responses(responses, connection_span.clone());
                    let (tx, rx) = mpsc::channel::<exchange::Exchange>(0);
                    spawn_off_broker_sink(Some(sink), addr.clone(), rx, connection_span.clone());
                    client.conns_by_host.insert(addr.to_string(), BrokerChannel{
                        hostname: addr.to_string(),
                        channel: tx.clone(),
                    });
                    channel = Some(Ok(tx));
                    address = Some(addr.to_string());
                    break;
                }
            }
        }

        let mut channel = match channel {
            None => return Err(ConnectError::EmptyBootstrapServers()),
            Some(Err(e)) => return Err(ConnectError::Io(e)),
            Some(Ok(ch)) => ch,
        };

        let (request, response) = exchange::make_exchange(
            &MetadataRequestV7 {
                topics: Some(Vec::new()),
                allow_auto_topic_creation: false,
            },
            client.config.client_id.clone(),
        );
        channel.send(request).await.map_err(KafkaError::from)?;

        let response = response.await;
        event!(Level::DEBUG, ?response);
        let response = response?;

        client.fill_brokers(&response.brokers)?;
        let address = address.expect("Got a connection but no address");
        if let Some(broker) = client
            .brokers
            .values_mut()
            .find(|info| info.host == address)
        {
            client.conns_by_id.insert(broker.node_id, BrokerChannel{
                hostname: address.clone(),
                channel: channel.clone(),
            });
        }

        Ok(client)
    }

    fn fill_brokers(
        &mut self,
        brokers: &Option<Vec<MetadataResponseV7_brokers>>,
    ) -> Result<(), KafkaError> {
        let brokers = match brokers {
            None => return Ok(()),
            Some(b) => b,
        };
        let mut new_brokers = BTreeMap::<i32, BrokerInfo>::new();
        for broker in brokers {
            let broker_info = BrokerInfo {
                node_id: broker.node_id,
                host: format!(
                    "{}:{}",
                    std::str::from_utf8(broker.host.0.as_ref())?.to_string(),
                    broker.port
                ),
                rack: broker
                    .rack
                    .as_ref()
                    .map(|rack| std::str::from_utf8(&rack.0).map(String::from))
                    .transpose()?,
            };
            new_brokers.insert(broker.node_id, broker_info);
        }
        std::mem::swap(&mut new_brokers, &mut self.brokers);
        Ok(())
    }

    // select a random (hopefully) established connection
    // use conns_by_host because conns_by_id does not need to be filled
    // TODO: if that connection is down, we probably need to timeout and try other connections if possible
    // Idea: we can timeout on the `tx.send(request)` and select another (connected) broker,
    // so we can choose a smaller timeout than if we timeout on `response.await`
    fn random_broker(&self) -> BrokerChannel {
        self
            .conns_by_host
            .values()
            .choose(&mut rand::thread_rng())
            // Using expect because this should never happen. At the very least, the bootstrap brokers should be known
            // TODO: Revisit when we disconnect brokers we're not interested in
            .expect("random_broker: No broker available")
            .clone()
    }

    /// Fetches metadata for the given topics using a Metadata Request V7.
    /// If topics is None, fetches metadata for _all_ topics
    pub async fn metadata_v7(
        &mut self,
        topics: Option<Vec<String>>,
    ) -> Result<MetadataResponseV7, KafkaError> {
        let response = self.random_broker().send_one(&MetadataRequestV7 {
                topics: topics.map(|xs| xs.into_iter().map(KafkaString::from).collect()),
                allow_auto_topic_creation: false,
            },
            self.config.client_id.clone(),
        ).await?;
        // debug!("Got metadata_v7 response: {:?}", response);
        self.fill_brokers(&response.brokers)?;
        Ok(response)
    }

    // TODO: Naming! Conn or BrokerChannel?
    /// Returns the BrokerChannel for that broker id.
    pub fn get_conn(&mut self, id: i32) -> Option<BrokerChannel> {
        if let Some(conn) = self.conns_by_id.get(&id) {
            return Some(conn.clone());
        }

        // No connection yet -> establish one!
        let broker_info = self.brokers.get(&id)?;

        let addr = &broker_info.host;
        let span = span!(Level::INFO, "connection", ?addr);
        event!(
            Level::DEBUG,
            ?addr,
            "Creating connection for to broker id {} {:?}",
            id,
            addr
        );
        let (tx, rx) = mpsc::channel::<exchange::Exchange>(0);
        spawn_off_broker_sink(None, addr.clone(), rx, span);
        let broker_channel = BrokerChannel{
            hostname: addr.clone(),
            channel: tx,
        };
        self.conns_by_id.insert(id, broker_channel.clone());
        self.conns_by_host.insert(addr.clone(), broker_channel.clone());

        Some(broker_channel)
    }

    pub async fn fetch_some(&mut self, topic: String) -> Result<(), KafkaError> {
        let metadata = self.metadata_v7(Some(vec![topic.clone()])).await?;
        for topic_metadata in metadata.topic_metadata.unwrap_or_default() {
            if topic_metadata.error_code != 0 {
                return Err(KafkaError::Protocol(topic_metadata.error_code));
            }
            for partition_metadata in topic_metadata.partition_metadata.unwrap_or_default() {
                if partition_metadata.partition != 95 {
                    continue;
                }
                // TODO: Can I ask ISRs as well?
                let mut broker = self.get_conn(partition_metadata.leader).ok_or(KafkaError::UnknownBrokerId(partition_metadata.leader))?;
                let request = franzi_proto::messages::fetch::FetchRequestV6 {
                        replica_id: -1,
                        max_wait_time: 30_000, //ms
                        min_bytes: 0,
                        max_bytes: 10_000,
                        isolation_level: 0,
                        topics: Some(vec![franzi_proto::messages::fetch::FetchRequestV6_topics {
                            topic: topic.clone().into(),
                            partitions: Some(vec![
                                franzi_proto::messages::fetch::FetchRequestV6_partitions {
                                    partition: partition_metadata.partition,
                                    fetch_offset: 0,
                                    log_start_offset: 0,
                                    partition_max_bytes: 10_000,
                                },
                            ]),
                        }]),
                    };
                broker.send_one(&request, self.config.client_id.clone()).await.map_err(KafkaError::from)?;
            }
        }
        Ok(())
    }

    // sarama consumes like this:
    // - FindCoordinatorRequest for consumer group to any broker;
    //   -> Response contains broker; remember Broker and Coordinator
    // - JoinGroupRequest with groupID, memberID (initially empty), timeout (in seconds?), ProtocolType: "consumer"
    //   meta contains topics, UserData (from config, optional), strategy ("range" or "roundrobin")
    //   -> response contains leader_id, member_id (if identical: we are the leader), members(?)
    //      - if leader: get members as type ConsumerGroupMemberMetadata struct { Version  int16, Topics   []string, UserData []byte}
    //        then balance according to strategy
    // - SyncGroupRequest
    //   - groupID, memberID, generationID
    //   - if leader, include plan with type ConsumerGroupMemberAssignment struct { Version  int16, Topics   map[string][]int32, UserData []byte }
    //     -> response contains ConsumerGroupMemberAssignment (for this member)
    // TODO: When a non-leader joins, how does the leader balance it?
    // - for each claim:
    //   - start heartbeat loop: (every heartbeat interval)
    //     send HeartbeatRequest with groupID, memberID, generation to coordinator
    //   - send OffsetFetchRequest _for joined group_, now we have a starting offset, watermarks, ... (yay!)
    //   - start FetchRequests

    async fn join_consumer_group(
        &mut self,
        group_id: String,
        topics: Vec<String>,
    ) -> Result<ConsumerGroupMembership, KafkaError> {
        let request = FindCoordinatorRequestV2 {
            coordinator_key: group_id.clone().into(),
            coordinator_type: 0, // (0 = group, 1 = transaction)
        };
        let mut broker_channel = self.random_broker();
        let response = broker_channel
            .send_one(&request, self.config.client_id.clone())
            .await?;

        if response.error_code != 0 {
            return Err(KafkaError::Protocol(response.error_code));
        }

        // TODO: update broker cache in case we don't know this broker?
        let coordinator_id = response.coordinator.node_id;

        let mut broker = self.get_conn(coordinator_id).ok_or(KafkaError::UnknownBrokerId(coordinator_id))?;
        let request = JoinGroupRequestV3 {
            group_id: group_id.into(),
            // TODO: Move both timeouts to config
            session_timeout: std::time::Duration::from_secs(10).as_millis() as i32,
            rebalance_timeout: std::time::Duration::from_secs(60).as_millis() as i32,
            member_id: "".into(),
            protocol_type: "consumer".into(),
            group_protocols: Some(vec![
                JoinGroupRequestV3_group_protocols{
                    protocol_name: "range".into(),
                    protocol_metadata: Default::default(),
                },
                JoinGroupRequestV3_group_protocols{
                    protocol_name: "roundrobin".into(),
                    protocol_metadata: Default::default(),
                },
            ]),
        };
        let response = broker.send_one(&request, self.config.client_id.clone()).await?;

        unimplemented!()
    }

    pub async fn fetch_offsets(
        &mut self,
        group_id: String,
        topic: String,
    ) -> Result<(), KafkaError> {
        self.join_consumer_group(group_id, vec![topic]).await?;

        // // TODO: Cache metadata!
        // let metadata = self.metadata_v7(Some(vec![topic.clone()])).await?;
        // let topic_metadata = metadata.topic_metadata.unwrap_or_default();

        // let mut all_partitions = Vec::new();
        // let mut partitions_by_leader: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        // for topic_metadata in topic_metadata {
        //     if topic_metadata.error_code != 0 {
        //         return Err(KafkaError::Protocol(topic_metadata.error_code));
        //     }
        //     for partition_metadata in topic_metadata.partition_metadata.unwrap_or_default() {
        //         partitions_by_leader
        //             .entry(partition_metadata.leader)
        //             .or_default()
        //             .push(partition_metadata.partition);
        //         all_partitions.push(partition_metadata.partition);
        //     }
        // }

        // debug!(
        //     "fetch_offsets: partitions_by_leader: {:?}",
        //     partitions_by_leader
        // );

        // for (leader, _partitions) in partitions_by_leader {
        //     let mut broker = self.get_conn(leader);
        //     let request = OffsetFetchRequestV5 {
        //         group_id: group_id.clone().into(),
        //         topics: Some(vec![OffsetFetchRequestV5_topics {
        //             topic: topic.clone().into(),
        //             partitions: Some(
        //                 all_partitions
        //                     .iter()
        //                     .map(|partition| OffsetFetchRequestV5_partitions {
        //                         partition: *partition,
        //                     })
        //                     .collect(),
        //             ),
        //         }]),
        //     };
        //     debug!("Sending to broker {}: {:?}", leader, request);
        //     let (request, response) =
        //         exchange::make_exchange(&request, self.config.client_id.clone());
        //     broker.send(request).await.map_err(KafkaError::from)?;
        //     let response = response.await?;
        //     event!(Level::DEBUG, ?response, "Got response from broker {:?}", leader);
        // }
        Ok(())
    }
}
