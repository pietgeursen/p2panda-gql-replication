use async_graphql::*;
use super::bamboo_entry::BambooEntry;
use super::payload::Payload;

#[derive(SimpleObject)]
pub struct EntryAndPayload{
    pub entry: BambooEntry,
    pub payload: Payload
}
