use std::collections::HashMap;
use std::path::Path;

use async_trait::async_trait;

use crate::error::{AvError, Result};
use crate::types::*;

/// Version-control engine. Only `scholion-vcs` implements this against jj-lib.
#[async_trait]
pub trait Vcs: Send + Sync {
    /// Import an existing git repo as a jj repo; returns the `main@` change.
    async fn init_repo(&self, repo: &str, git_source: &Path) -> Result<ChangeId>;
    async fn create_workspace(&self, repo: &str, slug: &str) -> Result<WorkspaceInfo>;
    async fn snapshot(&self, repo: &str, slug: &str) -> Result<ChangeId>;
    async fn changed_paths(
        &self,
        repo: &str,
        base: &ChangeId,
        head: &ChangeId,
    ) -> Result<Vec<String>>;
    /// `.gitignore`-filtered file list at `change`.
    async fn list_files(&self, repo: &str, change: &ChangeId) -> Result<Vec<String>>;
    async fn read_file(&self, repo: &str, change: &ChangeId, path: &str) -> Result<Vec<u8>>;
    /// `read_file` for many paths of one change, in input order, each with its own result (a
    /// missing path is `NotFound` there). The outer error is for the repo or change itself.
    async fn read_files(
        &self,
        repo: &str,
        change: &ChangeId,
        paths: &[String],
    ) -> Result<Vec<(String, Result<Vec<u8>>)>> {
        let mut out = Vec::with_capacity(paths.len());
        for p in paths {
            out.push((p.clone(), self.read_file(repo, change, p).await));
        }
        Ok(out)
    }

    /// The names of the imported repos, sorted.
    async fn list_repos(&self) -> Result<Vec<String>> {
        Err(AvError::Vcs(
            "list_repos is not supported by this Vcs".into(),
        ))
    }

    /// The commit the trunk bookmark points at.
    async fn trunk(&self, _repo: &str) -> Result<ChangeId> {
        Err(AvError::Vcs("trunk is not supported by this Vcs".into()))
    }
    /// Snapshots the workspace, refuses (`IllegalState`) unless it equals `expected_head`, then
    /// three-way merges the task's change onto trunk as one commit and advances trunk (and the
    /// exported git ref). Any conflict leaves trunk untouched and returns `Conflicted`.
    async fn merge_task(
        &self,
        _repo: &str,
        _slug: &str,
        _expected_head: &ChangeId,
        _message: &str,
    ) -> Result<MergeOutcome> {
        Err(AvError::Vcs("merge is not supported by this Vcs".into()))
    }
    /// Pushes the trunk ref (as exported to the repo's git dir) to `remote_url`, never forcing;
    /// returns the pushed commit. `branch` names the remote branch it lands on (`None`: the
    /// trunk's own name). A rejected (non-fast-forward) or failed push is an error.
    async fn push_backup(
        &self,
        _repo: &str,
        _remote_url: &str,
        _branch: Option<&str>,
    ) -> Result<ChangeId> {
        Err(AvError::Vcs(
            "backup push is not supported by this Vcs".into(),
        ))
    }
    /// Plan 15: fetches `branch` of `remote_url` into the repo's git dir (a private ref, never a
    /// local branch) and, when the fetched commit descends from trunk, moves trunk to it (and the
    /// exported git ref). Equal, behind or diverged changes nothing; see `Advance`.
    async fn advance_trunk(
        &self,
        _repo: &str,
        _remote_url: &str,
        _branch: &str,
    ) -> Result<Advance> {
        Err(AvError::Vcs(
            "advance_trunk is not supported by this Vcs".into(),
        ))
    }
    /// Trunk and its first-parent ancestors, newest first, at most `limit`.
    async fn trunk_log(&self, _repo: &str, _limit: usize) -> Result<Vec<LogEntry>> {
        Err(AvError::Vcs(
            "trunk_log is not supported by this Vcs".into(),
        ))
    }
    /// Every ancestor of trunk (merged branches too, not only first parents), newest commit time
    /// first, at most `limit`. What the import's cold start mines; the default is `trunk_log`.
    async fn history(&self, repo: &str, limit: usize) -> Result<Vec<LogEntry>> {
        self.trunk_log(repo, limit).await
    }
    /// The trunk commits (newest first, at most `limit`) whose first parent had different content
    /// at `path`, including the commits that created or deleted it. Walks at most `scan`
    /// commits of `trunk_log`.
    async fn path_log(
        &self,
        repo: &str,
        path: &str,
        limit: usize,
        scan: usize,
    ) -> Result<Vec<LogEntry>> {
        let mut out = Vec::new();
        for entry in self.trunk_log(repo, scan).await? {
            if out.len() >= limit {
                break;
            }
            let now = file_or_none(self, repo, Some(&entry.change), path).await?;
            let before = file_or_none(self, repo, entry.parents.first(), path).await?;
            if now != before {
                out.push(entry);
            }
        }
        Ok(out)
    }
    /// Rebases the task's working-copy commit onto the current trunk and updates the files on disk;
    /// conflicts stay in the workspace as markers.
    async fn rebase_workspace(&self, _repo: &str, _slug: &str) -> Result<RebaseOutcome> {
        Err(AvError::Vcs("rebase is not supported by this Vcs".into()))
    }
    /// Writes a new commit on trunk that inverts `merge_commit` (a single-parent commit of trunk's
    /// history): trunk + (its parent − it). A conflict writes nothing and returns `Conflicted`.
    async fn revert_merge(
        &self,
        _repo: &str,
        _merge_commit: &ChangeId,
        _message: &str,
    ) -> Result<MergeOutcome> {
        Err(AvError::Vcs("revert is not supported by this Vcs".into()))
    }
    /// Moves trunk back from `merge_commit` to `pre_merge` as a new forward operation. Refused
    /// (`IllegalState`) once trunk has moved past `merge_commit`.
    async fn undo_merge(
        &self,
        _repo: &str,
        _pre_merge: &ChangeId,
        _merge_commit: &ChangeId,
    ) -> Result<()> {
        Err(AvError::Vcs("undo is not supported by this Vcs".into()))
    }
    /// Forgets the task workspace `slug` in the repo and deletes its directory. `Ok(false)` when
    /// neither existed. The repo's own `main` workspace is refused (`Invalid`). The workspace's
    /// commits stay in the store, so `restore_workspace` can bring it back.
    async fn remove_workspace(&self, _repo: &str, _slug: &str) -> Result<bool> {
        Err(AvError::Vcs(
            "remove_workspace is not supported by this Vcs".into(),
        ))
    }
    /// Re-creates the removed workspace `slug` with `head` as its working-copy commit, so its
    /// files are `head`'s and a snapshot without edits returns `head`. `Invalid` when the
    /// directory exists.
    async fn restore_workspace(
        &self,
        _repo: &str,
        _slug: &str,
        _head: &ChangeId,
    ) -> Result<WorkspaceInfo> {
        Err(AvError::Vcs(
            "restore_workspace is not supported by this Vcs".into(),
        ))
    }
    /// Plan 14, remote checkpoints: applies a git-format patch (`git diff --binary --full-index
    /// --no-renames`) to the files of task workspace `slug`, whole or not at all, and returns the
    /// paths it wrote or deleted, in patch order. It only writes files: no snapshot, no command.
    /// A malformed or unsafe patch, or one made against other content, is `Invalid`.
    async fn apply_patch(&self, _repo: &str, _slug: &str, _patch: &[u8]) -> Result<Vec<String>> {
        Err(AvError::Vcs(
            "apply_patch is not supported by this Vcs".into(),
        ))
    }
    /// Plan 14: a git-format patch from `from` (`None`: the empty tree) to `to`, which `git apply`
    /// accepts; conflicted files carry their markers, as in a workspace. Empty when equal.
    async fn diff_patch(
        &self,
        _repo: &str,
        _from: Option<&ChangeId>,
        _to: &ChangeId,
    ) -> Result<Vec<u8>> {
        Err(AvError::Vcs(
            "diff_patch is not supported by this Vcs".into(),
        ))
    }
    /// Plan 14: the git tree id of `change`'s files (what `git write-tree` gives for the same
    /// files). A conflicted tree has none: its jj tree ids joined by `+`, never 40 hex digits.
    async fn tree_id(&self, _repo: &str, _change: &ChangeId) -> Result<String> {
        Err(AvError::Vcs("tree_id is not supported by this Vcs".into()))
    }
}

/// `path` at `change`, or `None` when the change is absent or has no such file.
async fn file_or_none<V: Vcs + ?Sized>(
    vcs: &V,
    repo: &str,
    change: Option<&ChangeId>,
    path: &str,
) -> Result<Option<Vec<u8>>> {
    let Some(change) = change else {
        return Ok(None);
    };
    match vcs.read_file(repo, change, path).await {
        Ok(bytes) => Ok(Some(bytes)),
        Err(AvError::NotFound(_)) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Tasks, records, anchors, conventions and the per-session ledger.
#[async_trait]
pub trait IntentStore: Send + Sync {
    /// Appends a review verdict bound to `head`.
    async fn add_verdict(
        &self,
        task: TaskId,
        reviewer: &str,
        verdict: Verdict,
        head: &ChangeId,
        base: Option<&ChangeId>,
        carried_from: Option<&ChangeId>,
    ) -> Result<()>;
    /// The task's newest verdict at any head.
    async fn latest_verdict_any(&self, task: TaskId) -> Result<Option<VerdictRow>>;
    /// The newest verdict at `head` given in the task's current review round (after its latest
    /// `submit` event), with who gave it and when. A verdict from before a revert or a request for
    /// changes and a resubmit of the same head is not in it.
    async fn round_verdict_at(&self, task: TaskId, head: &ChangeId) -> Result<Option<VerdictRow>>;
    /// The newest verdict by `reviewer`-not-`av` at `head` (the human approval a carry names).
    async fn human_verdict_at(&self, task: TaskId, head: &ChangeId) -> Result<Option<VerdictRow>>;
    /// Stores a check result for `change` (replacing an earlier run of the same command).
    async fn add_check(&self, repo: &str, check: &CheckRow) -> Result<()>;
    /// The check results stored for any of `changes`.
    async fn checks_for(&self, repo: &str, changes: &[String]) -> Result<Vec<CheckRow>>;
    /// A cached derived value (`kind` at `change` for `key`).
    async fn derived_get(
        &self,
        repo: &str,
        kind: &str,
        change: &ChangeId,
        key: &str,
    ) -> Result<Option<serde_json::Value>>;
    async fn derived_put(
        &self,
        repo: &str,
        kind: &str,
        change: &ChangeId,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<()>;
    /// The repo's policy (defaults when none is stored).
    async fn repo_settings(&self, repo: &str) -> Result<RepoSettings>;
    async fn set_repo_settings(&self, repo: &str, settings: &RepoSettings) -> Result<()>;
    /// The newest verdict at `head`, with its reviewer.
    async fn latest_verdict(
        &self,
        task: TaskId,
        head: &ChangeId,
    ) -> Result<Option<(String, Verdict)>>;
    async fn merge_state(&self, task: TaskId) -> Result<MergeState>;
    /// Overwrites the merge columns; `merged_at` follows `merge_commit`.
    async fn set_merge_state(&self, task: TaskId, state: &MergeState) -> Result<()>;
    /// Generates a unique slug from the title (`fix-login`, `fix-login-2`, ...).
    async fn create_task(&self, t: NewTask) -> Result<Task>;
    async fn get_task(&self, id: TaskId) -> Result<Task>;
    /// Every task with this slug, in any repo and any status, by repo.
    async fn tasks_with_slug(&self, slug: &str) -> Result<Vec<Task>>;
    /// The task of `repo` with this slug.
    async fn task_by_slug(&self, repo: &str, slug: &str) -> Result<Task>;
    /// Also sets head = base.
    async fn set_status(&self, id: TaskId, status: TaskStatus) -> Result<()>;
    /// Applies a lifecycle event atomically: the update only happens if the status is still the one
    /// the transition was computed from, so of two racing callers exactly one wins (the other gets
    /// `IllegalState`). Returns the new status.
    async fn transition(&self, id: TaskId, event: crate::lifecycle::Event) -> Result<TaskStatus>;
    /// `transition`, plus `r` written in the same transaction: both happen or neither does.
    async fn transition_with_record(
        &self,
        id: TaskId,
        event: crate::lifecycle::Event,
        r: NewRecord,
    ) -> Result<(TaskStatus, Record)>;
    /// Makes `agent` the owner of a handed-off task, or of an open task whose lease has expired:
    /// status `open`, generation + 1, lease `lease_secs` from now. Of racing claimants exactly one
    /// wins. Refused (`IllegalState`) for the current owner ("already yours"), for an open task
    /// whose lease still runs, and in every other status.
    async fn claim_task(&self, id: TaskId, agent: &str, lease_secs: u64) -> Result<Task>;
    /// Extends the lease to `lease_secs` from now if `generation` is still the task's; returns
    /// whether it was.
    async fn renew_lease(&self, id: TaskId, generation: i64, lease_secs: u64) -> Result<bool>;
    /// Records an imported repo. A second call never replaces a known `git_source`; it only
    /// fills in a missing one. A blank name is `Invalid`.
    async fn register_repo(&self, name: &str, git_source: Option<&str>) -> Result<()>;
    /// Every recorded repo, by name.
    async fn list_repos(&self) -> Result<Vec<crate::types::RepoEntry>>;

    // --- Plan 8: routing. Profiles, the backlog and the tags a handoff carries.
    /// Inserts or replaces the profile of `p.name`. Tags are stored lowercase; blank ones and a
    /// blank name are `Invalid`.
    async fn put_profile(&self, p: &crate::routing::Profile) -> Result<()>;
    /// Cheapest first, then by name.
    async fn list_profiles(&self) -> Result<Vec<crate::routing::Profile>>;
    /// Removes the profile `name` in one statement. `NotFound` when there is none. Unless `force`,
    /// `Conflict` while its agent owns a task in `open`, `handed_off`, `in_review` or `conflicted`
    /// (a conflicted task goes back to its owner); the check and the delete cannot be split by a
    /// task started in between. The agent's tasks stay.
    async fn delete_profile(&self, name: &str, force: bool) -> Result<()>;
    /// How many tasks `owner` holds open (in every repo). Tasks in review wait on other agents
    /// and do not count.
    async fn count_open_tasks(&self, owner: &str) -> Result<i64>;
    async fn post_backlog(
        &self,
        item: crate::routing::NewBacklogItem,
    ) -> Result<crate::routing::BacklogEntry>;
    /// Oldest first.
    async fn list_backlog(
        &self,
        repo: Option<&str>,
        state: Option<crate::routing::BacklogState>,
    ) -> Result<Vec<crate::routing::BacklogEntry>>;
    /// One backlog row. `NotFound` for an unknown id.
    async fn get_backlog(&self, id: uuid::Uuid) -> Result<crate::routing::BacklogEntry>;
    /// Withdraws a queued or proposed item. `NotFound` for an unknown id, `IllegalState` unless
    /// it is queued or proposed.
    async fn drop_backlog(&self, id: uuid::Uuid) -> Result<()>;
    /// Queues a proposed item, accepted by `by`. `NotFound` for an unknown id, `IllegalState`
    /// unless it is proposed (of racing callers exactly one succeeds).
    async fn accept_backlog(
        &self,
        id: uuid::Uuid,
        by: &str,
    ) -> Result<crate::routing::BacklogEntry>;
    /// Marks a queued item taken by `agent`; of racing callers exactly one gets `true`.
    async fn take_backlog(&self, id: uuid::Uuid, agent: &str) -> Result<bool>;
    /// Puts a taken item back in the queue, if no task was made for it yet.
    async fn untake_backlog(&self, id: uuid::Uuid) -> Result<()>;
    /// Names the task made for a taken item.
    async fn link_backlog_task(&self, id: uuid::Uuid, task: TaskId) -> Result<()>;
    /// Counts a queued or taken item toward a target of its repo (`None` unlinks it). The target
    /// is an id or the slug of its title and must be active. `NotFound` for an unknown item or
    /// target, `IllegalState` for a dropped item or a closed target.
    async fn set_backlog_target(
        &self,
        id: uuid::Uuid,
        target: Option<&str>,
    ) -> Result<crate::routing::BacklogEntry>;

    // --- Plan 10 A3: targets.
    /// A blank repo or title, or a title over 200 characters, is `Invalid`.
    async fn add_target(&self, t: crate::routing::NewTarget) -> Result<crate::routing::Target>;
    /// Oldest first, with each target's progress.
    async fn list_targets(
        &self,
        repo: Option<&str>,
        status: Option<crate::routing::TargetStatus>,
    ) -> Result<Vec<crate::routing::Target>>;
    /// `NotFound` for an unknown id.
    async fn set_target_status(
        &self,
        id: uuid::Uuid,
        status: crate::routing::TargetStatus,
    ) -> Result<crate::routing::Target>;
    /// The target `reference` names: its id, or the slug of its title. Without `repo` a slug must
    /// be unique across repos (`Invalid` otherwise); with it, only that repo's targets count.
    /// `NotFound` when nothing matches.
    async fn resolve_target(&self, repo: Option<&str>, reference: &str) -> Result<uuid::Uuid>;
    /// Tags the next agent of a handed-off task must have. `IllegalState` unless it is handed off.
    async fn set_handoff_needs(
        &self,
        id: TaskId,
        needs: &[String],
        prefer: Option<&str>,
    ) -> Result<()>;
    /// Every handed-off task (of `repo`, or all), with the tags and preferred agent it may carry,
    /// oldest handoff first. `since` is the handoff time (the epoch for a handoff made before it
    /// was recorded). A claim clears the tags.
    async fn handed_off_tasks(&self, repo: Option<&str>) -> Result<Vec<crate::routing::HandedOff>>;
    /// Open tasks whose owner went quiet (of `repo`, or all): the lease expired, or there never
    /// was one and the task is older than `lease_secs`. Oldest first.
    async fn abandoned_tasks(
        &self,
        repo: Option<&str>,
        lease_secs: u64,
    ) -> Result<Vec<crate::routing::Stale>>;
    /// Every conflicted task (of `repo`, or all): approved, but its merge conflicted, so the owner
    /// must sync, resolve and resubmit. `since` is when its owner's lease ends (as for
    /// `abandoned_tasks`); the owner is offered it at once, any other agent once `since` has passed.
    /// Oldest first.
    async fn conflicted_tasks(
        &self,
        repo: Option<&str>,
        lease_secs: u64,
    ) -> Result<Vec<crate::routing::Stale>>;
    /// Merged and abandoned tasks that entered that status at least `older_than_secs` ago, oldest
    /// first: their workspaces may be reclaimed (Plan 11). Empty unless the store keeps events.
    async fn reclaimable_tasks(&self, _older_than_secs: u64) -> Result<Vec<Task>> {
        Ok(vec![])
    }
    /// Every task (of `repo`, or all) with when it entered its current status: its newest event
    /// into that status, or its creation when there is none. Empty unless the store keeps events.
    async fn tasks_with_status_since(
        &self,
        _repo: Option<&str>,
    ) -> Result<Vec<(Task, chrono::DateTime<chrono::Utc>)>> {
        Ok(vec![])
    }
    /// Appends a feed event (`GET /v1/events`) that is not a status change, such as
    /// `workspace:reclaimed`. Does nothing unless the store keeps events.
    async fn add_event(
        &self,
        _repo: &str,
        _kind: &str,
        _task: Option<TaskId>,
        _payload: serde_json::Value,
    ) -> Result<()> {
        Ok(())
    }
    async fn set_workspace(&self, id: TaskId, path: &Path, base: &ChangeId) -> Result<()>;
    async fn set_pack_mode(&self, id: TaskId, mode: PackMode) -> Result<()>;
    async fn set_head(&self, id: TaskId, head: &ChangeId, touched: &[String]) -> Result<()>;
    /// Paths recorded by the last `set_head`, sorted.
    async fn touched_paths(&self, id: TaskId) -> Result<Vec<String>>;
    /// Other tasks of `repo` that are still in flight (open, handed off, in review or conflicted;
    /// not merged or abandoned) and whose last `set_head` touched any of `paths`.
    async fn live_tasks_touching(
        &self,
        repo: &str,
        paths: &[String],
        exclude: TaskId,
    ) -> Result<Vec<Task>>;
    /// Rejects summaries over 200 chars.
    async fn add_record(&self, r: NewRecord) -> Result<Record> {
        self.add_record_with(r, &[], None).await
    }
    /// `add_record`, plus the per-anchor content hashes (same order as `r.anchors`, see
    /// `set_anchor_hashes`) and the receipt the author cites (any accepted id form), all in the
    /// record's own transaction: on any error nothing is written.
    async fn add_record_with(
        &self,
        r: NewRecord,
        anchor_hashes: &[Option<String>],
        receipt: Option<&str>,
    ) -> Result<Record>;
    async fn task_records(&self, id: TaskId) -> Result<Vec<Record>>;
    /// Excludes superseded records.
    async fn records_anchored_to(
        &self,
        repo: &str,
        paths: &[String],
        symbols: &[String],
    ) -> Result<Vec<Record>>;
    async fn get_record(&self, id: RecordId) -> Result<Record>;
    /// Resolves a full UUID or a unique hex prefix (>= 6 chars), as shown in context packs.
    async fn find_record(&self, id_or_prefix: &str) -> Result<Record>;
    async fn add_convention(
        &self,
        repo: &str,
        scope_glob: &str,
        text: &str,
    ) -> Result<ConventionId>;
    async fn set_convention_required(&self, id: ConventionId, required: bool) -> Result<()>;
    /// Events after `since` (of one repo, or all), oldest first, at most `limit`.
    async fn events_since(
        &self,
        repo: Option<&str>,
        since: i64,
        limit: usize,
    ) -> Result<Vec<AvEvent>>;
    /// The newest event id (of one repo, or all); 0 when there is none.
    async fn last_event_id(&self, repo: Option<&str>) -> Result<i64>;
    /// Sets (or with `None` clears) a convention's check command.
    async fn set_convention_check(&self, id: ConventionId, command: Option<&str>) -> Result<()>;
    /// Hides a convention from packs, lists and checks (its row stays).
    async fn remove_convention(&self, repo: &str, id: ConventionId) -> Result<()>;
    /// Every convention of the repo, oldest first.
    async fn conventions(&self, repo: &str) -> Result<Vec<Convention>>;
    async fn conventions_for(&self, repo: &str, paths: &[String]) -> Result<Vec<Convention>>;
    /// Stores a pack manifest; returns its id as shown to agents (`rcp:<8 hex>`).
    async fn add_receipt(&self, r: NewReceipt) -> Result<String>;
    /// A receipt by full id or a unique hex prefix (>= 6 chars), with or without `rcp:`.
    async fn get_receipt(&self, id_or_prefix: &str) -> Result<serde_json::Value>;
    /// Stores, per anchor of `record` (same order as `Record::anchors`), the hash of the anchored
    /// code at the moment the record was written.
    async fn set_anchor_hashes(&self, record: RecordId, hashes: &[Option<String>]) -> Result<()>;
    /// Links a record to the receipt (any accepted id form) of the pack its author worked from.
    async fn cite_receipt(&self, record: RecordId, receipt: &str) -> Result<()>;
    /// Adds `tokens` to the session's expand total and returns the new total, or `None` (charging
    /// nothing) when that would take the total over `cap`. Check and charge are one atomic step.
    async fn charge_expand(
        &self,
        session: &str,
        tokens: u64,
        cap: Option<u64>,
    ) -> Result<Option<u64>>;
    async fn expand_tokens(&self, session: &str) -> Result<u64>;
    async fn ledger_get(&self, session: &str) -> Result<HashMap<String, Tier>>;
    async fn ledger_put(&self, session: &str, sent: &[(String, Tier)]) -> Result<()>;
    /// Full-text over `records.tsv`. Whitespace-only `query` returns an empty vec.
    /// Superseded records and other repos are omitted.
    async fn search_records_fts(
        &self,
        repo: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Record>>;
}

/// Derived similarity index. `scholion-pack` calls this and never talks to Qdrant or an embedder.
#[async_trait]
pub trait Semantic: Send + Sync {
    /// `limit` is per collection. Whitespace-only `query` returns `Ok(vec![])` and does not embed.
    async fn search(&self, repo: &str, query: &str, limit: usize) -> Result<Vec<SemanticHit>>;

    /// As `search`, with a cap of its own for records: `limit` code chunks and conventions,
    /// `record_limit` records. A pack wants more record hits than code hits, and the pre-read
    /// hook wants records only (`limit` 0). This default searches once with the larger cap and
    /// keeps the best of each kind.
    async fn search_with(
        &self,
        repo: &str,
        query: &str,
        limit: usize,
        record_limit: usize,
    ) -> Result<Vec<SemanticHit>> {
        let mut hits = self.search(repo, query, limit.max(record_limit)).await?;
        hits.sort_by(|a, b| b.score.total_cmp(&a.score));
        let (mut records, mut chunks, mut conventions) = (0, 0, 0);
        hits.retain(|h| {
            let (seen, cap) = match h.kind {
                SemanticKind::Record => (&mut records, record_limit),
                SemanticKind::CodeChunk => (&mut chunks, limit),
                SemanticKind::Convention => (&mut conventions, limit),
            };
            *seen += 1;
            *seen <= cap
        });
        Ok(hits)
    }
}
