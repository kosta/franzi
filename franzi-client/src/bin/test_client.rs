use franzi_base::types::KafkaString;
use franzi_base::Error;
use franzi_client::SharedBrokerClient;
use franzi_proto::messages::api_versions::ApiVersionsRequestV2;
use franzi_proto::messages::list_offsets::*;
use futures::future::Future;
use std::net::SocketAddr;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let fut = SharedBrokerClient::connect(&"127.0.0.1:9092".parse::<SocketAddr>().unwrap())
        .map_err(|e| -> Error { panic!("connect error: {}", e) })
        .and_then(|client| {
            client
                .send(ApiVersionsRequestV2 {})
                .map(|response| (client, response))
        })
        .and_then(move |(client, response)| {
            eprintln!("Took {:#?}", start.elapsed());
            eprintln!("Received response: {:#?}", response);

            // a second request
            client.send(ListOffsetsRequestV4 {
                replica_id: -1,
                isolation_level: 0,
                topics: Some(vec![ListOffsetsRequestV4_topics {
                    topic: KafkaString(String::from("dcz_admitad_rawfeed").into()),
                    partitions: Some(vec![ListOffsetsRequestV4_partitions {
                        partition: 0,
                        current_leader_epoch: -1, //?
                        timestamp: 0,             //?
                    }]),
                }]),
            })
        })
        .map(move |response| {
            eprintln!("Took {:#?}", start.elapsed());
            eprintln!("Received ListOffets response: {:#?}", response);
        })
        .map_err(|e| panic!("Error: {}", e));

    tokio::run(fut);
}
