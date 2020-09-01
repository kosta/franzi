#![forbid(unsafe_code)]
#![warn(clippy::all)]
//TODO: Re-enable once you got the time to fix this
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]

use franzi_client::Cluster;
use tracing::info;

use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::new("DEBUG"))
            .finish(),
    )
    .expect("tracing::subscriber::set_global_default");

    let mut args = std::env::args().skip(1);

    let brokers: Vec<_> = args
        .next()
        .expect("Expected broker urls as first command line argument")
        .split(',')
        .map(String::from)
        .collect();
    let topic_name = args
        .next()
        .expect("Expected topic to fetch as second command line argument");

    let mut cluster = Cluster::connect(brokers).await?;
    info!("cluster: {:?}", cluster);

    // loop {
    //     info!("got metadata: {:?}", cluster.metadata_v7(None).await);
    //     sleep(Duration::from_secs(10)).await;
    // }

    // cluster.fetch_some(topic_name).await?;

    cluster.fetch_offsets("".into(), topic_name).await?;
    Ok(())
}
