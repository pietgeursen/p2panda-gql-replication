use async_graphql::{Context, Result};
use async_graphql::{InputObject, Object};
use futures::lock::Mutex;
use std::sync::Arc;
use lru::LruCache;

pub struct Replication {
    pub author_aliases: Arc<Mutex<LruCache<String, Vec<[u8;32]>>>>,
}

impl Replication {
    pub fn new(lru_size: usize) -> Self {
        Self { author_aliases: Arc::new(Mutex::new(LruCache::new(lru_size))) }
    }
}

impl Default for Replication {
    fn default() -> Self {
        Self::new(100)
    }
}

pub struct EntryWithPayload<'a, 'b> {
    pub entry: &'a [u8],
    pub payload: &'b [u8],
}

#[Object]
/// A bamboo entry and its payload
impl<'a, 'b> EntryWithPayload<'a, 'b> {
    pub async fn entry(&self) -> &[u8] {
        &self.entry
    }
    pub async fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(InputObject, Debug)]
/// Either the key or the alias of an author.
///
/// - You must set **one of** key or alias.
pub struct Author {
    /// The author's public key
    pub key: Option<[u8; 32]>,
    /// The author's alias
    pub alias: Option<u32>,
}

#[derive(InputObject, Debug)]
pub struct RangeRequest {
    /// The author of the feed.
    pub author: Author,
    /// The log id of the feed
    pub log_id: u64,
    /// The starting sequence number of the requested feed.
    pub sequence_start: u64,
    /// The end sequence of the requested feed. If not provided then get until the end of their
    /// feed.
    pub sequence_end: Option<u64>,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {

    /// Request a single entry by author, log and sequence
    pub async fn get_single_entry<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "The author alias id you're using, if any.")]
        author_alias_id: Option<String>,
        #[graphql(desc = "The author of the entry.")]
        author: Author,
        #[graphql(desc = "The log Id of the entry")]
        log_id: u64,
        #[graphql(desc = "The sequence number of the entry.")]
        sequence: u64,
    ) -> Result<EntryWithPayload<'a, 'a>> {
        unimplemented!()
    }

    /// Request collections of entries by an author and log, specified by a range of sequence
    /// numbers. 
    pub async fn get_entries_by_sequence_range<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "The author alias id you're using, if any.")]
        author_alias_id: Option<String>,
        ranges: Vec<RangeRequest>,
    ) -> Result<Vec<EntryWithPayload<'a, 'a>>> {
        unimplemented!()
    }

    /// Request all entries of all logs published by the requested author
    pub async fn get_all_entries_by_author<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "The author alias id you're using, if any.")]
        author_alias_id: Option<String>,
        #[graphql(desc = "The author of the entry.")]
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
    /// Set a collection of author aliases.
    ///
    /// Use this to minimise bandwidth in future requests and responses
    pub async fn set_author_aliases<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "The collection of authors to alias.")]
        author_aliases: AuthorAliases,
    ) -> Result<bool> {
        let replication = ctx.data::<Replication>()?;
        replication
            .author_aliases
            .lock()
            .await
            .put(author_aliases.author_alias_id, author_aliases.aliases);
        Ok(true)
    }
}
