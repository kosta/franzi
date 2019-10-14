#![forbid(unsafe_code)]

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


    eprintln!("Took {:#?}", start.elapsed());
    Ok(())
}
