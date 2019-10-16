#![forbid(unsafe_code)]

use bytes::Bytes;
use franzi_base::types::KafkaString;
use franzi_proto::messages::api_versions::ApiVersionsRequestV2;
use franzi_proto::messages::list_offsets::*;
use franzi_client::Cluster;
use std::net::SocketAddr;
use std::time::Instant;

use tracing_subscriber::{FmtSubscriber, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::builder()
    .with_env_filter(EnvFilter::new("TRACE"))
    .finish()).expect("tracing::subscriber::set_global_default");

    let cluster = Cluster::connect(vec!["localhost:9092".into()]).await?;

    Ok(())
}
