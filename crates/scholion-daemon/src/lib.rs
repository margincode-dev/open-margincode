//! Scholion daemon: HTTP API over the VCS engine, intent store, code index and context packer.

mod api;
mod attention;
mod auth;
mod backup;
mod build_cache;
mod checks;
mod deps_cache;
mod derived;
mod disk;
mod embed_choice;
mod embedded_pg;
mod error;
mod ext;
mod flow;
mod history;
mod indexing;
mod mcp;
mod office;
mod reindex;
mod remote;
mod routing_api;
mod serve;
mod ui;
mod wiring;
mod worker;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use scholion_core::Semantic;
use scholion_core::{IntentStore, TaskId, Vcs};
use scholion_index::CodeIndex;
use scholion_pack::Packer;
use scholion_semantic::{Embedder, SemanticSearch, VectorIndex};
use scholion_store::PgStore;

pub use api::{StartPackPause, router, router_with, sync_repos};
pub use attention::AttentionLimits;
pub use backup::{BACKUP_TICK, backoff, backup_tick, start_backups};
pub use disk::RECLAIM_TICK;
pub use embed_choice::EmbedChoice;
pub use embedded_pg::EmbeddedPg;
pub use ext::{
    AllowAll, ExtTool, Guarded, MergeGuard, NoUsage, ToolHandler, UsageReport, UsageSink,
};
pub use reindex::{SEMANTIC_LOCK_KEY, worker_tick};
pub use remote::MAX_PATCH_BYTES;
pub use serve::{BoxError, run_with, serve_with, shutdown_signal};
pub use wiring::{Config, build_state, embed_authorization};
pub use worker::{DrainStats, OutboxWorker};

#[derive(Clone)]
pub struct AppState {
    pub(crate) vcs: Arc<dyn Vcs>,
    pub(crate) store: Arc<dyn IntentStore>,
    pub(crate) index: Arc<dyn CodeIndex>,
    pub(crate) packer: Arc<Packer>,
    /// Where workspaces live inside this process (container path).
    pub(crate) ws_root: PathBuf,
    /// The same directory as the host sees it; this is what clients are told.
    pub(crate) ws_host_root: PathBuf,
    /// Concrete store: outbox, reindex listings and the advisory lock need Postgres itself.
    pub(crate) pg: Arc<PgStore>,
    pub(crate) semantic: Option<Arc<dyn Semantic>>,
    pub(crate) embedder: Option<Arc<dyn Embedder>>,
    pub(crate) vectors: Option<Arc<dyn VectorIndex>>,
    /// `AV_EXPAND_CAP_TOKENS`: per-session limit on tokens pulled through `expand`.
    pub(crate) expand_cap: Option<u64>,
    /// `AV_LEASE_SECS`: how long a claim lasts without a fenced write renewing it.
    pub(crate) lease_secs: u64,
    /// `AV_ATTN_*`: when a waiting task or a footprint needs attention (`GET /v1/attention`).
    pub(crate) attention: AttentionLimits,
    /// `AV_TOKEN`: when set, every route but `/healthz` and `/ui` needs it as a bearer token.
    pub(crate) token: Option<String>,
    /// `AV_WS_RETAIN_SECS`: how long a merged or abandoned task's workspace stays; `None` = for
    /// good, `Some(0)` = removed by the merge or abandon itself.
    pub(crate) ws_retain: Option<u64>,
    /// How often the daemon looks for workspaces to reclaim (`disk::RECLAIM_TICK`).
    pub(crate) reclaim_tick: std::time::Duration,
    /// Where the jj store lives (`AV_STORE_ROOT`), for `GET /v1/disk`; `None` when not known.
    pub(crate) store_root: Option<PathBuf>,
    /// Lite mode's own Postgres directory (`AV_PG_DIR`), for `GET /v1/disk`.
    pub(crate) pg_dir: Option<PathBuf>,
    /// `AV_BUILD_CACHE`: one cargo target per repo, shared by its task workspaces.
    pub(crate) build_cache: bool,
    /// What sessions reported adding outside any task workspace (`POST /v1/footprint`), newest
    /// first, one per session, in memory only.
    pub(crate) footprints: disk::Footprints,
    /// Open MCP sessions (`/mcp`), by `Mcp-Session-Id`.
    pub(crate) mcp: mcp::Sessions,
    task_locks: Arc<Mutex<HashMap<TaskId, Arc<tokio::sync::Mutex<()>>>>>,
    /// Asked before an approval and a merge; `AllowAll` unless an extension sets one.
    pub(crate) guard: Arc<dyn MergeGuard>,
    /// How many task starts copy an index at once; the others wait their turn. A copy holds a
    /// pooled connection for its whole length, so without a limit a burst of starts takes every
    /// connection and other requests fail with `pool timed out`.
    pub(crate) starts: Arc<tokio::sync::Semaphore>,
    /// Repo imports by name, running or finished since the daemon started.
    pub(crate) imports: api::Imports,
    /// Reviews offered by `next`, by task: who was offered it and until when (Plan 9).
    pub(crate) reviews: routing_api::Reviews,
    /// The longest `next` holds a call open waiting for work (`wait_secs` is clamped to it).
    pub(crate) max_wait: std::time::Duration,
    /// How often a waiting `next` looks again when nothing woke it (`routing_api::WAIT_TICK`).
    pub(crate) wait_tick: std::time::Duration,
    /// Bumped when what `next` may hand out changes in a way no event records: a backlog post
    /// or drop, a profile change. Waiting `next` calls watch it (and the event feed).
    pub(crate) work: Arc<tokio::sync::watch::Sender<u64>>,
    /// Tools an extension adds to `/mcp`. `with_mcp_tools` refuses a name a Community tool or an
    /// earlier extension tool already has, so a clash never reaches dispatch.
    pub(crate) mcp_tools: Arc<Vec<ext::ExtTool>>,
    /// Told about `POST /v1/agents/{name}/usage`; `NoUsage` unless an extension sets one.
    pub(crate) usage: Arc<dyn ext::UsageSink>,
    extensions: ext::Extensions,
    events: ext::EventFeed,
    /// `false` until the daemon's shutdown future completes (`serve_with`); extensions watch it
    /// through `shutdown()`.
    shutdown: Arc<tokio::sync::watch::Sender<bool>>,
}

impl AppState {
    /// No semantic layer: packs behave as in Plan 1 and `POST /v1/reindex` only rebuilds the outbox.
    pub fn new(
        vcs: Arc<dyn Vcs>,
        store: Arc<PgStore>,
        index: Arc<dyn CodeIndex>,
        ws_root: PathBuf,
        ws_host_root: PathBuf,
    ) -> Self {
        let dyn_store: Arc<dyn IntentStore> = store.clone();
        let starts = default_start_limit(store.pool().options().get_max_connections());
        let packer = Arc::new(Packer::new(vcs.clone(), dyn_store.clone(), index.clone()));
        Self {
            vcs,
            store: dyn_store,
            index,
            packer,
            ws_root,
            ws_host_root,
            pg: store,
            semantic: None,
            embedder: None,
            vectors: None,
            expand_cap: None,
            lease_secs: wiring::DEFAULT_LEASE_SECS,
            attention: AttentionLimits::default(),
            token: None,
            ws_retain: Some(wiring::DEFAULT_WS_RETAIN_SECS),
            reclaim_tick: disk::RECLAIM_TICK,
            store_root: None,
            pg_dir: None,
            build_cache: true,
            footprints: Arc::default(),
            mcp: Arc::default(),
            task_locks: Arc::default(),
            guard: Arc::new(AllowAll),
            starts: Arc::new(tokio::sync::Semaphore::new(starts)),
            imports: Arc::default(),
            reviews: Arc::default(),
            max_wait: std::time::Duration::from_secs(routing_api::MAX_WAIT_SECS),
            wait_tick: routing_api::WAIT_TICK,
            work: Arc::new(tokio::sync::watch::channel(0).0),
            mcp_tools: Arc::default(),
            usage: Arc::new(ext::NoUsage),
            extensions: ext::Extensions::default(),
            events: ext::EventFeed::default(),
            shutdown: Arc::new(tokio::sync::watch::channel(false).0),
        }
    }

    /// A handle that turns `true` when the daemon starts shutting down (its shutdown future
    /// completed; the graceful shutdown of open connections follows), so an extension's
    /// background task stops with the daemon: `shutdown.wait_for(|down| *down).await`. A state
    /// not served through `serve_with` never signals it.
    pub fn shutdown(&self) -> tokio::sync::watch::Receiver<bool> {
        self.shutdown.subscribe()
    }

    /// Tells every `shutdown()` handle that the daemon is stopping.
    pub(crate) fn signal_shutdown(&self) {
        self.shutdown.send_replace(true);
    }

    /// At most `n` task starts copy an index at once (`AV_MAX_CONCURRENT_STARTS`).
    pub fn with_start_limit(mut self, n: usize) -> Self {
        self.starts = Arc::new(tokio::sync::Semaphore::new(n.max(1)));
        self
    }

    /// Keeps an extension's service in the state, one per type; handlers added with
    /// `router_with` get it back with `extension`.
    pub fn with_extension<T: Send + Sync + 'static>(mut self, value: Arc<T>) -> Self {
        self.extensions.insert(value);
        self
    }

    /// The service of type `T` given to `with_extension`, if any.
    pub fn extension<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.extensions.get()
    }

    /// Adds `tools` to `/mcp`: listed after the Community tools, and called with the same
    /// argument checks. Panics on a name a Community tool or an earlier extension tool already
    /// has: that is a bug in the extension, found at start-up. Because registration refuses the
    /// clash, `tools/call` can search either list first.
    pub fn with_mcp_tools(mut self, tools: Vec<ext::ExtTool>) -> Self {
        let mut all = self.mcp_tools.as_ref().clone();
        for t in tools {
            assert!(
                !mcp::community_tool_names().contains(&t.name)
                    && all.iter().all(|o| o.name != t.name),
                "MCP tool name already taken: {}",
                t.name
            );
            all.push(t);
        }
        self.mcp_tools = Arc::new(all);
        self
    }

    /// Tells `sink` about every usage report (`POST /v1/agents/{name}/usage`).
    pub fn with_usage_sink(mut self, sink: Arc<dyn ext::UsageSink>) -> Self {
        self.usage = sink;
        self
    }

    /// Creates or updates an agent profile, as `PUT /v1/agents/{name}` does, so an extension's
    /// own notion of an agent can drive Community routing without a second table. With
    /// `managed_by` set the profile is the extension's: the API refuses to overwrite it (409)
    /// until a put with `managed_by: None` lets it go.
    pub async fn put_profile(
        &self,
        profile: &scholion_core::routing::Profile,
    ) -> scholion_core::Result<()> {
        self.store.put_profile(profile).await?;
        self.work_changed();
        Ok(())
    }

    /// Wakes every waiting `next`: what it may be given changed without an event saying so.
    pub(crate) fn work_changed(&self) {
        self.work.send_modify(|n| *n = n.wrapping_add(1));
    }

    /// Asks `guard` before every submit, approval and merge, after the lifecycle rules pass.
    pub fn with_merge_guard(mut self, guard: Arc<dyn MergeGuard>) -> Self {
        self.guard = guard;
        self
    }

    /// Lifecycle events (`GET /v1/events`, of every repo) committed from now on, in process.
    pub async fn subscribe_events(
        &self,
    ) -> scholion_core::Result<tokio::sync::broadcast::Receiver<scholion_core::AvEvent>> {
        self.events.subscribe(self.store.clone()).await
    }

    /// Serializes the workspace operations of one task (snapshot → re-index → store head), so a
    /// slower request can never store an older head or index older content after a newer one.
    /// Per process: the daemon is the only writer.
    pub(crate) async fn lock_task(&self, id: TaskId) -> tokio::sync::OwnedMutexGuard<()> {
        let lock = {
            let mut map = self.task_locks.lock().unwrap_or_else(|e| e.into_inner());
            map.retain(|_, l| Arc::strong_count(l) > 1); // drop idle locks
            map.entry(id).or_default().clone()
        };
        lock.lock_owned().await
    }

    /// Requires `token` on every route but `/healthz` and `/ui`; `None` leaves the API open.
    pub fn with_token(mut self, token: Option<String>) -> Self {
        self.token = token.filter(|t| !t.trim().is_empty());
        self
    }

    /// The configured `AV_TOKEN`, if any.
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// The longest `next` waits for work (default `MAX_WAIT_SECS`); shorter in tests.
    pub fn with_max_wait(mut self, max: std::time::Duration) -> Self {
        self.max_wait = max;
        self
    }

    /// How often a waiting `next` looks again when nothing woke it (default `WAIT_TICK`, 10 s):
    /// what only time releases (a lease running out, a review reservation ending) waits for it.
    pub fn with_wait_tick(mut self, tick: std::time::Duration) -> Self {
        self.wait_tick = tick;
        self
    }

    /// How long a merged or abandoned task's workspace stays on disk (`AV_WS_RETAIN_SECS`):
    /// `None` keeps it, `Some(0)` removes it with the merge or abandon.
    pub fn with_ws_retain(mut self, secs: Option<u64>) -> Self {
        self.ws_retain = secs;
        self
    }

    /// How often the daemon reclaims workspaces (default `RECLAIM_TICK`); shorter in tests.
    pub fn with_reclaim_tick(mut self, tick: std::time::Duration) -> Self {
        self.reclaim_tick = tick;
        self
    }

    /// The store root and lite mode's Postgres directory, reported by `GET /v1/disk`.
    pub fn with_disk_paths(mut self, store_root: Option<PathBuf>, pg_dir: Option<PathBuf>) -> Self {
        self.store_root = store_root;
        self.pg_dir = pg_dir;
        self
    }

    /// Whether a cargo repo's task workspaces share one build cache (`AV_BUILD_CACHE`, default on).
    pub fn with_build_cache(mut self, on: bool) -> Self {
        self.build_cache = on;
        self
    }

    /// The thresholds of `GET /v1/attention` (`AV_ATTN_*`).
    pub fn with_attention(mut self, limits: AttentionLimits) -> Self {
        self.attention = limits;
        self
    }

    pub fn with_lease_secs(mut self, secs: u64) -> Self {
        self.lease_secs = secs;
        self
    }

    pub fn with_expand_cap(mut self, cap: Option<u64>) -> Self {
        self.expand_cap = cap;
        self
    }

    /// Packs search `semantic` (and fall back to full-text when it fails). Does not start a worker.
    pub fn with_semantic(mut self, semantic: Arc<dyn Semantic>) -> Self {
        self.packer = Arc::new(Packer::with_semantic(
            self.vcs.clone(),
            self.store.clone(),
            self.index.clone(),
            Some(semantic.clone()),
        ));
        self.semantic = Some(semantic);
        self
    }

    /// Embedder + vector index: packs search them, `reindex` and `worker` write to them.
    pub fn with_index_backend(
        mut self,
        embedder: Arc<dyn Embedder>,
        vectors: Arc<dyn VectorIndex>,
    ) -> Self {
        self.embedder = Some(embedder.clone());
        self.vectors = Some(vectors.clone());
        self.with_semantic(Arc::new(SemanticSearch::new(embedder, vectors)))
    }

    /// The outbox worker for this state's backends; `None` when no backend is configured.
    pub fn worker(&self) -> Option<OutboxWorker> {
        Some(OutboxWorker::new(
            self.pg.clone(),
            self.index.clone(),
            self.embedder.clone()?,
            self.vectors.clone()?,
        ))
    }

    /// The Postgres pool: the worker loop's advisory lock, and an extension's own queries.
    pub fn pool(&self) -> &sqlx::PgPool {
        self.pg.pool()
    }

    /// Runs an extension's migrations in its own Postgres `schema` (see
    /// `PgStore::migrate_extension`); call it once at start-up, before serving. Build the
    /// migrator with `sqlx::migrate!` in the extension's crate.
    pub async fn migrate_extension(
        &self,
        schema: &str,
        migrator: &sqlx::migrate::Migrator,
    ) -> scholion_core::Result<()> {
        self.pg.migrate_extension(schema, migrator).await
    }

    /// Inverse of `to_host_path`: where a workspace path a client was told lives in this process.
    pub(crate) fn to_local_path(&self, p: &Path) -> PathBuf {
        match p.strip_prefix(&self.ws_host_root) {
            Ok(rel) => self.ws_root.join(rel),
            Err(_) => p.to_path_buf(),
        }
    }

    pub(crate) fn to_host_path(&self, p: &Path) -> PathBuf {
        match p.strip_prefix(&self.ws_root) {
            Ok(rel) => self.ws_host_root.join(rel),
            Err(_) => p.to_path_buf(),
        }
    }
}

/// A quarter of the pool copies at once, so the rest serves every other request.
pub(crate) fn default_start_limit(pool_size: u32) -> usize {
    (pool_size as usize / 4).max(1)
}
