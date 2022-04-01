use async_graphql::{Data, Request, Variables};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use log::trace;
use p2panda_gql_replication::gql::EntryHash;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use p2panda_gql_replication::*;
use p2panda_gql_replication::gql::client::{GetEntryByHash, get_entry_by_hash};
use graphql_client::GraphQLQuery;
//use tide::{http::mime, Body, Response, StatusCode};

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

        let hash = EntryHash(vec![1,2,3]);
        let variables = get_entry_by_hash::Variables{ hash: Some(hash) };
        let query = GetEntryByHash::build_query(variables);
        //variables.

        //let query = "{
        //    entryByHash(hash: \"notahash\"){
        //        entry
        //    }
        //}
        //"
        //.to_owned();

        let var_vec = serde_json::to_vec(&query.variables).unwrap();
        let variables: serde_json::Value = serde_json::from_slice(&var_vec).unwrap();
        let variables = async_graphql::Variables::from_json(variables);

        let req = Request {
            query: query.query.to_owned(),
            operation_name: None,
            variables,
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
