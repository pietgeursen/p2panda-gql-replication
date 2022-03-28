use async_graphql::*;
use super::public_key::PublicKey;

#[derive(InputObject, SimpleObject)]
pub struct AliasedAuthor {
    public_key: PublicKey,
    alias: ID,
}
