//! Extension points for code built on top of the daemon: extra routes (`router_with`), typed
//! services kept in `AppState` (`with_extension`), the event feed in process
//! (`subscribe_events`), a guard the lifecycle asks before an approval or a merge
//! (`MergeGuard`, which also sees a submit with the paths it changes), extra MCP tools
//! (`AppState::with_mcp_tools`), a sink for the usage agents report (`UsageSink`), a writer of
//! agent profiles (`AppState::put_profile`), and migrations of the extension's own tables in its own Postgres schema
//! (`AppState::migrate_extension`, then `AppState::pool` for its queries). The daemon alone uses
//! none of them: no extensions, and a guard that allows everything the lifecycle rules allow.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use scholion_core::{AvEvent, IntentStore, Task};
use serde::Serialize;
use serde_json::Value;
use tokio::sync::broadcast;

/// Services of an extension, one per type.
#[derive(Clone, Default)]
pub(crate) struct Extensions(Arc<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>);

impl Extensions {
    pub(crate) fn insert<T: Send + Sync + 'static>(&mut self, value: Arc<T>) {
        Arc::make_mut(&mut self.0).insert(TypeId::of::<T>(), value);
    }

    pub(crate) fn get<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.0.get(&TypeId::of::<T>())?.clone().downcast().ok()
    }
}

/// What a `MergeGuard` is asked about. The lifecycle rules (tests at head, a reviewer other than
/// the owner, an approval at head) were already checked when it is asked.
#[derive(Debug, Clone, Copy)]
pub enum Guarded<'a> {
    /// `reviewer` approves `task` at its head.
    Approve { task: &'a Task, reviewer: &'a str },
    /// `agent` merges the approved `task` into main.
    Merge { task: &'a Task, agent: &'a str },
    /// The owner submits `task` for review; `changed_paths` are the files its workspace changes
    /// against its base. Asked after the "nothing to submit" and conflict-marker checks and before
    /// anything is written, so a refusal leaves the task open and adds no records.
    Submit {
        task: &'a Task,
        changed_paths: &'a [String],
    },
}

/// Asked before a submit, an approval and a merge. `Err(reason)` refuses the request with HTTP 422
/// and the reason; the task stays as it was.
#[async_trait]
pub trait MergeGuard: Send + Sync {
    async fn check(&self, action: Guarded<'_>) -> Result<(), String>;
}

/// The daemon's own guard: the lifecycle rules are all there is.
pub struct AllowAll;

#[async_trait]
impl MergeGuard for AllowAll {
    async fn check(&self, _: Guarded<'_>) -> Result<(), String> {
        Ok(())
    }
}

/// Runs an extension's MCP tool.
#[async_trait]
pub trait ToolHandler: Send + Sync {
    /// `agent` is the calling session's agent name, `args` the arguments, already checked against
    /// the tool's `input_schema`. `Err(text)` is a tool error (`isError`), as for Community tools.
    async fn call(&self, agent: &str, args: Value) -> Result<String, String>;
}

/// An MCP tool an extension adds (`AppState::with_mcp_tools`). It is listed by `tools/list` after
/// the Community tools and dispatched by `tools/call` to its handler.
#[derive(Clone)]
pub struct ExtTool {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) input_schema: Value,
    pub(crate) handler: Arc<dyn ToolHandler>,
}

impl ExtTool {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: Value,
        handler: Arc<dyn ToolHandler>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
            handler,
        }
    }

    /// The `tools/list` entry.
    pub(crate) fn definition(&self) -> Value {
        serde_json::json!({"name": self.name, "description": self.description,
                           "inputSchema": self.input_schema})
    }
}

/// What an agent reports with `POST /v1/agents/{name}/usage`.
#[derive(Debug, Clone, Serialize)]
pub struct UsageReport {
    pub agent: String,
    pub task_id: Option<String>,
    pub session: Option<String>,
    pub model: Option<String>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    /// What the agent's CLI says the call cost, if it does.
    pub cost_usd: Option<f64>,
}

/// Told about every usage report. `Err(reason)` refuses it with HTTP 422 and the reason.
#[async_trait]
pub trait UsageSink: Send + Sync {
    async fn report(&self, report: UsageReport) -> Result<(), String>;
}

/// The daemon's own sink: reports are accepted and nothing is stored.
pub struct NoUsage;

#[async_trait]
impl UsageSink for NoUsage {
    async fn report(&self, _: UsageReport) -> Result<(), String> {
        Ok(())
    }
}

/// How many events a slow subscriber may fall behind before it misses some
/// (`RecvError::Lagged`).
const FEED_CAPACITY: usize = 1024;
/// How often the feed reads new events. Same as the `/v1/events` long poll.
const FEED_POLL: std::time::Duration = std::time::Duration::from_millis(200);
const FEED_BATCH: usize = 500;

/// The event feed in process: one reader of the events table for all subscribers, started by
/// the first subscriber and stopped when the last one is gone.
#[derive(Clone, Default)]
pub(crate) struct EventFeed(Arc<Mutex<Option<broadcast::Sender<AvEvent>>>>);

impl EventFeed {
    /// Every event committed after this call, of every repo, oldest first.
    pub(crate) async fn subscribe(
        &self,
        store: Arc<dyn IntentStore>,
    ) -> scholion_core::Result<broadcast::Receiver<AvEvent>> {
        // read before taking the lock: a feed started now begins after this id
        let since = store.last_event_id(None).await?;
        let mut slot = self.0.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(tx) = slot.as_ref() {
            return Ok(tx.subscribe());
        }
        let (tx, rx) = broadcast::channel(FEED_CAPACITY);
        *slot = Some(tx.clone());
        tokio::spawn(feed(self.0.clone(), tx, store, since));
        Ok(rx)
    }
}

async fn feed(
    slot: Arc<Mutex<Option<broadcast::Sender<AvEvent>>>>,
    tx: broadcast::Sender<AvEvent>,
    store: Arc<dyn IntentStore>,
    mut since: i64,
) {
    loop {
        {
            // checked under the lock `subscribe` takes, so no subscriber joins a stopping feed
            let mut slot = slot.lock().unwrap_or_else(|e| e.into_inner());
            if tx.receiver_count() == 0 {
                *slot = None;
                return;
            }
        }
        match store.events_since(None, since, FEED_BATCH).await {
            Ok(events) => {
                let full = events.len() == FEED_BATCH;
                for ev in events {
                    since = ev.id;
                    let _ = tx.send(ev);
                }
                if full {
                    continue; // more are waiting
                }
            }
            Err(e) => tracing::warn!(error = %e, "event feed read failed"),
        }
        tokio::time::sleep(FEED_POLL).await;
    }
}
