#![forbid(unsafe_code)]
#![warn(clippy::all)]
//TODO: Re-enable once you got the time to fix this
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]

use bytes::{Bytes, BytesMut};
use debug_stub_derive::DebugStub;
use franzi_base::{types::KafkaString, Error as KafkaError};
use franzi_proto::{
    exchange,
    exchange::Exchange,
    messages::metadata::{MetadataRequestV7, MetadataResponseV7, MetadataResponseV7_brokers},
};
use futures::{channel::mpsc, SinkExt, Stream};
use rand::seq::{IteratorRandom, SliceRandom};
use std::{collections::BTreeMap, convert::From, fmt, fmt::Debug, io};
use tracing::{event, span, Level, Span};
use tracing_futures::Instrument;

pub mod broker;

#[derive(Clone, Debug, Default)]
pub struct BrokerInfo {
    pub node_id: i32,
    /// host:port
    pub host: String,
    pub rack: Option<String>,
}

type BrokerChannel = mpsc::Sender<exchange::Exchange>;

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
                            event!(Level::DEBUG, "Reconnected");
                            sink = Some(client);
                            spawn_off_broker_responses(responses, span_inner.clone());
                            break; // reconnect loop
                        }
                        Err(e) => {
                            event!(Level::DEBUG, ?e, "Error reconnecting...");
                            // TODO: Configurable exponential backoff
                            let amount = std::time::Duration::from_secs(30);
                            tokio::time::delay_for(amount).await
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
                    client.conns_by_host.insert(addr.to_string(), tx.clone());
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
            client.conns_by_id.insert(broker.node_id, channel.clone());
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
                    std::str::from_utf8(broker.host.0.as_ref())
                        .map_err(KafkaError::from)?
                        .to_string(),
                    broker.port
                ),
                rack: None, // TODO
            };
            new_brokers.insert(broker.node_id, broker_info);
        }
        std::mem::swap(&mut new_brokers, &mut self.brokers);
        Ok(())
    }

    /// Fetches metadata for the given topics using a Metadata Request V7.
    /// If topics is None, fetches metadata for _all_ topics
    pub async fn metadata_v7(
        &mut self,
        topics: Option<Vec<String>>,
    ) -> Result<MetadataResponseV7, KafkaError> {
        // select a random (hopefully) established connection
        // use conns_by_host because conns_by_id does not need to be filled
        // TODO: if that connection is down, we probably need to timeout and try other connections if possible
        // Idea: we can timeout on the `tx.send(request)` and select another (connected) broker,
        // so we can choose a smaller timeout than if we timeout on `response.await`
        let mut tx = self
            .conns_by_host
            .values()
            .choose(&mut rand::thread_rng())
            .expect("No connections available")
            .clone();
        let (request, response) = exchange::make_exchange(
            &MetadataRequestV7 {
                topics: topics.map(|xs| xs.into_iter().map(KafkaString::from).collect()),
                allow_auto_topic_creation: false,
            },
            self.config.client_id.clone(),
        );
        tx.send(request).await.map_err(KafkaError::from)?;
        let response = response.await?;
        self.fill_brokers(&response.brokers)?;
        Ok(response)
    }

    // TODO: Naming! Conn or BrokerChannel?
    /// Returns the BrokerChannel for that broker id. If the broker id is unknown, returns a closed channel.
    pub fn get_conn(&mut self, id: i32) -> BrokerChannel {
        if let Some(conn) = self.conns_by_id.get(&id) {
            return conn.clone();
        }

        // No connection yet -> establish one!
        let broker_info = match self.brokers.get(&id) {
            // Unknown broker id -> return closed channel
            None => return mpsc::channel(0).0,
            Some(b) => b,
        };

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
        self.conns_by_id.insert(id, tx.clone());
        self.conns_by_host.insert(addr.clone(), tx.clone());

        tx
    }

    pub async fn fetch_some(&mut self, topic: String) -> Result<(), KafkaError> {
        let metadata = self.metadata_v7(Some(vec![topic.clone()])).await?;
        for topic_metadata in metadata.topic_metadata.unwrap_or_default() {
            if topic_metadata.error_code != 0 {
                return Err(KafkaError::Protocol(topic_metadata.error_code));
            }
            for partition_metadata in topic_metadata.partition_metadata.unwrap_or_default() {
                // TODO: Can I ask ISRs as well?
                let mut broker = self.get_conn(partition_metadata.leader);
                let (request, response) = exchange::make_exchange(
                    &franzi_proto::messages::fetch::FetchRequestV6 {
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
                    },
                    self.config.client_id.clone(),
                );
                broker.send(request).await.map_err(KafkaError::from)?;
                let response = response.await?;
                event!(Level::DEBUG, ?response, "Got response");
            }
        }
        Ok(())
    }
}
