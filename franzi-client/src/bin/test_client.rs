#![forbid(unsafe_code)]
#![warn(clippy::all)]
//TODO: Re-enable once you got the time to fix this
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]

use franzi_client::Cluster;
use std::time::Duration;
use tracing::info;

use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub async fn sleep(amount: Duration) {
    tokio::timer::delay(tokio::clock::now() + amount).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::new("DEBUG"))
            .finish(),
    )
    .expect("tracing::subscriber::set_global_default");

    let cluster = Cluster::connect(vec!["localhost:9092".into()]).await?;
    info!("cluster: {:?}", cluster);

    loop {
        info!("got metadata: {:?}", cluster.metadata_v7(None).await);
        sleep(Duration::from_secs(10)).await;
    }
}
