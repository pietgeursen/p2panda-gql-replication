use p2panda_gql_replication::*;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

fn main(){
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(ReplicationDb::default())
        .finish();

    let sdl = schema.sdl();

    println!("{}", sdl);
}
