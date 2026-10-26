# agentvcs Plan 3: Collaboration (handoff, review, merge, MCP, hooks, backup) — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Agents can hand a task to another agent, submit it for review, have it reviewed and merged into `main` (or flagged as conflicted), undo mistakes, and push merged history to a git remote as backup. Every capability is reachable over MCP, HTTP and the `av` CLI, and Claude Code hooks keep checkpoints and context flowing without the agent asking.

**Architecture:** The lifecycle is a state machine in `av-core` (pure, table-tested) driven by a new `av-flow` crate that composes the existing `Vcs`, `IntentStore` and `CodeIndex` traits; `av-vcs` gains the merge/undo/push operations (the only place touching jj-lib). Bundles (handoff / PR) are rendered by `av-pack` with the same token budgeter as context packs. `av-mcp` is a thin MCP façade over the daemon's service functions, mounted at `/mcp`; the HTTP routes and CLI call the same functions.

**Tech Stack:** jj-lib 0.46 (pinned), sqlx/Postgres, axum 0.8, the official Rust MCP SDK (`rmcp`, streamable-HTTP server feature; **Task 8 Step 1 pins the exact version and verifies the API before any code is written**), clap + ureq (CLI), bash (hook scripts).

**Spec:** `docs/superpowers/specs/2026-10-18-agentvcs-design.md` (sections "Agent workflow, handoff, PR" and "Failure handling & deployment"). Builds on Plan 1 (merged to `master`) and is independent of Plan 2 except that Task 5 reindexes through the `CodeIndex` trait, and Plan 2's outbox, if present, is fed by the same call.

## Global Constraints

- Everything from Plan 1's Global Constraints still holds (Rust pinned 1.97.1, fmt + `clippy -D warnings` clean, no SQLite, no LLM in `av-pack`, commit trailer `Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>`).
- Task status enum is unchanged: `open | handed_off | in_review | conflicted | merged | abandoned`. Legal transitions are exactly those in Task 1's table; anything else is `AvError::IllegalState` (new variant in `av-core`, mapped to 422 by the daemon; `Invalid` stays 400 and keeps meaning "malformed input", so Plan 1 tests are unaffected).
- `submit` and `handoff` are refused (`AvError::IllegalState`, HTTP 422) unless the task has an `intent` record and at least one record of kind `decision`, `attempt`, `note` or `handoff` with a non-empty summary. The refusal lists changed symbols that no record is anchored to.
- Merge is never silent about conflicts: a conflicted merge leaves `main` untouched, sets status `conflicted`, and the response names the conflicting paths.
- `main` is only ever advanced by `merge`. No operation rewrites or deletes a merged change except `undo`, which restores the jj operation before the merge and is itself recorded.
- Git backup push never blocks or fails a merge: failures are stored, retried with backoff (1 s, 2 s, 4 s … capped at 5 min) and surfaced by `status`.
- The daemon stays bound to localhost in compose. A shared-secret token (`AV_TOKEN`) is enforced on every route including `/mcp` once set (Task 9); unset means open, as in Plan 1.
- Workspace paths returned to clients are host paths (Plan 1 rule), including in bundles.
- **Trust model:** agents are cooperative. `agent` and `reviewer` names are self-asserted strings, so "reviewer != owner" and "undo only by the merging agent" prevent accidents, not impersonation. `AV_TOKEN` (Task 9) is the only access control. State this in the README.
- `ChangeId` stays the jj **commit id** hex (the Plan 1 shortcut). Any squash/rebase makes new commit ids, so verdicts, `head_change`, `merge_commit` and `last_pushed_change` bind to the exact commit they were recorded against; a re-snapshot after approval invalidates the approval (that is the "changed since approved" rule, not a bug).

## Review Focus

Inputs the spec implies but no feature task names; each is pinned by a test in the owning task.

1. **Submit with no changes** (head == base) → refused with "nothing to submit", status unchanged (Task 2).
2. **Two tasks edit the same hunk; the second merges after the first** → second becomes `conflicted`, `main` equals the first task's result, nothing half-applied (Task 5).
3. **Merge while the owner agent is still writing** into the workspace → the head check and the merge happen under one per-repo lock, so an edit between check and merge cannot slip in; two concurrent `merge` calls on one task → exactly one merges; a crash after main advanced but before the task row updated → the retry completes the transition without merging twice (Task 5).
4. **Reviewer is the owner / non-owner posts a review on a task not `in_review`** → refused (Task 4).
5. **Handoff of a task whose workspace was deleted by hand** → clear error, task keeps its state, records still readable (Task 3).
6. **Backup remote unreachable or rejecting non-fast-forward** → merge still succeeds, `status` shows the failure and next retry time (Task 6).
7. **MCP client sends malformed or oversized arguments** (e.g. 5 MB `body`) → JSON-RPC error, no partial write (Task 8).

## File Structure

```
crates/av-core/    src/lifecycle.rs (new; transitions)   src/types.rs, traits.rs (extended)
crates/av-vcs/     src/jj.rs (+merge, undo, op_log, diff_hunks, push)   tests/merge.rs (new)
crates/av-store/   migrations/0008_flow.sql              src/pg.rs (+claims, reviews, backup_state)
crates/av-flow/    (new) src/{lib,submit,review,merge,undo,backup}.rs   tests/flow.rs
crates/av-pack/    src/bundle.rs (new)   src/assemble.rs (+review pinning, conflict section)
crates/av-mcp/     (new) src/{lib,tools}.rs   tests/mcp.rs
crates/av-daemon/  src/api.rs (+routes)  src/auth.rs (new)  src/wiring.rs (+AV_TOKEN, AV_BACKUP_*)
crates/av-cli/     src/main.rs (+subcommands)  src/hook.rs (new)
integrations/claude-code/  settings.snippet.json  hooks/{session-start,post-edit}.sh  install.sh
integrations/generic/av-wrap.sh
```

---

### Task 1: Lifecycle state machine

**Files:** Create `crates/av-core/src/lifecycle.rs`; modify `crates/av-core/src/lib.rs`. Test: unit tests in `lifecycle.rs`.

**Interfaces — Produces:**

```rust
pub enum Event { Handoff, Claim, Submit, RequestChanges, Approve, MergeOk, MergeConflict, Resolve, Undo, Abandon }
pub fn next(status: TaskStatus, event: Event) -> Result<TaskStatus>   // Err(AvError::IllegalState) when illegal
```

Transition table (every other pair is illegal):

| from | event | to |
|---|---|---|
| open | Handoff | handed_off |
| handed_off | Claim | open |
| open | Submit | in_review |
| in_review | RequestChanges | open |
| in_review | Approve | in_review (approval is recorded, status unchanged) |
| in_review | MergeOk | merged |
| in_review | MergeConflict | conflicted |
| conflicted | Resolve | open (applied inside `Flow::submit`, Task 5; no route emits it directly) |
| merged | Undo | in_review (only `Flow::undo`, Task 5, emits it) |
| open, handed_off, in_review, conflicted | Abandon | abandoned |

- [ ] **Step 1: Failing tests** `legal_transitions_follow_the_table` (all rows), `abandoned_is_terminal_and_merged_allows_only_undo` (every event from `abandoned` → `Err`; from `merged` only `Undo` is legal), `illegal_pair_error_names_both_sides` (message contains `"open"` and `"MergeOk"`).
- [ ] **Step 2:** `cargo test -p av-core lifecycle` → FAIL (module missing).
- [ ] **Step 3:** Implement `next` as one exhaustive `match`; no wildcard arm for legal pairs.
- [ ] **Step 4:** `cargo test -p av-core` → PASS; clippy clean.
- [ ] **Step 5:** Commit `feat(core): task lifecycle state machine`.

### Task 2: Record discipline and the submit gate

**Files:** Create `crates/av-flow/{Cargo.toml,src/lib.rs,src/submit.rs}`, `crates/av-flow/tests/flow.rs`; modify `crates/av-core/src/traits.rs` (add `claim_task`, `transition`). Test: `tests/flow.rs` using `av-testutil::TestDb` and the `FakeVcs` pattern from `av-pack/tests/common`.

**Interfaces — Consumes:** `Vcs::snapshot`, `Vcs::changed_paths`, `IntentStore::{task_records, records_anchored_to, set_head}`, `CodeIndex::symbols_in`. **Produces:**

```rust
// av-core traits.rs
async fn transition(&self, id: TaskId, event: Event) -> Result<TaskStatus>;   // validates via lifecycle::next, atomic UPDATE ... WHERE status = $from
// av-flow
pub struct Flow { /* Arc<dyn Vcs>, Arc<dyn IntentStore>, Arc<dyn CodeIndex> */ }
pub struct SubmitReport { pub status: TaskStatus, pub head: ChangeId, pub changed: Vec<String>, pub unrecorded_symbols: Vec<String> }
impl Flow { pub async fn submit(&self, task: TaskId, agent: &str) -> Result<SubmitReport>; }
```

`unrecorded_symbols` = qnames of symbols (in changed files, via hunk overlap as in Plan 1's seeding) with no record anchored to them or to their file; it is advisory in the success response and listed in the refusal message when the gate fails.

- [ ] **Step 1: Failing tests:** `submit_refused_without_decision_or_note` (only the auto `intent` record exists → `IllegalState`, message contains "record a decision"), `submit_refused_when_head_equals_base` ("nothing to submit"), `submit_ok_moves_to_in_review_and_snapshots`, `submit_lists_unrecorded_symbols` (changed function `parse_x` without anchor appears in `unrecorded_symbols`; after recording an anchored note it does not), `concurrent_submit_only_one_wins` (two `submit` calls via `tokio::join!` → exactly one `Ok`, the other `IllegalState`).
- [ ] **Step 2:** `cargo test -p av-flow` → FAIL.
- [ ] **Step 3:** Implement `transition` in `PgStore` with a conditional `UPDATE` (a lost race returns `IllegalState`), then `Flow::submit`: snapshot → gate → `transition(Submit)` → return report.
- [ ] **Step 4:** `cargo test -p av-flow -p av-store` → PASS.
- [ ] **Step 5:** Commit `feat(flow): record discipline and submit gate`.

### Task 3: Handoff, claim, and the handoff bundle

**Files:** Create `crates/av-pack/src/bundle.rs`, `crates/av-flow/src/lib.rs` additions; modify `crates/av-pack/src/{lib.rs,budget.rs}` (add `allocate_with_shares`), `crates/av-store/src/pg.rs` (`claim_task`). Test: `crates/av-pack/tests/bundle.rs`, `crates/av-flow/tests/flow.rs`; golden file `crates/av-pack/tests/golden/handoff.md`.

**Interfaces — Produces:**

```rust
// av-pack
pub struct BundleRequest { pub task_id: TaskId, pub budget_tokens: u32, pub family: AgentFamily }
pub struct Bundle { pub markdown: String, pub tokens: u32, pub omitted: Vec<String> }
impl Packer { pub async fn handoff_bundle(&self, r: BundleRequest) -> Result<Bundle>; }
// av-flow
impl Flow {
    pub async fn handoff(&self, task: TaskId, agent: &str, state: HandoffState) -> Result<Bundle>;
    pub async fn claim(&self, task: TaskId, agent: &str) -> Result<Task>;       // handed_off -> open, owner_agent = agent
    // Lease + fencing (external-design note item 6): claim sets `lease_expires = now() + AV_LEASE_SECS (default 1800)`
    // and increments `tasks.generation`; the claim response carries the generation, `.av-task` stores it, and
    // checkpoint/record/submit send it. A write with a stale generation is IllegalState("claim lost"); an expired
    // lease lets another agent claim an `open` task too. Tests: `stale_generation_write_is_refused`,
    // `expired_lease_can_be_claimed_by_another_agent`.
}
pub struct HandoffState { pub done: String, pub remaining: String, pub broken: String, pub questions: String }
```

`budget.rs` today has a closed five-variant `Section` with fixed shares. Add `pub fn allocate_with_shares(items: Vec<Item>, budget: u32, family: AgentFamily, shares: &[(String, u8)]) -> Allocation` (extend `Item` with `group: Option<String>`; `allocate` keeps its behavior and delegates). Unit test in `tests/budget.rs`: `custom_shares_respect_budget_and_spill_over`.

Bundle sections in order: Goal · State (done / remaining / currently broken) · Records (decisions and dead ends first, via the Plan 1 budgeter, newest superseding first) · Changes (per-symbol diff summary; hunks only while the budget allows, otherwise handles) · Open questions. `handoff` evaluates the Task 2 gate on the records that ALREADY exist, then inserts the `handoff` record and applies the transition in one Postgres transaction (a failed gate leaves no record behind, and the new record cannot satisfy its own gate); it stores `HandoffState` as one `handoff`-kind record (summary = first 200 chars of `remaining`), runs the Task 2 gate, then `transition(Handoff)`.

- [ ] **Step 1: Failing tests:** `bundle_is_within_budget_for_budgets_150_1000_8000` (parameterised; `tokens <= budget`), `bundle_matches_golden_for_fixture_task`, `handoff_requires_gate_and_moves_to_handed_off`, `claim_changes_owner_and_reopens`, `failed_handoff_gate_leaves_no_handoff_record`, `claim_by_current_owner_is_refused` ("already yours"), `handoff_with_missing_workspace_errors_cleanly` (Review Focus 5: workspace dir removed; error mentions the path; status stays `open`).
- [ ] **Step 2:** `cargo test -p av-pack bundle` → FAIL.
- [ ] **Step 3:** Implement `bundle.rs` with `allocate_with_shares` and its own section shares (goal 10 · state 25 · records 30 · changes 30 · questions 5); `Flow::handoff`/`claim` as above.
- [ ] **Step 4:** `cargo test -p av-pack -p av-flow` → PASS; update golden once via `UPDATE_GOLDEN=1` and review the diff by eye.
- [ ] **Step 5:** Commit `feat(flow): handoff bundles and claim`.

### Task 4: Review — PR bundle, verdicts, pinned feedback

**Files:** Create `crates/av-flow/src/review.rs`, `crates/av-store/migrations/0008_flow.sql`; modify `crates/av-pack/src/bundle.rs`, `crates/av-pack/src/assemble.rs`, `crates/av-store/src/pg.rs`. Test: `crates/av-flow/tests/flow.rs`, `crates/av-pack/tests/pack.rs`.

**Interfaces — Produces:**

```rust
// migration 0008: review_verdicts(id bigserial PRIMARY KEY, task_id uuid, reviewer text, verdict text CHECK IN ('approve','request_changes'),
//                                 head_change text NOT NULL, created_at timestamptz DEFAULT now())   -- append-only; no uniqueness
// Merge is authorized only if the LATEST verdict row for (task, current head_change) is 'approve'; a later request_changes
// supersedes an earlier approve at the same head.
pub enum Verdict { Approve, RequestChanges }
impl Packer { pub async fn pr_bundle(&self, r: BundleRequest) -> Result<Bundle>; }   // handoff bundle + checklist + risks + test status
impl Flow {
    pub async fn post_review(&self, task: TaskId, reviewer: &str, verdict: Verdict,
                             findings: Vec<NewReviewFinding>) -> Result<Task>;
}
pub struct NewReviewFinding { pub summary: String, pub body: String, pub anchors: Vec<Anchor> }
```

PR bundle additions: **checklist** = conventions in scope (`conventions_for` over changed paths) rendered as `- [ ]` lines; **risk areas** = (a) `decision` records with no sibling `attempt` record on the same anchors ("no alternatives recorded"), (b) touched symbols whose reference in-degree ≥ 5 (new `CodeIndex::in_degree(repo, scope, ids) -> Result<HashMap<SymbolId, u32>>` counting `symbol_refs` rows by `dst`; add to `av-index` with a test); **test status** = the latest `validation` record bound to the task's current `head_change` (external-design note item 5): a new record kind `validation` with structured body `{command, exit_code, env_fingerprint, head_change}` posted by `av validate -- <cmd>` (the CLI runs the command in the workspace, then records it); none at the current head → "tests: not run at this head". `Approve` is refused (IllegalState) unless such a record with `exit_code == 0` exists at the head being approved; tests `approve_requires_passing_validation_at_head`, `validation_at_older_head_does_not_count`; `RequestChanges` → `transition(RequestChanges)`; `Approve` stores a verdict row bound to `head_change`. Packs for the owner include open `review` records of the current task pinned at the top of the History section (ranked above every `record_score` result, and a full-tier slot is reserved for them before `allocate` runs, so they appear at full tier even at budget 400).

- [ ] **Step 1: Failing tests:** `reviewer_cannot_be_owner`, `review_on_task_not_in_review_is_refused` (Review Focus 4), `request_changes_returns_task_to_open_and_pins_findings` (next `context` for the owner has the finding as the first History item, full tier, even with budget 400 and with an on-seed `attempt`/`decision` record present), `approve_binds_to_head_change`, `request_changes_after_approve_blocks_merge_even_at_same_head`, `pr_bundle_lists_conventions_checklist_and_risks`, `pr_bundle_reports_tests_not_reported_when_absent`.
- [ ] **Step 2:** `cargo test -p av-flow review` → FAIL.
- [ ] **Step 3:** Implement migration, store methods (`add_verdict`, `latest_verdict`), `post_review`, `pr_bundle`, pinning in `gather`.
- [ ] **Step 4:** `cargo test --workspace` → PASS.
- [ ] **Step 5:** Commit `feat(flow): review verdicts, PR bundle and pinned findings`.

### Task 5: Merge (squash, rebase, conflicts) and undo

**Files:** Create `crates/av-flow/src/{merge,undo}.rs`, `crates/av-vcs/tests/merge.rs`; modify `crates/av-core/src/traits.rs` (Vcs), `crates/av-vcs/src/jj.rs`, `crates/av-pack/src/assemble.rs` (conflict section). Test: `crates/av-vcs/tests/merge.rs`, `crates/av-flow/tests/flow.rs`.

**Interfaces — Produces** (added to `Vcs`):

```rust
pub enum MergeOutcome { Merged { main: ChangeId, changes: Vec<ChangeId> }, Conflicted { paths: Vec<String> } }
async fn merge_task(&self, repo: &str, slug: &str, expected_head: &ChangeId, message: &str, task_id: TaskId) -> Result<MergeOutcome>;
//   Holds the per-repo mutex for the whole call. 1. snapshot the task workspace and return
//   Err(IllegalState("changed since approved")) if the snapshot != expected_head. 2. The workspace is ONE working-copy
//   commit (snapshot rewrites it in place), so there is nothing to squash: split its diff against base into
//   one commit per top-level directory touched (cap 5; one commit otherwise), built by tree construction, with
//   `message`. 3. rebase those commits onto the current trunk; a rebase of a conflicting commit SUCCEEDS and writes
//   a conflicted commit, so check every rebased commit (`Commit::has_conflict`) and collect conflicting paths from
//   its merged tree. 4. Any conflict -> drop the transaction (trunk untouched) -> Conflicted.
//   Otherwise advance the trunk bookmark and call `git::export_refs` in the SAME transaction, so the bare git dir's
//   `refs/heads/<trunk>` equals the new trunk; the op description is `av merge <task_id>`.
async fn rebase_workspace(&self, repo: &str, slug: &str) -> Result<RebaseOutcome>;  // rebases the task onto current trunk, keeps conflicts as markers in the workspace, updates base; RebaseOutcome { head: ChangeId, conflicts: Vec<String> }
async fn undo_merge(&self, repo: &str, pre_merge: &ChangeId, merge_commit: &ChangeId) -> Result<()>;
//   jj-lib 0.46 has no ready-made undo/restore-operation (verify with grep before coding), and the op log is shared
//   by every workspace, so restoring an old operation would also rewind other tasks' merges and snapshots. Instead,
//   under the repo mutex: refuse (IllegalState("main moved since merge")) unless trunk == merge_commit; otherwise a
//   NEW forward operation sets the trunk bookmark back to `pre_merge`, then `git::export_refs`.
async fn op_log(&self, repo: &str, limit: usize) -> Result<Vec<OpSummary>>;  // id, description, time (read-only; used by `av ops` and merge crash-recovery)
// av-flow
impl Flow {
    pub async fn merge(&self, task: TaskId, agent: &str) -> Result<MergeReport>;   // needs a verdict row == head_change
    pub async fn undo(&self, task: TaskId, agent: &str) -> Result<UndoReport>;
}
pub struct MergeReport { pub status: TaskStatus, pub main: Option<ChangeId>, pub conflicts: Vec<String>, pub backup: BackupState }
```

Migration `0008_flow.sql` also adds `tasks.pre_merge_commit text`, `tasks.merge_commit text` and `tasks.merged_at timestamptz` (all null until merged).

`merge`: (0) crash recovery first: if the task is `in_review` and `op_log` shows `av merge <task_id>` as the newest op on trunk, main already has the merge, so skip to the `MergeOk` transition with the trunk commit as `merge_commit` (never merge twice); (1) require a verdict row whose `head_change` is the one passed as `expected_head`; (2) record `pre_merge_commit` = current trunk; (3) call `merge_task` (head check, merge and ref export all happen under its lock; concurrent callers serialize and the loser sees a non-`in_review` task or a stale head and gets `IllegalState`); on success `MergeOk` → reindex `main` scope for the changed paths (`read_file` each path from the new trunk and pass `(path, bytes)` to `CodeIndex::index_paths`; `remove_paths` only for paths absent from the new trunk) and re-anchor nothing (records keep path anchors); `MergeConflict` → status `conflicted`, store conflicting paths, and the owner's next pack gets a **Conflicts** section: conflicting symbols plus the other task's records (found with a new `IntentStore::tasks_touching(repo, paths, statuses: &[TaskStatus], since: Option<DateTime<Utc>>)` over `open` ∪ `merged` tasks, using a new `tasks.merged_at` column; `open_tasks_touching` only returns `open`). This needs a new pack section: add `Section::Conflicts` with a heading and a share (take 5 points from Code: map 10 · task 10 · history 20 · code 45 · conventions+live 10 · conflicts 5) in `budget.rs` and `render.rs`, with a golden-file update. **Leaving `conflicted`:** `Flow::submit` on a `conflicted` task runs `rebase_workspace` (the conflicts appear as markers in the workspace), snapshots, and if `conflicts` is still non-empty returns `IllegalState` listing the paths; otherwise it applies `Resolve` then `Submit` in one conditional `UPDATE` (`conflicted → open → in_review`) and records a `note` `rebased onto main`. Prior verdicts do not carry over (new head). `undo` is allowed only for the merging agent, only while trunk still equals `merge_commit` (see `undo_merge`), sets the task back to `in_review`, and re-syncs the `main` index scope: for each path the task merged, read it from the restored trunk; if present `index_paths` it, only if absent `remove_paths` it (never delete a file the task merely modified). It adds a `note` record `undo of <merge_commit>`.

- [ ] **Step 1: Failing tests (av-vcs):** `merge_fast_forwards_main_and_exports_git_ref` (after merge `git -C <repo>/git rev-parse refs/heads/<trunk>` equals the returned `main`), `merge_splits_into_one_change_per_top_level_dir_up_to_five`, `merge_refuses_when_workspace_head_differs_from_expected`, `rebase_workspace_reports_conflict_paths_and_writes_markers`, `undo_merge_refused_after_a_later_merge` (A merges, B merges, undo A → refused, B's content intact), `merge_conflict_leaves_main_untouched` (Review Focus 2: two workspaces edit the same line; second returns `Conflicted{paths:["a.py"]}` and `list_files`/`read_file` of main equal the first merge), `undo_merge_restores_trunk_and_git_ref`, `op_log_lists_merge_then_snapshot`.
- [ ] **Step 2:** `cargo test -p av-vcs --test merge` → FAIL.
- [ ] **Step 3:** Implement with jj-lib 0.46. Before coding, grep `~/.cargo/registry/src/*/jj-lib-0.46*/` for the rewrite/rebase helpers, `git::export_refs`, bookmark setters and conflict APIs, and record the real names as a ledger ruling. Remember `rebase_descendants()` after every rewrite (Plan 1 gotcha).
- [ ] **Step 4:** `cargo test -p av-vcs` → PASS.
- [ ] **Step 5:** **Failing tests (av-flow):** `merge_requires_matching_approval`, `merge_refuses_when_head_changed_after_approval`, `concurrent_merges_only_one_wins`, `crash_after_merge_task_resumes_without_remerge` (simulate by running `merge_task` directly, then `Flow::merge`), `merge_ok_reindexes_main_and_marks_merged`, `merge_conflict_sets_conflicted_and_pack_names_conflicting_symbols`, `conflict_fix_resubmit_merge` (conflict → owner fixes markers → `submit` → approve → `merge` succeeds), `undo_returns_task_to_in_review_and_main_to_previous`, `undo_of_a_modified_file_keeps_it_indexed`. Run → FAIL; implement; run → PASS.
- [ ] **Step 6:** Commit `feat(flow): merge with first-class conflicts, and undo`.

### Task 6: Git backup push

**Files:** Create `crates/av-flow/src/backup.rs`; modify `crates/av-vcs/src/jj.rs` (`push_backup`), `crates/av-store/migrations/0008_flow.sql` (`backup_state(repo text PRIMARY KEY, remote text, last_pushed_change text, failures int, last_error text, next_retry timestamptz)`), `crates/av-daemon/src/wiring.rs`. Test: `crates/av-vcs/tests/merge.rs`, `crates/av-flow/tests/flow.rs`.

**Interfaces — Produces:**

```rust
// Vcs
async fn push_backup(&self, repo: &str, remote_url: &str) -> Result<ChangeId>;   // pushes trunk bookmark only; never force
pub struct BackupState { pub remote: Option<String>, pub last_pushed: Option<ChangeId>, pub failures: u32,
                         pub last_error: Option<String>, pub next_retry: Option<DateTime<Utc>> }
impl Flow { pub async fn run_backup(&self, repo: &str) -> Result<BackupState>; }   // called by the retry loop and fired (not awaited) after merge: `merge` returns as soon as main advanced; the push runs in a spawned task with a 60 s timeout on the git process, and its outcome only updates `backup_state`
```

`push_backup` pushes the git ref that `merge_task`/`undo_merge` already exported (Task 5); it never touches jj state. Remote comes from `AV_BACKUP_REMOTE_<REPO>` or `av repo backup set <repo> <url>`. A remote that is a local bare repo path (tests) or an SSH/HTTPS URL both work through `git push` on the colocated bare git dir; credentials are the host's, mounted read-only (document in README). Backoff doubles from 1 s, capped at 300 s; the daemon runs one retry loop per configured repo with `tokio::time::interval` and shuts it down with the server.

- [ ] **Step 1: Failing tests:** `push_backup_to_local_bare_remote_has_main_commit` (clone result contains the merged commit), `push_backup_non_fast_forward_is_an_error_not_a_force`, `merge_succeeds_when_remote_unreachable_and_records_failure` (Review Focus 6), `backoff_doubles_and_caps_at_300s` (pure function over `failures`), `successful_retry_clears_failure_state`.
- [ ] **Step 2:** `cargo test -p av-flow backup` → FAIL.
- [ ] **Step 3:** Implement `push_backup` via `git -C <bare> push -- <url> refs/heads/<trunk>` through the existing `git_cmd` helper (same validation: reject URLs starting with `-` or `ext::`), plus state handling in `backup.rs`.
- [ ] **Step 4:** `cargo test --workspace` → PASS.
- [ ] **Step 5:** Commit `feat(flow): git backup push with backoff`.

### Task 7: HTTP routes and CLI for the new lifecycle

**Files:** Modify `crates/av-daemon/src/{api.rs,lib.rs}`, `crates/av-cli/src/main.rs`, `crates/av-cli/src/client.rs`. Test: `crates/av-daemon/tests/api.rs`, `crates/av-cli/tests/cli.rs`.

**Interfaces — Produces:** routes (all under `/v1/tasks/{id}`): `POST /handoff` (body `HandoffState` + `agent`), `POST /claim`, `POST /submit`, `GET /bundle?kind=handoff|pr&budget=`, `POST /review` (`reviewer`, `verdict`, `findings[]`), `POST /merge`, `POST /undo`, `GET /ops`; `GET /v1/repos/{name}/backup`; `GET /v1/repos/{name}/search?q=&limit=` (≤ 20) over `IntentStore::search_records_fts(repo, query, limit) -> Result<Vec<Record>>` (Postgres `records.tsv @ plainto_tsquery`, superseded hidden; Plan 2 Task 2 defines the same method, so whichever plan lands first adds it and the other reuses it). CLI: `av search <query>`. `status` response gains `backup`, `conflicts`, `verdicts`. CLI: `av handoff --done … --remaining … [--broken …] [--question …]`, `av claim <task>`, `av submit`, `av bundle [--pr] [--budget N]`, `av review <task> --verdict approve|request-changes [--finding "summary|path:symbol|body"]…`, `av merge`, `av undo`, `av ops`.

- [ ] **Step 1: Failing tests:** one daemon test per route walking the happy path `start → record → checkpoint → submit → review(approve) → merge` against `TestDb` + real `JjVcs` on a fixture repo, asserting statuses at each step; `illegal_transition_returns_422_with_reason`; CLI tests for `av submit` output (prints changed symbols lacking records) and non-zero exit on refusal.
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement routes as thin wrappers over `Flow`; map `AvError::IllegalState` → 422 (add the variant and the mapping in `crates/av-daemon/src/error.rs`; existing 400 mappings stay). **Step 4:** `cargo test --workspace` → PASS. **Step 5:** Commit `feat(api,cli): lifecycle routes and commands`.

### Task 8: MCP server

**Files:** Create `crates/av-mcp/{Cargo.toml,src/lib.rs,src/tools.rs,tests/mcp.rs}`; modify `crates/av-daemon/src/{lib.rs,api.rs}` (mount at `/mcp`), `Cargo.toml` (workspace dep). Test: `crates/av-mcp/tests/mcp.rs`.

**Interfaces — Produces:** MCP tools, each a thin call into the same service functions as Task 7, with names exactly: `start_task`, `checkpoint`, `record`, `get_context`, `expand`, `read_symbol`, `search_history`, `handoff`, `claim_task`, `submit`, `review_task`, `post_review`, `merge`, `undo`, `status`. `read_symbol(id)` = `expand("sym:<id>")`; `search_history(query, limit≤20)` = the Task 7 search route's service function (full-text; Plan 2's semantic layer later feeds the same function). Tool descriptions carry the discipline text: *"Call `record` with kind=attempt immediately when an approach fails, including why."* The MCP session id is used as the ledger session so `get_context` dedups per MCP connection automatically; the agent name comes from the `clientInfo` of `initialize`.

- [ ] **Step 1: Verify the SDK.** Check crates.io for the current `rmcp` release (and its streamable-HTTP server feature) and that it supports the Rust toolchain pin; read its server example; write the chosen version and any API differences in the ledger as a ruling. If no suitable crate exists, implement the minimal JSON-RPC subset (`initialize`, `tools/list`, `tools/call`) over a single `POST /mcp` instead (the spec only needs request/response) and ledger that.
- [ ] **Step 2: Failing tests:** `tools_list_contains_all_fifteen_tool_names`, `start_task_then_get_context_roundtrip_over_mcp`, `second_get_context_in_same_session_marks_items_sent_earlier`, `malformed_arguments_return_jsonrpc_error_and_write_nothing` (record with missing `summary`, and a 5 MB `body` → error, `task_records` unchanged; Review Focus 7), `unknown_tool_is_method_error`.
- [ ] **Step 3:** Run → FAIL. **Step 4:** Implement; set axum's `DefaultBodyLimit` to 8 MiB and enforce the real cap (`body` ≤ 64 KiB, `summary` ≤ 200 chars, total arguments ≤ 1 MiB) inside the MCP handler so violations come back as JSON-RPC errors, not HTTP 413. **Step 5:** `cargo test -p av-mcp` → PASS; manually: `claude mcp add --transport http av http://localhost:7420/mcp`, then `claude mcp list` shows `av` connected (**requires the Docker stack or a locally started daemon**; record the result in the README).
- [ ] **Step 6:** Commit `feat(mcp): streamable HTTP MCP server with the fifteen agent tools`.

### Task 9: Hooks, wrapper, auth token, and Plan 1 follow-ups

**Files:** Create `crates/av-cli/src/hook.rs`, `integrations/claude-code/*`, `integrations/generic/av-wrap.sh`, `crates/av-daemon/src/auth.rs`; modify `crates/av-daemon/src/api.rs` (`expand` repo check), `crates/av-store/migrations/0008_flow.sql` (prune support), `crates/av-index/src/store.rs` (scope cleanup), `README.md`. Test: `crates/av-cli/tests/cli.rs`, `crates/av-daemon/tests/api.rs`, `crates/av-index/tests/`.

**Interfaces — Produces:**

- `av hook session-start` → prints a ≤ 800-token pack for the task in the current directory (silent exit 0 if there is no `.av-task`); `av hook post-edit` → debounced checkpoint (skips if the previous one was < 2 s ago, tracked in `.av-task`), exits 0 even when the daemon is down (hooks must never break the agent), writing one line to stderr.
- `integrations/claude-code/settings.snippet.json`: `SessionStart` → `session-start.sh`; `PostToolUse` with matcher `Edit|Write|MultiEdit` → `post-edit.sh`; `install.sh` merges the snippet into `~/.claude/settings.json` with `jq`, after copying the previous file to `settings.json.bak`, and refuses to run if `jq` is missing.
- `integrations/generic/av-wrap.sh <cmd…>`: runs `av context` into `$AV_CONTEXT_FILE`, runs the command, then `av checkpoint`.
- Auth: `AV_TOKEN` set → every route (including `/mcp`) requires `Authorization: Bearer <token>`, compared in constant time; `/healthz` stays open. `crates/av-cli/src/client.rs` sends `Authorization: Bearer $AV_TOKEN` on every request (add it to this task's file list), including the hook commands.
- Cleanup: `PgIndex::drop_scope(repo, scope)` called on `merge` and `abandon` (when Plan 2's outbox exists it first enqueues a `Delete` per `code_chunks` row of that scope, so no orphan Qdrant points remain); `POST /v1/maintenance/prune` deletes ledger rows for sessions idle > 30 days (add `ledger.last_seen timestamptz DEFAULT now()` in migration 0008 and set it in `ledger_put`) and `parse_cache` rows unreferenced by any scope; `expand` verifies the record's/symbol's repo equals the task's repo (Plan 1 deferred minor; `get_symbols` must also return `repo`, which `SYMBOL_COLS` currently omits).

- [ ] **Step 1: Failing tests:** `hook_session_start_is_silent_without_task_file`, `hook_post_edit_exits_zero_when_daemon_down`, `hook_post_edit_debounces_within_two_seconds`, `token_required_when_configured_and_healthz_open`, `wrong_token_is_401_and_comparison_is_constant_time` (status-only assertion; the compare hashes both sides with SHA-256 and uses `subtle::ConstantTimeEq` on the digests, so length differences do not short-circuit; this is not a timing test), `expand_rejects_item_from_another_repo`, `drop_scope_removes_symbols_but_not_parse_cache_entries`, `prune_removes_idle_sessions_and_orphan_cache`.
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement. **Step 4:** `cargo test --workspace`; `bash -n` and `shellcheck` (if installed) on every script; run `install.sh` against a temp `HOME` and assert the resulting JSON still parses and contains both hooks.
- [ ] **Step 5:** Update README (hooks install, MCP add command, backup, token) and the "Deferred minors" list; commit `feat: hooks, wrapper, auth token, scope cleanup`.

---

## Verification that needs Docker (cannot run in the build environment)

Compose smoke test of the full lifecycle through `/mcp`, backup push from inside the container with a mounted SSH key, and the Claude Code install on a real `~/.claude`. List the outcome of each in the final report; do not claim them verified without a run.
