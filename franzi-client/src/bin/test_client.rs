use franzi_client::Client;
use futures::future::Future;
use std::net::SocketAddr;

fn main() {
    let fut = Client::connect(vec!["127.0.0.1:9092".parse::<SocketAddr>().unwrap()])
        .map(|client| {
            eprintln!("Got client {:#?}", client);
        })
        .map_err(|e| panic!("Error: {:#?}", e));

    tokio::run(fut);
}
