#![forbid(unsafe_code)]
#![warn(clippy::all)]

use bytes::Bytes;
use franzi_base::types::KafkaString;
use franzi_proto::messages::api_versions::ApiVersionsRequestV2;
use franzi_proto::messages::list_offsets::*;
use std::net::SocketAddr;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let client_id: Bytes = b"franzi_test_client"[..].into();

    let (mut client, responses) =
        franzi_client::broker::connect(&"[::1]:9092".parse::<SocketAddr>()?).await?;
    tokio::spawn(async {
        responses
            .run()
            .await
            .expect("Error reading kafka responses")
    });
    let api_versions_response_v2 = client
        .send_one(ApiVersionsRequestV2 {}, client_id.clone())
        .await?;
    eprintln!("Took {:#?}", start.elapsed());
    eprintln!("Received response: {:#?}", api_versions_response_v2);

    // a second request
    let list_offsets_response_v4 = client
        .send_one(
            ListOffsetsRequestV4 {
                replica_id: -1,
                isolation_level: 0,
                topics: Some(vec![ListOffsetsRequestV4_topics {
                    topic: KafkaString(String::from("dfr_admitad_normfeed").into()),
                    partitions: Some(vec![ListOffsetsRequestV4_partitions {
                        partition: 0,
                        current_leader_epoch: -1, //?
                        timestamp: 0,             //?
                    }]),
                }]),
            },
            client_id.clone(),
        )
        .await?;

    eprintln!("Took {:#?}", start.elapsed());
    eprintln!(
        "Received ListOffets response: {:#?}",
        list_offsets_response_v4
    );
    Ok(())
}
