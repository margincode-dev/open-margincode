//! Scholion daemon: HTTP API over the VCS engine, intent store, code index and context packer.

mod api;
mod auth;
mod backup;
mod checks;
mod derived;
mod embed_choice;
mod error;
mod flow;
mod indexing;
mod mcp;
mod reindex;
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

pub use api::router;
pub use backup::{BACKUP_TICK, backoff, backup_tick, start_backups};
pub use embed_choice::EmbedChoice;
pub use reindex::{SEMANTIC_LOCK_KEY, worker_tick};
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
    /// `AV_TOKEN`: when set, every route but `/healthz` needs it as a bearer token.
    pub(crate) token: Option<String>,
    /// Open MCP sessions (`/mcp`), by `Mcp-Session-Id`.
    pub(crate) mcp: mcp::Sessions,
    task_locks: Arc<Mutex<HashMap<TaskId, Arc<tokio::sync::Mutex<()>>>>>,
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
            token: None,
            mcp: Arc::default(),
            task_locks: Arc::default(),
        }
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

    /// Requires `token` on every route but `/healthz`; `None` leaves the API open.
    pub fn with_token(mut self, token: Option<String>) -> Self {
        self.token = token.filter(|t| !t.trim().is_empty());
        self
    }

    /// The configured `AV_TOKEN`, if any.
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
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

    /// The Postgres pool, for the worker loop's advisory lock.
    pub fn pool(&self) -> &sqlx::PgPool {
        self.pg.pool()
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
