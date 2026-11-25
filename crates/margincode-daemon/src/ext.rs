//! Extension points for code built on the daemon, which starts through `run_with` (`serve.rs`): its `customize`
//! step gets the built `AppState`, adds the points below and returns it with its extra routes. The daemon alone uses
//! none of them (`AllowAll`, `NoCheckpointHook`, `NoUsage`, no extensions).
//!
//! Points: `router_with` (routes), `with_extension` (typed services), `subscribe_events`, `MergeGuard`,
//! `with_mcp_tools`, `UsageSink`, `CheckpointHook`, `put_profile`, the constitution's reads (`constitution`,
//! `constitution_history`) and the unguarded rule writes (`amend_constitution`, `amend_shared_constitution`,
//! `add_convention`, `remove_convention`), and `migrate_extension` then `pool` for the
//! extension's own tables. `AppState::shutdown` turns `true` when the daemon stops, for `customize`'s background tasks.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use margincode_core::routing::BacklogEntry;
use margincode_core::{AvEvent, Convention, IntentStore, Record, Task, Verdict};
use serde::Serialize;
use serde_json::Value;
use tokio::sync::broadcast;
use uuid::Uuid;

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
/// the owner, an approval at head) were already checked when it is asked, except as `Review`
/// says.
#[derive(Debug, Clone, Copy)]
pub enum Guarded<'a> {
    /// `reviewer` approves `task` at its head.
    Approve { task: &'a Task, reviewer: &'a str },
    /// `reviewer` gives `task` a `verdict` (an approval or a change request). Asked for every
    /// verdict once the task is known to be in review and the reviewer is not its owner, before an
    /// approval's own checks (tests at head, the repo's checks) and before anything is written, so
    /// a refusal leaves the task in review with no verdict and no `review` records. An approval
    /// that passes is then asked as `Approve` too, so a guard that knows only `Approve` keeps
    /// working.
    Review {
        task: &'a Task,
        reviewer: &'a str,
        verdict: Verdict,
    },
    /// `agent` merges the approved `task` into main.
    Merge { task: &'a Task, agent: &'a str },
    /// The owner submits `task` for review; `changed_paths` are the files its workspace changes
    /// against its base. Asked after the "nothing to submit" and conflict-marker checks and before
    /// anything is written, so a refusal leaves the task open and adds no records. Asked the same
    /// way, with the head's changed paths, before a reviewer's approval of a head that moved
    /// since the latest submit; a refusal then refuses that approval. Not asked for an approval
    /// `av` carries: a carry keeps the approved set of changed files.
    Submit {
        task: &'a Task,
        changed_paths: &'a [String],
    },
    /// `by` accepts the proposed backlog `item` (`POST /v1/backlog/{id}/accept`): it would become
    /// queued, and `next` may then offer it. `item.posted_by` is who proposed it. Asked once the
    /// item is known to be proposed and before anything is written, so a refusal leaves it
    /// proposed.
    AcceptBacklog { item: &'a BacklogEntry, by: &'a str },
    /// `by` would change what other tasks' agents are shown of `repo`'s records
    /// (`MemoryAction`). `dream` is the run the write comes from, when it comes from one. Asked
    /// once the daemon's own rules allow the write and before anything is written, so a refusal
    /// leaves the records and verdicts as they were.
    MemoryWrite {
        repo: &'a str,
        action: MemoryAction<'a>,
        by: &'a str,
        dream: Option<Uuid>,
    },
    /// A caller would change `repo`'s backup or move its trunk from a remote (`BackupAction`).
    /// The routes carry no actor, so a guard that wants one refuses them and offers its own. Asked
    /// after the daemon's own checks (a remote that runs a program, a bad branch) and before
    /// anything is written or any git runs, so a refusal leaves the backup and trunk as they were.
    /// Not asked for the daemon's own pushes (after a merge, an undo, a revert or an advance; a
    /// retry; `AV_BACKUP_REMOTES` at start-up) nor for the advance a target with `advance` set
    /// makes before each of those.
    Backup {
        repo: &'a str,
        action: BackupAction<'a>,
    },
}

/// What `Guarded::Backup` would do. A `remote` is shown without the user and password a URL may
/// carry.
#[derive(Debug, Clone, Copy)]
pub enum BackupAction<'a> {
    /// `POST /v1/repos/{name}/backup`: the target set, then pushed to; `remote` `None` removes
    /// the backup. `branch` `None` is the trunk's own name; with `advance`, every push first
    /// advances trunk from the remote's `main`.
    SetTarget {
        remote: Option<&'a str>,
        branch: Option<&'a str>,
        advance: bool,
    },
    /// `POST /v1/repos/{name}/backup/push`: a push now, after an advance when the target has
    /// `advance` set.
    Push,
    /// `POST /v1/repos/{name}/trunk/advance`: trunk fast-forwarded to `branch` of `remote` when
    /// that descends from it.
    Advance { remote: &'a str, branch: &'a str },
}

/// A write to a repository's shared memory, as `Guarded::MemoryWrite` names it.
#[derive(Debug, Clone, Copy)]
pub enum MemoryAction<'a> {
    /// A new record would supersede `record`, which packs then no longer show. `by` is the new
    /// record's author.
    Supersede { record: &'a Record },
    /// The one review of a proposed verdict (`verdict` is its id, `kind` what it says of
    /// `record`: `confirmed`, `disputed`, `duplicate`, `outdated`, `promote` or `conflict`; the
    /// last two are reviewed by name only, never with `dream`). With `agree` it takes effect in
    /// packs; without, it never does. `by` is the reviewer.
    VerdictReview {
        verdict: Uuid,
        kind: &'a str,
        record: &'a Record,
        agree: bool,
    },
    /// A rule agents are held to would change: an article of `repo`'s own constitution (`POST`,
    /// `PUT`, `DELETE /v1/repos/{name}/constitution/articles…`), or a required convention
    /// (`POST /v1/repos/{name}/conventions` with `required`, `DELETE` of a required one).
    /// Ordinary conventions are not asked about. `by` is the name the request gave (for a
    /// convention, empty when it gave none: `by` in the body, `?by=` on the `DELETE`). Asked after
    /// the daemon's own checks and before anything is written. The `AppState` methods that write
    /// rules for an extension do not ask.
    Rule {
        kind: RuleKind,
        change: RuleChange<'a>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind {
    /// An article of the repository's own constitution (`art:N`).
    Article,
    /// A required convention.
    RequiredConvention,
}

/// What `MemoryAction::Rule` would do.
#[derive(Debug, Clone, Copy)]
pub enum RuleChange<'a> {
    /// A new rule as it would read. For an article, `number` is the one it would get (unless
    /// another amendment comes first), `title` is set and `scope_glob` is `**`; for a required
    /// convention `number` and `title` are `None`.
    Add {
        number: Option<i32>,
        title: Option<&'a str>,
        text: &'a str,
        scope_glob: &'a str,
        check: Option<&'a str>,
    },
    /// Article `number` as it would read after the amendment (articles only).
    Amend {
        number: i32,
        title: &'a str,
        text: &'a str,
        check: Option<&'a str>,
    },
    /// Article `number` repealed, or a required convention (`number` is `None`) removed;
    /// `convention` is the row in force now.
    Remove {
        number: Option<i32>,
        convention: &'a Convention,
    },
}

/// Asked before a submit, a review verdict, an approval, a merge, the acceptance of a proposed
/// backlog item, a write to shared memory (a supersede, a verdict's review, a change to an
/// article or a required convention) and a caller's change to the backup or advance of trunk
/// (`Guarded::Backup`). `Err(reason)`
/// refuses the request with HTTP 422 and the reason; the task (or item, record or verdict) stays
/// as it was. A review verdict asks `Review` first and, for an approval, `Approve`
/// after it.
#[async_trait]
pub trait MergeGuard: Send + Sync {
    async fn check(&self, action: Guarded<'_>) -> Result<(), String>;
}

pub struct AllowAll;

#[async_trait]
impl MergeGuard for AllowAll {
    async fn check(&self, _: Guarded<'_>) -> Result<(), String> {
        Ok(())
    }
}

/// Asked after a checkpoint snapshotted and indexed a task's workspace (`POST
/// /v1/tasks/{id}/checkpoint`, a remote checkout's patch too; not the snapshots `validate`,
/// `submit` and `merge` take, whose answers are their own). What it returns is added to the
/// answer beside `head_change`, `changed` and `warnings`: an object whose keys are added where the
/// daemon's are not (the daemon's win). A `notices` field, a list of strings, is for the agent:
/// the MCP `checkpoint` tool answers with the whole object, and the Claude Code `post-edit` hook
/// prints `notices` as `additionalContext`, so an extension can warn an agent right after an
/// edit (another task editing the same lines, say). `Err(reason)` is logged and the answer is
/// as it was; the checkpoint itself is done by then and never fails on the hook.
#[async_trait]
pub trait CheckpointHook: Send + Sync {
    /// `task` as it was before the checkpoint (its `head_change` is the old head), `head` the
    /// new head, `changed` the paths the workspace changes against the task's base, sorted.
    async fn after_checkpoint(
        &self,
        task: &Task,
        head: &margincode_core::ChangeId,
        changed: &[String],
    ) -> Result<Value, String>;
}

pub struct NoCheckpointHook;

#[async_trait]
impl CheckpointHook for NoCheckpointHook {
    async fn after_checkpoint(
        &self,
        _: &Task,
        _: &margincode_core::ChangeId,
        _: &[String],
    ) -> Result<Value, String> {
        Ok(Value::Object(Default::default()))
    }
}

/// `answer` plus the hook's keys that it lacks; a hook that fails or answers a non-object adds nothing.
pub(crate) async fn with_hook(
    hook: &dyn CheckpointHook,
    task: &Task,
    head: &margincode_core::ChangeId,
    changed: &[String],
    mut answer: Value,
) -> Value {
    match hook.after_checkpoint(task, head, changed).await {
        Ok(Value::Object(extra)) => {
            if let Some(a) = answer.as_object_mut() {
                for (k, v) in extra {
                    a.entry(k).or_insert(v);
                }
            }
        }
        Ok(other) => tracing::warn!(
            task = %task.id,
            kind = other.to_string().chars().take(40).collect::<String>(),
            "checkpoint hook answered something other than an object; ignored"
        ),
        Err(e) => tracing::warn!(task = %task.id, error = %e, "checkpoint hook failed; ignored"),
    }
    answer
}

#[async_trait]
pub trait ToolHandler: Send + Sync {
    /// `agent` is the calling session's agent name, `args` the arguments, already checked against
    /// the tool's `input_schema`. `Err(text)` is a tool error (`isError`), as for Community tools.
    async fn call(&self, agent: &str, args: Value) -> Result<String, String>;
}

/// An MCP tool an extension adds (`AppState::with_mcp_tools`). It is listed by `tools/list` after
/// the Community tools. A name a Community tool, or an earlier extension tool, already has is
/// refused when the tool is registered, so `tools/call` can search either list first.
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

    pub(crate) fn definition(&self) -> Value {
        serde_json::json!({"name": self.name, "description": self.description,
                           "inputSchema": self.input_schema})
    }
}

/// What an agent reports with `POST /v1/agents/{name}/usage`: one call's usage. Every field but
/// `agent` is optional in the body (a missing count is 0).
#[derive(Debug, Clone, Serialize)]
pub struct UsageReport {
    pub agent: String,
    pub task_id: Option<String>,
    pub session: Option<String>,
    pub model: Option<String>,
    /// The provider's id of the call (`message.id`), so a sink can tell a retried report from a
    /// second call. `None` when the client has none.
    pub call_id: Option<String>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    /// Tokens written to the prompt cache (Anthropic's `cache_creation_input_tokens`).
    pub cache_write_tokens: u64,
    /// What the agent's CLI says the call cost, if it does.
    pub cost_usd: Option<f64>,
}

/// Told about every usage report. `Err(reason)` refuses it with HTTP 422 and the reason.
#[async_trait]
pub trait UsageSink: Send + Sync {
    async fn report(&self, report: UsageReport) -> Result<(), String>;
}

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
    ) -> margincode_core::Result<broadcast::Receiver<AvEvent>> {
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
