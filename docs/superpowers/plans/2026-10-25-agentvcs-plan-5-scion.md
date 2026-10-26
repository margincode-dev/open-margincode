# agentvcs Plan 5: Ideas from Scion (carried approvals, post-merge checks, workspace-free API, derived data) — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Port the parts of Anthropic's Scion design ("Redesigning source control", 2026) that fit a
single-machine agent fleet:
- the server does history, merges and rebases, and its API is complete enough that a job needs no
  checkout;
- derived data is computed per changeset from its parent;
- there is no merge queue: main is checked after merge, and bisect/revert are automated;
- approvals carry semantic constraints, so reviewers judge intent rather than every rebase.

**Why:** a field test with eight DeepSeek agents (headless Claude Code against DeepSeek's
Anthropic-compatible endpoint, every merge through av) hit each of these gaps:
- Half of the reviews were re-reviews after `av sync` resolved a README-only conflict, because an
  approval is bound to the exact head.
- A reviewer approved violations of a required convention as "style only".
- Merges that were textually clean could still break main, and nothing noticed.
- `undo` only reverts the newest merge.
- A reviewer read workspace folders directly because no read API exists.
- An MCP-only agent cannot `sync` or `validate`, so a conflict strands it.
- Orchestrators have to poll.

**Architecture:**
- New tables (migrations numbered at merge time, after master's highest): `events`, `checks`,
  `derived`, `repo_settings`, plus columns on `review_verdicts` and `conventions`.
- New read-only `Vcs` methods (`path_log`, `archive`) and one write method (`revert_merge`),
  implemented in `av-vcs` only.
- Diff helpers move to `av-pack/src/diff.rs`.
- Commands still run only on the host: the CLI (`av validate`, `av runner`) runs them and posts the
  results; the daemon, often in Docker, never executes repository code.

**Tech Stack:** unchanged (jj-lib =0.46, sqlx/Postgres, axum 0.8, hand-rolled MCP, clap + ureq),
plus `tar` for the archive endpoint.

## Global Constraints

- Plan 1–4 constraints still hold: Rust 1.97.1, fmt + `clippy -D warnings`, no SQLite, no LLM in
  `av-pack`, one PR per task off `master`, TDD.
- The daemon never runs repository commands. Validation and checks are attestations posted by a
  client (CLI or MCP); the tool descriptions say so.
- Carrying an approval fails closed. Missing data, a mode/exec change or a non-UTF-8 difference
  means no carry and an ordinary re-review. Every carry is visible: a `review` record by `av` names
  the original approval's head.
- A revert is a new commit on main, never a history rewrite. Its marker (`av revert {id}`) must not
  contain the merge marker (`av task {id}`).
- Event ids are committed in order: insert under `pg_advisory_xact_lock`, in the same transaction
  as the state change.
- Cached derived data is keyed by immutable commit id and a versioned kind (`blame_v1`). It never
  needs invalidation, only pruning.

## Tasks (order and dependencies)

| # | Task | Depends on |
|---|---|---|
| 1 | E: field-test UX fixes | — |
| 2 | C1: workspace-free read API (file, tree, diff, path log) + MCP read tools | — |
| 3 | C2: MCP `validate`/`sync`/`abandon`/`list_conventions` + `events` feed with long-poll | — |
| 4 | A: carried approvals | 2 (diff.rs), 3 (transactional transitions) |
| 5 | D1: enforced conventions (check commands, list/rm, approve gate) | — |
| 6 | B1: post-merge checks (`repo_settings.check_command`, `checks`, archive endpoint, `av runner`, main health) | 3 |
| 7 | B2: `av revert <task>` (any merge) + `av bisect` | 6 |
| 8 | D2: derived-data cache + `av why` blame | 2 (path_log) |
| 9 | F: open-source readiness (LICENSE, scrub personal data, CONTRIBUTING) | Plan 4 / housekeeping PRs merged |

### Task 1: Field-test UX fixes
- [ ] Failing tests:
  - CLI submit output says `submitted (in_review); advisory: N changed symbols have no record` on
    stdout (`av-cli/src/main.rs:644-647`).
  - `expand` accepts `conv:<id>` and `task:<slug>` (`av-pack/src/packer.rs:303-355`).
  - The Map's repo tree omits files already shown as skeletons (`assemble.rs` `repo_tree_item`;
    golden `basic.md`).
  - Another task's `intent` record ranks below this task's decisions (`score.rs`).
- [ ] Implement, then update the golden files with a reviewed diff.

### Task 2: Workspace-free read API
- [ ] Add an `at` resolver in `api.rs`: `main` → trunk, a hex change id, or `task:<slug>` → that
  task's `head_change`. A task read serves the last checkpoint, not live edits.
- [ ] Routes:
  - `GET /v1/repos/{r}/files/{*path}?at=` (raw bytes; 404 when missing)
  - `GET /v1/repos/{r}/tree?at=` (paths)
  - `GET /v1/repos/{r}/diff?from=&to=` (unified diff per changed path)
  - `GET /v1/repos/{r}/log?path=&limit=`
- [ ] Add `Vcs::path_log(repo, path, limit)` (default-erroring). The jj version walks `trunk_log`
  and compares `path_value(path)` against the parent's.
- [ ] Add `av-pack/src/diff.rs` with `unified_diff(a, b, path)` and `patch_signature(base, head)`.
  `patch_signature` is the removed/added text per op group, without line numbers, keeping hunk
  count and order. `bundle.rs` reuses it.
- [ ] MCP tools `read_file`, `list_files`, `diff`, `log`. CLI `av show <path> [--at]` and
  `av diff [--from --to]`.

### Task 3: MCP completeness + event feed
- [ ] MCP tools: `validate` (posts an agent-reported exit code: an attestation), `sync`, `abandon`,
  `list_conventions` (new `GET /v1/repos/{r}/conventions`).
- [ ] Migration: `events(id bigserial PK, repo, kind, task_id uuid NULL, change text NULL,
  payload jsonb, at timestamptz)` with an index on `(repo, id)`.
- [ ] `PgStore::transition` becomes a transaction that also inserts the event under
  `pg_advisory_xact_lock(<const>)`. `record_merge` reads `merge_commit` into the `merged` event's
  payload.
- [ ] `GET /v1/events?repo=&since=&wait=`: axum long-poll on `Arc<Notify>` in `AppState`. Arm
  `notified()` before querying; cap `wait` at 25 s; with no `since`, start at the current head.
- [ ] CLI `av events [--repo] [--follow]`. Tests: ordering under concurrent transitions (swarm),
  wake-up within the wait, no lost event across a commit race.

### Task 4: Carried approvals
- [ ] Migration: `review_verdicts` gets `base_change text` and `carried_from text`. New table
  `repo_settings(repo text PK, carry_globs text[] NOT NULL DEFAULT '{**/*.md}', check_command text)`.
- [ ] `IntentStore::latest_verdict_any(task)`. `add_verdict` stores `base` and `carried_from`.
- [ ] `flow::carry_approval(st, &task) -> Option<Carry>`: idempotent, never takes `MERGE_LOCK`.
  Called at the end of `submit` and `validate`, and in `merge` before `latest_verdict`. It carries
  only if all of these hold:
  - the newest verdict row is an approve, and its `carried_from` chain ends at a non-`av`
    reviewer;
  - a passing validation exists at the new head;
  - the file sets of the two patches are equal;
  - for each file, `patch_signature(base0→H0) == patch_signature(base→head)`, or the file is in
    `changed(base0,H0) ∩ changed(base0,base)` and matches `carry_globs`;
  - no mode/exec bit differs.
- [ ] On carry, insert an `approve` verdict at the head (reviewer `av`, `carried_from` = H0) and a
  `review` record. `status` shows it. The review bundle gains an "Interdiff since approval"
  section.
- [ ] CLI `av repo settings --repo r [--carry-globs ...]`.
- [ ] Tests:
  - pure rebase carries;
  - a README-only resolution carries;
  - a code change in the resolution does not;
  - a request-changes after the approval blocks;
  - missing `base_change` (old rows) fails closed.

### Task 5: Enforced conventions
- [ ] Migration: `conventions.check_command text`. `av convention add --check "<cmd>"`,
  `av convention list --repo`, `av convention rm <id>` (soft delete: `removed_at`).
- [ ] `GET /v1/tasks/{id}/checks` returns the required conventions that have a check and match the
  changed paths.
- [ ] `av validate` runs them in the workspace and posts
  `checks: [{convention_id, exit_code, output_tail}]`.
- [ ] `validation_at` aggregates the latest result per convention at the head.
- [ ] `review --verdict approve` is refused (422) when an applicable check failed or was not run;
  the message names the convention.

### Task 6: Post-merge checks
- [ ] Migration: `checks(id, repo, change, command, exit_code, output_tail, runner, at,
  UNIQUE(repo, change, command))`. `repo_settings.check_command` is set with
  `av repo settings --check "<cmd>"`.
- [ ] `POST /v1/repos/{r}/checks`. `GET /v1/repos/{r}/archive?at=` returns a tar of the tree
  (`Vcs::archive`, regular files only, exec bit kept).
- [ ] `av runner --repo r --trust`:
  - follows `/v1/events`;
  - for each `merged` event, unpacks the archive into a scratch dir;
  - runs the check command and posts the result.
  - Without `--trust` it refuses to run: the command comes from the daemon's database.
- [ ] Main health: `av log` marks each commit ✓/✗/·, `av status` shows "main is red since
  <task>", and packs add a Live work line while main is red.

### Task 7: Revert + bisect
- [ ] `Vcs::revert_merge(repo, merge_commit, message) -> MergeOutcome`. It computes
  `MergedTree::merge([(trunk,"main"), (M,"merge"), (M.parent,"before")])`, refuses when M has more
  than one parent, and writes nothing on conflict.
- [ ] `Event::Revert` (Merged → InReview). Fix `merged_already` to scan newest-first and stop at
  `av revert {id}`, so a reverted task can merge again.
- [ ] `av revert <task>` (owner, merger, or `--agent` with a reason recorded as a note).
- [ ] `av bisect --repo r`: binary search over trunk's linear history using recorded checks
  (asking the runner for missing ones). It prints the first bad commit, its task (anchored regex on
  the description's last line) and that task's intent and decisions.

### Task 8: Derived data + `av why`
- [ ] Migration: `derived(repo, kind, change, key, value jsonb, computed_at,
  PRIMARY KEY (repo, kind, change, key))`.
- [ ] `blame_v1(path)` at a change is a run-length list of (commit index, line count). It is
  computed forward from the nearest cached ancestor along `path_log`: lines a diff leaves
  unchanged inherit, and the walk stops at the first commit without an `av task` marker. It is
  computed eagerly after `record_merge` for changed paths.
- [ ] `GET /v1/repos/{r}/why?path=&line=|symbol=` returns the task, its intent, and decisions and
  attempts anchored to the path or symbol. CLI `av why path[:line|:Symbol]`; MCP tool `why`.

### Task 9: Open-source readiness
- [x] `LICENSE` (MIT, matching `Cargo.toml`).
- [x] Remove personal paths, the e-mail address and session links from tracked files. Move the
  internal handoff docs out of the repo. Make CLAUDE.md generic and add `CONTRIBUTING.md`.
- [x] Rename the project to **Scholion** (σχόλιον, a note in a manuscript's margin, as av's
  records are notes in the margins of the code): README, crate names `av-*` → `scholion-*` (the
  `av` binary name is decided in this task), docs and compose project names.
- [x] Open the README with "How this was built": the project is vibe-coded (AI coding agents
  wrote almost all of it, with one human steering and reviewing), Plan 5 is partly inspired by
  Anthropic's Scion, and the project is not affiliated with Anthropic. Add a section "Ideas
  borrowed from Scion".
- [x] Gate: `git ls-files | xargs grep -nE '<personal patterns>'` is empty; secrets scan of the
  history; CI green.
- Ruling: the CLI installs as `scholion` plus the alias `av` (same program, `src/alias.rs`); agent-facing
  text, env vars (`AV_*`), `.av-task`, the Docker volume names and the merge markers keep `av`. The
  public repo starts from one squashed commit; this private repo keeps its history.

## Verification

- Every task: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`.
- End to end:
  - `scripts/smoke.sh`.
  - The eight-agent DeepSeek stress run, before and after this plan. Compare re-reviews, rounds,
    convention violations caught, post-merge breaks caught, cost and wall time.
