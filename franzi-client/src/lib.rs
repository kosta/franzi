use franzi_base::Error as KafkaError;
use franzi_proto::messages::metadata::MetadataRequestV0;
use futures::{channel::mpsc, future, Future, Sink, SinkExt, StreamExt};
use rand::seq::SliceRandom;
use std::{
    boxed::Box,
    convert::From,
    collections::BTreeMap,
    io,
    fmt::Debug
};

pub mod broker;

#[derive(Debug, Default)]
pub struct BrokerInfo {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
    pub rack: Option<String>,
}

type KafkaProtocolRequestSender = mpsc::Sender<broker::KafkaProtocolRequest>;
type FromSendError = fn(mpsc::SendError) -> KafkaError;
// type BrokerChannel = futures::Sink::SinkMapErr<Error=KafkaError>;

#[derive(Debug, Default)]
pub struct Client {
    bootstrap_addrs: Vec<String>,
    brokers: BTreeMap<i32, BrokerInfo>,
    topic_leaders: BTreeMap<(String, i32), i32>,
    // conns_by_id: BTreeMap<i32, BrokerChannel>,
    // conns_by_host: BTreeMap<String, BrokerChannel>,
}

pub enum ConnectError {
    EmptyBootstrapServers(),
    IoError(io::Error),
}

impl Client {
    /// tries to connect to each given broker once, re
    pub async fn connect(mut addrs: Vec<String>) -> Result<Client, ConnectError> {
        if addrs.is_empty() {
            return Err(ConnectError::EmptyBootstrapServers());
        }
        addrs.shuffle(&mut rand::thread_rng());

        let client = Client{
            bootstrap_addrs: addrs,
            ..Default::default()
        };

        let mut error = None;

        for addr in &client.bootstrap_addrs {
            match broker::connect(addr).await {
                // store last error
                Err(e) => error = Some(e),
                Ok((broker_client, responses)) => {
                    // clear error
                    error = None;
                    // TODO: Connection between responses and client channel?
                    tokio::spawn(async {
                        responses.run().await.expect("Spawn tokio response")
                    });
                    let (tx, rx) = mpsc::channel::<broker::KafkaProtocolRequest>(1);
                    // let addr = addr.to_string();
                    // tokio::spawn(async move {
                    //     rx.forward(broker_client).await.expect("TODO: Handle broker errors");
                    // });
                    let tx = tx.sink_map_err(KafkaError::from);
                    // client.conns_by_host.insert(addr.to_string(), );
                    break
                }
            }
        };

        unimplemented!("todo");
        Ok(client)

            //     broker
            //         .send(MetadataRequestV0 { topics: Some(Vec::new()) })
            //         .map(|response| (broker, response))
            // })
            // .and_then(|(_, response)| {
            //     let brokers = match response
            //         .brokers
            //         .unwrap_or_default()
            //         .into_iter()
            //         .map(|b| {
            //             Ok((
            //                 b.node_id,
            //                 BrokerInfo {
            //                     node_id: b.node_id,
            //                     host: std::str::from_utf8(b.host.0.as_ref())?.to_string(),
            //                     port: b.port,
            //                     rack: None, // TODO
            //                 },
            //             ))
            //         })
            //         .collect()
            //     {
            //         Ok(brokers) => brokers,
            //         Err(e) => return future::err(e),
            //     };
            //     future::ok(Client {
            //         brokers,
            //         topic_leaders: BTreeMap::default(),
            //         // TODO: Figure out who this connection belongs to?
            //         conns: BTreeMap::default(),
            //     })
            // })
    }
}
