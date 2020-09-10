#![forbid(unsafe_code)]
#![warn(clippy::all)]
//TODO: Re-enable once you got the time to fix this
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]

use franzi_client::Cluster;
use structopt::StructOpt;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Debug, StructOpt)]
enum Command {
    FetchMetadata {
        topic: String,
    },
    FetchOffsets {
        topic: String,
        #[structopt(default_value)]
        group_id: String,
    },
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Kafka bootstrap url to connect to (separated by commas)
    bootstrap: String,
    #[structopt(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::new("TRACE"))
            .finish(),
    )
    .expect("tracing::subscriber::set_global_default");

    let opt = Opt::from_args();

    let brokers = opt.bootstrap.split(',').map(String::from).collect();

    let mut cluster = Cluster::connect(brokers).await?;
    debug!("cluster: {:?}", cluster);

    use Command::*;
    match opt.command {
        FetchMetadata { topic } => {
            info!(
                "Metadata for {:?}: {:?}",
                topic.clone(),
                cluster.metadata_v7(Some(vec![topic])).await?
            );
        }
        FetchOffsets { topic, group_id } => {
            info!(
                "Offsets for {:?}: {:?}",
                topic.clone(),
                cluster.fetch_offsets(group_id, topic).await?
            );
        }
    }
    Ok(())
}
