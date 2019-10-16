#![forbid(unsafe_code)]
#![warn(clippy::all)]
//TODO: Re-enable once you got the time to fix this
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]

use franzi_client::Cluster;

use tracing_subscriber::{FmtSubscriber, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::builder()
    .with_env_filter(EnvFilter::new("TRACE"))
    .finish()).expect("tracing::subscriber::set_global_default");

    Cluster::connect(vec!["localhost:9092".into()]).await?;

    Ok(())
}
