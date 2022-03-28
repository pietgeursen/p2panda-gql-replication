use crate::gql::SchemaType;
use async_graphql::{Request, Response};
use futures::{prelude::*, TryFutureExt};
use log::{error, trace, warn};
use qp2p::{Config, ConnectionError, Endpoint, EndpointError, IncomingConnections};
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;
// TODO remove me
// qp2p needs to know about its peers to initiate connections
pub struct Qp2pServer {
    pub peers: Vec<SocketAddr>,
    pub incoming_conns: IncomingConnections,
    pub schema: SchemaType,
}

impl Qp2pServer {
    // TODO: peers? remove?
    pub async fn new(
        peers: Vec<SocketAddr>,
        schema: SchemaType,
        config: Option<Config>,
    ) -> Result<(Self, Endpoint), EndpointError> {
        trace!("new qp2p server");

        let config = config.unwrap_or_else(|| {
            Config {
                idle_timeout: Duration::from_secs(60 * 60).into(), // 1 hour idle timeout.
                ..Default::default()
            }
        });
        let (endpoint, incoming_conns, _contact) =
            Endpoint::new_peer(SocketAddr::from((Ipv4Addr::LOCALHOST, 8099)), &[], config).await?;

        let local_addr = endpoint.local_addr();
        let public_addr = endpoint.public_addr();

        trace!(
            "started qp2p endpoint with local_addr: {:?}, public_addr: {:?}",
            local_addr,
            public_addr
        );

        Ok((
            Qp2pServer {
                peers,
                incoming_conns,
                schema,
            },
            endpoint,
        ))
    }

    pub async fn serve(mut self) -> Result<(), ConnectionError> {
        loop {
            let (connection, mut incoming_messages) = match self.incoming_conns.next().await {
                Some((connection, incoming_messages)) => {
                    trace!("opened connection!");
                    (connection, incoming_messages)
                }
                None => {
                    error!("connection open failed");
                    break;
                }
            };

            loop {
                match incoming_messages.next().await {
                    Ok(Some(bytes)) => {
                        trace!("received bytes: {:?}", bytes);
                        match serde_json::from_slice::<Request>(&bytes) {
                            Ok(request) => {
                                self.schema
                                    .execute(request)
                                    .then({
                                        let connection = connection.clone();
                                        |response: Response| async move {
                                            let bytes = serde_json::to_vec(&response)
                                                .expect("to be able to serialize response");
                                            connection.send(bytes.into()).await
                                        }
                                    })
                                    .unwrap_or_else(|err| {
                                        warn!("couldn't send response: {:?}", err);
                                    })
                                    .await;
                            }
                            Err(err) => {
                                error!("err, couldn't deserialize request. Err: {:?}", err);
                            }
                        };
                    }
                    Ok(None) => {
                        trace!("no more messages from remote peer");
                        break;
                    }
                    Err(err) => {
                        error!("Error receiving from stream: {:?}", err);
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
