#![forbid(unsafe_code)]
#![warn(clippy::all)]
//TODO: Re-enable once you got the time to fix this
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]

use bytes::Bytes;
use debug_stub_derive::DebugStub;
use franzi_base::Error as KafkaError;
use franzi_proto::{
    exchange,
    messages::metadata::MetadataRequestV0,
};
use futures::{channel::mpsc, SinkExt, StreamExt};
use rand::seq::SliceRandom;
use std::{
    convert::From,
    collections::BTreeMap,
    fmt,
    io,
    fmt::Debug
};
use tracing::{event, span, Level};
use tracing_futures::Instrument;

pub mod broker;

#[derive(Debug, Default)]
pub struct BrokerInfo {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
    pub rack: Option<String>,
}

type BrokerChannel = mpsc::Sender<exchange::Exchange>;

#[derive(Debug, Default)]
pub struct ClusterConfig {
    pub bootstrap_addrs: Vec<String>,
    pub client_id: Bytes,
}

// TODO: Better Debug formatting...
#[derive(DebugStub, Default)]
pub struct Cluster {
    config: ClusterConfig,
    brokers: BTreeMap<i32, BrokerInfo>,
    topic_leaders: BTreeMap<(String, i32), i32>,
    #[debug_stub="conns_by_id"]
    conns_by_id: BTreeMap<i32, BrokerChannel>,
    #[debug_stub="conns_by_id"]
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
            ConnectError::KafkaError(_) => "Error trying to speak with server"
        }
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl Cluster {
    /// tries to connect to each given broker once, returning the last error or the cluster
    #[allow(clippy::cognitive_complexity)] // oops :)
    pub async fn connect(mut addrs: Vec<String>) -> Result<Cluster, ConnectError> {
        let span = span!(Level::INFO, "connect", ?addrs);
        let _enter = span.enter();

        addrs.shuffle(&mut rand::thread_rng());

        // TODO: Make connect() a method on ClusterConfig
        let mut client = Cluster{
            config: ClusterConfig{
                bootstrap_addrs: addrs,
                client_id: b"franzi_test_client"[..].into(),
            },
            ..Default::default()
        };

        let mut channel = None;

        for addr in &client.config.bootstrap_addrs {
            let connection_span = span!(parent: &span, Level::INFO, "connection", ?addr);
            let _ = span.enter();
            match broker::connect(addr).await {
                // store last error
                Err(e) => {
                    event!(Level::DEBUG, ?addr, error = ?e, "Connection error");
                    channel = Some(Err(e));
                },
                Ok((broker_client, responses)) => {
                    event!(Level::DEBUG, ?addr, "Connected");
                    // TODO: Connection between responses and client channel?
                    tokio::spawn(async {
                        event!(Level::DEBUG, "handling broker responses");
                        responses.run().await.expect("TODO: Handle broker responses error")
                    }.instrument(connection_span.clone()));
                    let (tx, rx) = mpsc::channel::<exchange::Exchange>(1);
                    // let addr = addr.to_string();
                    tokio::spawn(async move {
                        event!(Level::DEBUG, "handling broker requests");
                        rx.map(Ok).forward(broker_client).await.expect("TODO: Handle broker request errors");
                    }.instrument(connection_span.clone()));
                    client.conns_by_host.insert(addr.to_string(), tx.clone());
                    channel = Some(Ok(tx));
                    break
                }
            }
        };

        let mut channel = match channel {
            None => return Err(ConnectError::EmptyBootstrapServers()),
            Some(Err(e)) => return Err(ConnectError::Io(e)),
            Some(Ok(ch)) => ch,
        };

        let (request, response) =exchange::make_exchange(&MetadataRequestV0 { topics: Some(Vec::new()) }, client.config.client_id.clone());
        channel.send(request).await.map_err(KafkaError::from)?;

        let response = response.await;
        event!(Level::DEBUG, ?response);
        let response = response?;

        for broker in response.brokers.unwrap_or_default() {
            client.brokers.insert(broker.node_id, BrokerInfo{
                node_id: broker.node_id,
                host: std::str::from_utf8(broker.host.0.as_ref()).map_err(KafkaError::from)?.to_string(),
                port: broker.port,
                rack: None, // TODO
            });
        }

        Ok(client)
    }
}
