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

    let topic_name = match std::env::args().skip(1).next() {
        None => panic!("Expected one command line argument: topic to fetch"),
        Some(topic_name) => topic_name,
    };

    let mut cluster = Cluster::connect(vec!["localhost:9092".into()]).await?;
    info!("cluster: {:?}", cluster);

    // loop {
    //     info!("got metadata: {:?}", cluster.metadata_v7(None).await);
    //     sleep(Duration::from_secs(10)).await;
    // }

    cluster.fetch_some(topic_name.into()).await?;
    Ok(())
}
