use franzi_base::Error;
use franzi_proto::messages::metadata::MetadataRequestV0;
use futures::{future, Future};
use rand::seq::SliceRandom;
use std::collections::BTreeMap;
use std::net::SocketAddr;

pub mod broker;

#[derive(Debug, Default)]
pub struct BrokerInfo {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
    pub rack: Option<String>,
}

#[derive(Debug, Default)]
pub struct Client {
    pub brokers: BTreeMap<i32, BrokerInfo>,
    pub topic_leaders: BTreeMap<(String, i32), i32>,
    pub conns: BTreeMap<i32, broker::SharedBrokerClient>,
}

impl Client {
    /// tries to connect to each given broker once
    pub fn connect(mut addrs: Vec<SocketAddr>) -> impl Future<Item = Client, Error = Error> {
        addrs.shuffle(&mut rand::thread_rng());
        let broker_connection = future::loop_fn(addrs, |mut addrs| {
            // TODO: Turn into error
            let addr = addrs.pop().expect("Non-empty address list");
            broker::SharedBrokerClient::connect(&addr).then(|res| {
                match res {
                    Ok(client) => {
                        // we got a connection. return it
                        Ok(future::Loop::Break(client))
                    }
                    Err(e) => {
                        if addrs.is_empty() {
                            // all attempts exhausted. return last error.
                            Err(e)
                        } else {
                            // try next addr
                            Ok(future::Loop::Continue(addrs))
                        }
                    }
                }
            })
        });

        broker_connection
            .map_err(Error::from)
            .and_then(|broker| {
                broker
                    .send(MetadataRequestV0 { topics: Some(Vec::new()) })
                    .map(|response| (broker, response))
            })
            .and_then(|(_, response)| {
                let brokers = match response
                    .brokers
                    .unwrap_or_default()
                    .into_iter()
                    .map(|b| {
                        Ok((
                            b.node_id,
                            BrokerInfo {
                                node_id: b.node_id,
                                host: std::str::from_utf8(b.host.0.as_ref())?.to_string(),
                                port: b.port,
                                rack: None, // TODO
                            },
                        ))
                    })
                    .collect()
                {
                    Ok(brokers) => brokers,
                    Err(e) => return future::err(e),
                };
                future::ok(Client {
                    brokers,
                    topic_leaders: BTreeMap::default(),
                    // TODO: Figure out who this connection belongs to?
                    conns: BTreeMap::default(),
                })
            })
    }
}
