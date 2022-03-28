use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use std::net::Ipv4Addr;
use std::net::SocketAddr;
//use tide::{http::mime, Body, Response, StatusCode};

mod db;
mod gql;
mod qp2p_server;

use db::ReplicationDb;
use gql::QueryRoot;
use qp2p_server::Qp2pServer;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(ReplicationDb::default())
        .finish();

    let peers = vec![SocketAddr::from((Ipv4Addr::LOCALHOST, 8099))];
    let (server, _endpoint) = Qp2pServer::new(peers, schema, None)
        .await
        .expect("server failed to start");
    server
        .serve()
        .await
        .expect("expected serve to complete cleanly");

    Ok(())
}
