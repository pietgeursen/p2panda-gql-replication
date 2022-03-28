use async_graphql::{Data, Request, Variables};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use log::trace;
use std::collections::HashMap;
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
    let (server, endpoint) = Qp2pServer::new(peers, schema, None)
        .await
        .expect("server failed to start");

    tokio::spawn(async move {
        let (conn, mut incoming) = endpoint
            .connect_to(&SocketAddr::from((Ipv4Addr::LOCALHOST, 8099)))
            .await
            .unwrap();

        let query = "{
            entryByHash(hash: \"notahash\"){
                entry
            }
        }
        "
        .to_owned();

        let req = Request {
            query,
            operation_name: None,
            variables: Variables::default(),
            uploads: Vec::new(),
            data: Data::default(),
            extensions: HashMap::default(),
            disable_introspection: true,
        };

        conn.send(serde_json::to_vec(&req).unwrap().into())
            .await
            .expect("to be able to send request");
        let response = incoming.next().await;
        trace!("response: {:?}", response);
    });

    server
        .serve()
        .await
        .expect("expected serve to complete cleanly");

    Ok(())
}
