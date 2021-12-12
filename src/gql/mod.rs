use async_graphql::{Context, Result};
use async_graphql::{InputObject, Object};
use futures::lock::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct Replication {
    pub author_aliases: Arc<Mutex<HashMap<String, Vec<[u8;32]>>>>,
}

pub struct EntryWithPayload<'a, 'b> {
    pub entry: &'a [u8],
    pub payload: &'b [u8],
}

#[Object]
impl<'a, 'b> EntryWithPayload<'a, 'b> {
    pub async fn entry(&self) -> &[u8] {
        &self.entry
    }
    pub async fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(InputObject, Debug)]
pub struct Author {
    pub key: Option<[u8; 32]>,
    pub alias: Option<u32>,
}

#[derive(InputObject, Debug)]
pub struct RangeRequest {
    pub author: Author,
    pub log_id: u64,
    pub sequence_start: u64,
    pub sequence_end: u64,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn get_single_entry<'a>(
        &self,
        ctx: &Context<'a>,
        author_alias_id: Option<String>,
        author: Author,
        log_id: u64,
        sequence: u64,
    ) -> Result<EntryWithPayload<'a, 'a>> {
        unimplemented!()
    }

    pub async fn get_entries_by_sequence_range<'a>(
        &self,
        ctx: &Context<'a>,
        author_alias_id: Option<String>,
        ranges: Vec<RangeRequest>,
    ) -> Result<Vec<EntryWithPayload<'a, 'a>>> {
        unimplemented!()
    }

    pub async fn get_all_entries_by_author<'a>(
        &self,
        ctx: &Context<'a>,
        author_alias_id: Option<String>,
        author: Author,
    ) -> Result<Vec<EntryWithPayload<'a, 'a>>> {
        unimplemented!()
    }
    // get_log_heights_deltas
    // is_author_alias_id_valid
}

#[derive(InputObject, Debug)]
pub struct AuthorAliases {
    /// A unique id to use to store the aliases by. You could use a hash of all the author keys or
    /// a uuid.
    author_alias_id: String,

    /// The zero based index of the author key is the alias.
    ///
    /// Eg if `aliases` is [ [<first author pub key>], [<second author pub key>]]
    /// Then you can refer to <second author pub key> using the alias 1
    aliases: Vec<[u8;32]>,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn set_author_aliases<'a>(
        &self,
        ctx: &Context<'a>,
        author_aliases: AuthorAliases,
    ) -> Result<bool> {
        let replication = ctx.data::<Replication>()?;
        replication
            .author_aliases
            .lock()
            .await
            .insert(author_aliases.author_alias_id, author_aliases.aliases);
        Ok(true)
    }
}
