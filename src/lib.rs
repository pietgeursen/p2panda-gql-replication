use async_graphql::{Data, Request, Variables};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use log::trace;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
//use tide::{http::mime, Body, Response, StatusCode};

pub mod db;
pub mod gql;
pub mod qp2p_server;

pub use db::ReplicationDb;
pub use gql::QueryRoot;
pub use qp2p_server::Qp2pServer;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
