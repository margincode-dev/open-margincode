use std::path::PathBuf;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AvError, Result};

pub const MAX_SUMMARY_CHARS: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolId(pub i64);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChangeId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConventionId(pub i64);

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for RecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! str_enum {
    ($name:ident { $($variant:ident => $s:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { $($variant),+ }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self { $(Self::$variant => $s),+ }
            }
        }

        impl FromStr for $name {
            type Err = AvError;
            fn from_str(s: &str) -> Result<Self> {
                match s {
                    $($s => Ok(Self::$variant),)+
                    other => Err(AvError::Invalid(format!(
                        "unknown {}: {other}", stringify!($name)
                    ))),
                }
            }
        }
    };
}

str_enum!(TaskStatus {
    Open => "open",
    HandedOff => "handed_off",
    InReview => "in_review",
    Conflicted => "conflicted",
    Merged => "merged",
    Abandoned => "abandoned",
});

str_enum!(RecordKind {
    Intent => "intent",
    Decision => "decision",
    Attempt => "attempt",
    Note => "note",
    Handoff => "handoff",
    Review => "review",
    Validation => "validation",
});

/// Why a record of kind `by`, of another task (`own_task` false) or of the same one, must not
/// supersede a record of kind `old`; `None` when it may. The one rule for the daemon, the store
/// and the packer (which asks with `by` a note: could any record of another task correct this).
///
/// Superseding hides a record from every pack, hook and search, so no agent may hide a review
/// finding or a validation (a failed check stays visible), and only a task itself restates its
/// `intent` or its `handoff` state, with a record of the same kind (a note in its place would
/// leave the task without the intent a pack leads with, or without the state a claim reads).
/// What corrects a decision, an attempt or a note is one of those three: packs and the pre-read
/// hook show them where the old record was, and would not show an intent, a handoff or a
/// validation there, so the claim would vanish instead of being corrected.
pub fn supersede_refusal(old: RecordKind, by: RecordKind, own_task: bool) -> Option<&'static str> {
    use RecordKind::*;
    match (old, by) {
        (Review | Validation, _) => Some("reviews and validations cannot be superseded"),
        (Intent | Handoff, _) if !own_task => {
            Some("only its own task can supersede a task's intent or handoff")
        }
        (Intent, Intent) | (Handoff, Handoff) => None,
        (Intent, _) => Some("a task restates its intent with an intent record only"),
        (Handoff, _) => Some("a task restates its handoff with a handoff record only"),
        (Decision | Attempt | Note, Decision | Attempt | Note) => None,
        (Decision | Attempt | Note, _) => {
            Some("only a decision, attempt or note corrects a decision, attempt or note")
        }
    }
}

str_enum!(Verdict {
    Approve => "approve",
    RequestChanges => "request_changes",
});

str_enum!(Tier {
    Handle => "handle",
    Summary => "summary",
    Full => "full",
});

str_enum!(SemanticKind {
    Record => "record",
    Convention => "convention",
    CodeChunk => "code_chunk",
});

str_enum!(OutboxOp {
    Upsert => "upsert",
    Delete => "delete",
    Supersede => "supersede",
});

impl PartialOrd for Tier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AgentFamily {
    Claude,
    #[default]
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Anchor {
    pub path: String,
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub repo: String,
    pub slug: String,
    pub title: String,
    pub goal: String,
    pub status: TaskStatus,
    pub owner_agent: String,
    pub base_change: ChangeId,
    pub head_change: ChangeId,
    pub workspace_path: PathBuf,
    /// Incremented by every claim; writes that carry an older generation are refused.
    pub generation: i64,
    /// While in the future, only the owner works on an open task; after it, another agent may
    /// claim it. Renewed by the owner's fenced writes.
    pub lease_expires: Option<DateTime<Utc>>,
    /// What this task's context packs contain.
    #[serde(default)]
    pub pack_mode: PackMode,
}

/// `TaskOnly` packs keep just the Task section: the av engine and workflow without the context
/// compiler (the benchmark's `av_no_packs` arm, Plan 4).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackMode {
    #[default]
    Full,
    TaskOnly,
}

impl PackMode {
    pub fn as_str(self) -> &'static str {
        match self {
            PackMode::Full => "full",
            PackMode::TaskOnly => "task_only",
        }
    }
}

/// A row of the `repos` table. `git_source` is `None` for a repo imported before the table existed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepoEntry {
    pub name: String,
    pub git_source: Option<String>,
    pub added_at: chrono::DateTime<chrono::Utc>,
}

/// `base_change` / `workspace_path` are empty until `IntentStore::set_workspace`:
/// the workspace needs the slug the store generates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTask {
    pub repo: String,
    pub title: String,
    pub goal: String,
    pub owner_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRecord {
    pub task_id: TaskId,
    pub kind: RecordKind,
    pub summary: String,
    pub body: String,
    pub change_id: Option<ChangeId>,
    pub supersedes: Option<RecordId>,
    pub author_agent: String,
    pub anchors: Vec<Anchor>,
}

impl NewRecord {
    pub fn validate(&self) -> Result<()> {
        let n = self.summary.chars().count();
        if n > MAX_SUMMARY_CHARS {
            return Err(AvError::SummaryTooLong(n));
        }
        Ok(())
    }
}

/// What the handing-off agent says about the task; stored as the JSON body of a `handoff` record.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffState {
    #[serde(default)]
    pub done: String,
    #[serde(default)]
    pub remaining: String,
    /// What is currently broken (failing tests, half-done edits).
    #[serde(default)]
    pub broken: String,
    #[serde(default)]
    pub questions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: RecordId,
    pub created_at: DateTime<Utc>,
    pub task_id: TaskId,
    pub kind: RecordKind,
    pub summary: String,
    pub body: String,
    pub change_id: Option<ChangeId>,
    pub supersedes: Option<RecordId>,
    /// Set when a newer record supersedes this one (the record is then hidden from packs).
    pub superseded_by: Option<RecordId>,
    pub author_agent: String,
    pub anchors: Vec<Anchor>,
    /// Per anchor (same order): SHA-256 of the anchored code when the record was written, if known.
    #[serde(default)]
    pub anchor_hashes: Vec<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Convention {
    pub id: ConventionId,
    pub repo: String,
    pub scope_glob: String,
    pub text: String,
    /// Hard constraint: never dropped from a pack by ranking (Plan 2b).
    #[serde(default)]
    pub required: bool,
    /// For a required convention: a command whose exit 0 shows the change follows the rule.
    /// `av validate` runs it; approval needs it to pass at the head (Plan 5).
    #[serde(default)]
    pub check_command: Option<String>,
}

/// Where a task stands with respect to trunk: the last merge (until undone) and the paths of the
/// last conflicted merge attempt.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MergeState {
    pub pre_merge: Option<ChangeId>,
    pub merge_commit: Option<ChangeId>,
    pub merged_by: Option<String>,
    pub conflicts: Vec<String>,
}

/// Result of merging a task into trunk (Plan 3 Task 5). A conflicted merge leaves trunk untouched.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum MergeOutcome {
    Merged { pre_merge: ChangeId, main: ChangeId },
    Conflicted { paths: Vec<String> },
}

/// Git backup of a repo's trunk (Plan 3 Task 6). `remote: None` = no backup configured.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BackupState {
    pub remote: Option<String>,
    /// The remote branch main is pushed to; `None` = the trunk's own name.
    #[serde(default)]
    pub branch: Option<String>,
    pub last_pushed: Option<ChangeId>,
    /// Consecutive failed pushes since the last success.
    pub failures: u32,
    pub last_error: Option<String>,
    /// When the next attempt is due; `None` = nothing pending.
    pub next_retry: Option<DateTime<Utc>>,
    /// Plan 15: before each push, advance trunk from the remote's `main` (`Vcs::advance_trunk`).
    #[serde(default)]
    pub advance: bool,
}

/// Result of advancing trunk from a remote branch (Plan 15, `Vcs::advance_trunk`). Only
/// `FastForwarded` changes anything.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum Advance {
    /// The remote branch descended from trunk; trunk now points at it.
    FastForwarded { from: ChangeId, to: ChangeId },
    /// The remote branch is trunk.
    UpToDate { head: ChangeId },
    /// The remote branch is behind trunk (an ancestor of it): nothing to take.
    Ahead { trunk: ChangeId, remote: ChangeId },
    /// Neither descends from the other; trunk is left alone.
    Diverged { trunk: ChangeId, remote: ChangeId },
}

/// Checks a backup branch name: what git accepts as a branch (see `git check-ref-format`), and
/// nothing that could be read as an option or a refspec (`:`, a leading `-` or `+`).
pub fn check_backup_branch(name: &str) -> Result<()> {
    let bad = |why: &str| Err(AvError::Invalid(format!("backup branch {name:?} {why}")));
    if name.is_empty() || name.len() > 200 {
        return bad("must be 1 to 200 characters");
    }
    if name
        .chars()
        .any(|c| c.is_control() || c.is_whitespace() || "~^:?*[\\".contains(c) || !c.is_ascii())
    {
        return bad("has a character git does not allow in a branch name");
    }
    if name.starts_with(['-', '+', '/', '.']) || name.ends_with(['/', '.']) {
        return bad("starts or ends with a character git does not allow");
    }
    if name.contains("..") || name.contains("//") || name.contains("@{") || name == "@" {
        return bad("has a sequence git does not allow");
    }
    if name
        .split('/')
        .any(|part| part.starts_with('.') || part.ends_with(".lock"))
    {
        return bad("has a part that starts with . or ends with .lock");
    }
    Ok(())
}

/// One commit of trunk's history (`Vcs::trunk_log`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    pub change: ChangeId,
    pub parents: Vec<ChangeId>,
    pub description: String,
}

/// One stored review verdict.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerdictRow {
    pub reviewer: String,
    pub verdict: Verdict,
    pub head: ChangeId,
    /// The task's base when the verdict was given (absent on rows older than Plan 5).
    pub base: Option<ChangeId>,
    /// For an approval carried by av: the human-approved head it was carried from.
    pub carried_from: Option<ChangeId>,
    /// When the verdict was given.
    #[serde(default)]
    pub at: Option<DateTime<Utc>>,
}

/// Per-repo policy (Plan 5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoSettings {
    /// Files that may differ after a conflict resolution without a new review.
    pub carry_globs: Vec<String>,
    /// The command `av runner` runs on every new main.
    pub check_command: Option<String>,
    /// What becomes of a backlog item an agent posts: queued (default) or proposed.
    #[serde(default)]
    pub agent_posts: crate::routing::AgentPosts,
}

impl Default for RepoSettings {
    fn default() -> Self {
        RepoSettings {
            carry_globs: vec!["**/*.md".into()],
            check_command: None,
            agent_posts: crate::routing::AgentPosts::Queue,
        }
    }
}

/// One post-merge check result on a commit of main (Plan 5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckRow {
    pub change: String,
    pub command: String,
    pub exit_code: i32,
    pub output_tail: String,
    pub runner: String,
    pub at: DateTime<Utc>,
}

/// One entry of the event feed: a task lifecycle change, in commit order (`id` only grows).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvEvent {
    pub id: i64,
    pub repo: String,
    /// `started`, `checkpoint`, `submit`, `approve`, `request_changes`, `merged`, `conflicted`,
    /// `resolve`, `handoff`, `claim`, `undo` or `abandon`. A `checkpoint` is a snapshot that
    /// moved the task's head (its payload: `from`, the old head, and `touched`, how many paths
    /// the task changes against its base).
    pub kind: String,
    pub task_id: Option<TaskId>,
    /// The merge commit for `merged`, else the task's head when the event happened.
    pub change: Option<String>,
    pub payload: serde_json::Value,
    pub at: DateTime<Utc>,
}

/// Result of rebasing a task workspace onto the current trunk. Conflicting files are left in the
/// workspace with conflict markers; `conflicts` names them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RebaseOutcome {
    pub head: ChangeId,
    pub base: ChangeId,
    pub conflicts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub path: PathBuf,
    pub base_change: ChangeId,
}

/// One search hit. JSON keys match the Qdrant payload, plus `kind` and `score`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticHit {
    pub kind: SemanticKind,
    pub score: f32,
    pub repo: String,
    pub task_id: Option<TaskId>,
    pub record_id: Option<RecordId>,
    pub symbol_id: Option<SymbolId>,
    pub convention_id: Option<ConventionId>,
    pub path: Option<String>,
    pub change_id: Option<ChangeId>,
    pub superseded: bool,
}

/// One item of a context receipt: what was sent, at which tier, and why.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiptItem {
    pub id: String,
    pub tier: Tier,
    pub score: f32,
    /// `seed`, `hop1`, `hop2`, `anchored`, `semantic`, `fts`, `task`, `map`, `convention`, `live`, …
    pub reason: String,
    /// Source path and 1-based inclusive line range, for code items.
    pub source: Option<(String, u32, u32)>,
    pub tokens: u32,
}

/// The manifest of one context pack (Plan 2b): stored so a result can be traced to its inputs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewReceipt {
    pub task_id: TaskId,
    pub session: String,
    pub head_change: ChangeId,
    pub packer_version: u32,
    pub budget: u32,
    pub tokens: u32,
    pub items: Vec<ReceiptItem>,
    pub omitted: Vec<String>,
}

/// A row to insert into the semantic outbox, in the same transaction as the entity.
#[derive(Debug, Clone, PartialEq)]
pub struct NewOutbox {
    pub kind: SemanticKind,
    pub op: OutboxOp,
    pub entity_id: String,
    pub repo: String,
    pub task_id: Option<TaskId>,
    pub record_id: Option<RecordId>,
    pub symbol_id: Option<SymbolId>,
    pub convention_id: Option<ConventionId>,
    pub path: Option<String>,
    pub change_id: Option<ChangeId>,
    pub superseded: bool,
    pub text: Option<String>,
}

/// A claimed outbox row, including retry state.
#[derive(Debug, Clone, PartialEq)]
pub struct OutboxRow {
    pub id: i64,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub row: NewOutbox,
}

/// One entry of an agent's timeline (`scholion_store::PgStore::agent_timeline`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineItem {
    pub at: DateTime<Utc>,
    /// A record kind (`intent`, `decision`, `attempt`, `note`, `handoff`, `review`,
    /// `validation`), a review verdict (`approve`, `request_changes`), or `started`, `claim`,
    /// `submit` or `merged`.
    pub kind: String,
    pub task_id: Uuid,
    pub repo: String,
    pub slug: String,
    pub title: String,
    /// A record's summary; empty for the others.
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_record(summary: String) -> NewRecord {
        NewRecord {
            task_id: TaskId(uuid::Uuid::nil()),
            kind: RecordKind::Note,
            summary,
            body: String::new(),
            change_id: None,
            supersedes: None,
            author_agent: "test".into(),
            anchors: vec![],
        }
    }

    #[test]
    fn summary_over_200_chars_rejected() {
        assert!(new_record("x".repeat(200)).validate().is_ok());
        assert!(matches!(
            new_record("x".repeat(201)).validate(),
            Err(crate::AvError::SummaryTooLong(201))
        ));
    }

    #[test]
    fn summary_length_counts_chars_not_bytes() {
        assert!(new_record("é".repeat(200)).validate().is_ok());
    }

    #[test]
    fn task_status_roundtrips_as_snake_case() {
        let json = serde_json::to_string(&TaskStatus::HandedOff).unwrap();
        assert_eq!(json, "\"handed_off\"");
        assert_eq!(
            serde_json::from_str::<TaskStatus>(&json).unwrap(),
            TaskStatus::HandedOff
        );
        assert_eq!(
            "in_review".parse::<TaskStatus>().unwrap(),
            TaskStatus::InReview
        );
        assert_eq!(TaskStatus::Conflicted.as_str(), "conflicted");
    }

    #[test]
    fn record_kind_str_roundtrip() {
        for k in [
            RecordKind::Intent,
            RecordKind::Decision,
            RecordKind::Attempt,
            RecordKind::Note,
            RecordKind::Handoff,
            RecordKind::Review,
            RecordKind::Validation,
        ] {
            assert_eq!(k.as_str().parse::<RecordKind>().unwrap(), k);
        }
        assert!("bogus".parse::<RecordKind>().is_err());
    }

    #[test]
    fn tiers_are_ordered_handle_summary_full() {
        assert!(Tier::Handle < Tier::Summary && Tier::Summary < Tier::Full);
    }

    #[test]
    fn semantic_hit_json_has_payload_keys() {
        let hit = SemanticHit {
            kind: SemanticKind::CodeChunk,
            score: 0.5,
            repo: "demo".into(),
            task_id: None,
            record_id: None,
            symbol_id: Some(SymbolId(7)),
            convention_id: None,
            path: None,
            change_id: None,
            superseded: false,
        };
        let value = serde_json::to_value(&hit).unwrap();
        let obj = value.as_object().expect("object");
        let mut keys: Vec<_> = obj.keys().cloned().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                "change_id",
                "convention_id",
                "kind",
                "path",
                "record_id",
                "repo",
                "score",
                "superseded",
                "symbol_id",
                "task_id",
            ]
        );
        assert_eq!(obj["kind"], "code_chunk");
        assert_eq!(obj["symbol_id"], 7);
        assert!(obj["task_id"].is_null());
        assert_eq!(obj["superseded"], false);
    }

    #[test]
    fn what_may_be_superseded() {
        use RecordKind::*;
        let corrects = [Decision, Attempt, Note];
        let all = [Intent, Decision, Attempt, Note, Handoff, Review, Validation];
        for old in corrects {
            for by in corrects {
                assert_eq!(supersede_refusal(old, by, false), None, "{old:?} by {by:?}");
                assert_eq!(supersede_refusal(old, by, true), None, "{old:?} by {by:?}");
            }
            // a correction that packs and the pre-read hook would not show in the old one's place
            for by in [Intent, Handoff, Review, Validation] {
                for own in [false, true] {
                    let why = supersede_refusal(old, by, own);
                    assert!(
                        why.is_some_and(|w| w.contains("decision, attempt or note")),
                        "{old:?} by {by:?}: {why:?}"
                    );
                }
            }
        }
        for old in [Review, Validation] {
            for by in all {
                assert!(supersede_refusal(old, by, false).is_some(), "{old:?}");
                assert!(supersede_refusal(old, by, true).is_some(), "{old:?}");
            }
        }
        // a task restates its own intent or handoff, with a record of the same kind only
        for old in [Intent, Handoff] {
            for by in all {
                assert!(supersede_refusal(old, by, false).is_some(), "{old:?}");
                let own = supersede_refusal(old, by, true);
                if by == old {
                    assert_eq!(own, None, "{old:?} by {by:?}");
                } else {
                    assert!(
                        own.is_some_and(|w| w.contains(old.as_str())),
                        "{old:?} by {by:?}: {own:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn outbox_op_and_kind_match_sql_checks() {
        assert_eq!(OutboxOp::Supersede.as_str(), "supersede");
        assert_eq!(SemanticKind::CodeChunk.as_str(), "code_chunk");
        assert_eq!(
            "supersede".parse::<OutboxOp>().unwrap(),
            OutboxOp::Supersede
        );
        assert_eq!(
            OutboxOp::Supersede.as_str().parse::<OutboxOp>().unwrap(),
            OutboxOp::Supersede
        );
        assert_eq!(
            "code_chunk".parse::<SemanticKind>().unwrap(),
            SemanticKind::CodeChunk
        );
        assert_eq!(
            SemanticKind::CodeChunk
                .as_str()
                .parse::<SemanticKind>()
                .unwrap(),
            SemanticKind::CodeChunk
        );
        assert!("drop".parse::<OutboxOp>().is_err());
        assert!("drop".parse::<SemanticKind>().is_err());
    }
}
