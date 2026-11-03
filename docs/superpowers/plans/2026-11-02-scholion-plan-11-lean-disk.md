# Plan 11: Lean Disk — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** scholion accounts for and reclaims what agents create on disk. Workspaces of merged and
abandoned tasks are removed after a retention period (and restored when a merge is undone or
reverted); every task workspace of a repo shares one cargo build cache; the hooks tell a session
what it left on disk outside its workspace; `av disk` and `av gc` show and reclaim it.

**Context.** Claude Code sessions filled an Ubuntu WSL2 distro to 333 GB; it was deleted
(salvage: `D:\Projects\wsl-pack-2026-11-02`, whose README lists what was left behind). The point
is the files **Claude Code created by itself** while working: build directories, clones, test
data, scratch. Nothing tracked them and nothing owned their lifetime. This is a Community feature:
if scholion misses something or has gaps, build that.

**What the sessions created, and why it stayed** (salvage README, the Docker volumes still on the
machine, the code):

| Created by the agent | Size | Why it stayed |
|---|---|---|
| A cargo `target/` per place it built: one per task workspace, `~/dogfood/target-*`, `/tmp` | ~24 GB **each** (the `sch-target` volume is one build of this workspace; `ee-target` 23 GB) | scholion gives every task a full checkout under `~/ws/<repo>/<slug>` and sets no shared target dir; nothing deletes a workspace, ever. Ten builds ≈ 240 GB. **This is most of the 333 GB.** |
| Task workspaces, `~/avtest` (daemon test data), `~/ws` | 2 GB + 3.2 GB | Never removed on merge/abandon/undo/revert; slugs never reused (`Vcs` has no delete/forget). |
| Bench clones and venvs (`~/bench-work`) | ~1 GB per session (`bench/README.md:228-230`) | `scholion-bench` keeps every scrubbed clone; av bench tasks are never abandoned. |
| Ad-hoc dirs outside any workspace (`~/dogfood`, `~/avtest`, `~/.grok/worktrees`) | tens of GB | No hook sees them; the agent is never asked about them; `av` has no disk view. |
| Claude Code's own data (`~/.claude`) | 0.1 GB | Not the problem: `cleanupPeriodDays` (30) sweeps it; worktrees are auto-removed. |

**Gaps in scholion** (verified): no workspace removal anywhere (`flow::record_merge` / `abandon`
only drop index rows); no `gc`/`disk` command; no `.cargo/config.toml` written; the hooks only
look at edits, never at what a session leaves on disk; `scholion-bench` never cleans up.

**Design facts:**
- Cargo probes `.cargo/config.toml` in the cwd and **every parent**; deeper files win, env
  (`CARGO_TARGET_DIR`) beats files; relative paths resolve against the parent of `.cargo`. One
  file at `<ws_root>/<repo>/.cargo/config.toml` covers every `<ws_root>/<repo>/<slug>` without
  touching the repo tree or any jj snapshot; the repo's own config or the user's env still wins.
- Claude Code: a `SessionStart` hook may append `export VAR=…` lines to `$CLAUDE_ENV_FILE`; they
  apply to every later Bash command of the session (docs: hooks.md "Persist environment
  variables"). Hook input carries `cwd` and `session_id`. `Stop` hooks may block with a reason.
- jj-lib 0.46: `View::remove_workspace(&WorkspaceName)` (view.rs:123) is what `jj workspace
  forget` ends in (find the `MutableRepo` wrapper with `grep -n remove_w src/repo.rs`).
  `GitBackend::gc` exists but shells out to `git gc`, so store gc is a later opt-in.
- Lifecycle: `merged` can go back to `open` (revert) or `in_review` (undo); `abandoned` is
  terminal. A task's head commit stays in jj without its working copy, so a workspace can be
  re-created at the head when a merge is undone or reverted. Open / handed-off / in-review /
  conflicted workspaces are never touched.
- `events` already logs `status:<status>` per task with a timestamp (`set_status_reporting`,
  `scholion-store/src/pg.rs`), so "merged N seconds ago" needs **no migration** (numbers clash
  with agentvcs today).

**Tech Stack:** Rust 1.97.1 (pinned), axum daemon, Postgres (`PgStore`), jj-lib `=0.46.0`, clap
CLI, Claude Code hooks.

**Base branch:** `main`. Branch `plan-11-lean-disk`, one PR.

## Global Constraints

- TDD per task (CONTRIBUTING.md): the failing test first. Before each commit:
  `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`.
- Agent-facing text says `av`, never `scholion`.
- Do not change `ext.rs` signatures (the commercial edition builds on them).
- No migration (numbers clash with agentvcs). New `Vcs` and `IntentStore` methods get default
  impls so test doubles and agentvcs's older tree keep compiling.
- The daemon deletes only under `ws_root`; never anything else. The daemon never runs repository
  commands.
- No new MCP tools (`tests/mcp.rs` size guard).

---

### Task 1: `scholion-vcs`: remove and restore a workspace

**Files:**
- Modify: `crates/scholion-core/src/traits.rs` (`Vcs`)
- Modify: `crates/scholion-vcs/src/jj.rs`
- Test: `crates/scholion-vcs/tests/vcs.rs`

**Interfaces:**
- `async fn remove_workspace(&self, repo, slug) -> Result<bool>`: forget in jj (transaction:
  remove the wc commit, `rebase_descendants()`, op "forget workspace {slug}"), then
  `remove_dir_all(ws_dir)`. Idempotent (`Ok(false)` when nothing to do). Refuses `main`.
- `async fn restore_workspace(&self, repo, slug, head: &ChangeId) -> Result<WorkspaceInfo>`:
  refactor `create_workspace` (jj.rs:342-369) into `create_workspace_at(repo, slug, commit)`;
  the old one checks out trunk, the new one `head`. If jj still knows the name (dir deleted by
  hand, as `handoff_with_missing_workspace_errors_cleanly` does), forget first.

- [ ] **Step 1: Write the failing tests** beside the existing workspace tests: remove deletes the
  dir and the name leaves the view; restore brings back the content of `head`; remove twice is
  `Ok(false)`; `main` refused.
- [ ] **Step 2: Run** `cargo test -p scholion-vcs`. Expected: FAIL (missing methods).
- [ ] **Step 3: Implement** both methods (default impls on `Vcs` return `AvError::Vcs`).
- [ ] **Step 4: Run** `cargo test -p scholion-vcs`. Expected: PASS.
- [ ] **Step 5: Commit**: `vcs: remove and restore a task workspace`.

### Task 2: daemon: everything built inside a workspace dies with it

**Files:**
- Create: `crates/scholion-daemon/src/disk.rs` (routes merged in `api::routes()` like `office`)
- Modify: `crates/scholion-daemon/src/wiring.rs`, `lib.rs`, `serve.rs`, `flow.rs`, `api.rs`
- Modify: `crates/scholion-core/src/traits.rs` (`IntentStore`), `crates/scholion-store/src/pg.rs`
- Test: `crates/scholion-daemon/tests/disk.rs`

**Interfaces:**
- `Config.ws_retain_secs: Option<u64>` from `AV_WS_RETAIN_SECS` (default **3600**; `never`
  disables; `0` = at once), in `wiring.rs` next to `lease_secs`; `AppState::with_ws_retain`.
- `PgStore::reclaimable_tasks(older_than_secs)`: `merged`/`abandoned` tasks whose newest
  `status:merged`/`status:abandoned` event is older than the cutoff. On `IntentStore` with a
  default impl returning empty (agentvcs's older store keeps compiling).
- `reclaim(st, retain, dry_run, build_cache) -> Reclaimed`: per reclaimable task whose `ws_dir`
  exists: `lock_task`, measure, `vcs.remove_workspace`, remove the task's scratch dir
  `<ws_root>/<repo>/.scratch/<slug>` (Task 4), insert event `workspace:reclaimed`. Returns
  `{reclaimed: [{repo, slug, status, path (host), bytes}], freed_bytes, kept: [...]}`.
- Sweep in `serve.rs` next to the backup loop (`RECLAIM_TICK` 60 s; skipped when `never`). With
  `0`, `record_merge` and `abandon` call `reclaim` inline for that task and answer
  `"workspace": "removed"`, which the MCP `merge`/`abandon` text relays (the agent's cwd is gone).
- Restore where a merged task comes back: `flow::undo` (after `undo_merge`) and `flow::revert`
  (after `revert_merge`) call `ensure_workspace(st, &task)`: missing dir →
  `vcs.restore_workspace(repo, slug, &task.head_change)` + `store.set_workspace`.
- `GET /v1/disk`: `{ws_root, store_root, pg_dir, repos: [{name, store_bytes,
  build_cache_bytes, workspaces: [{slug, status, bytes, age_secs, reclaimable}]}], outside:
  [...] (Task 4), totals}`. Sizes by a blocking walk in `spawn_blocking` (reuse the walker crate
  already in `Cargo.lock`, `ignore` or `walkdir`).
- `POST /v1/gc` body `{dry_run?, retain_secs?, build_cache?}` → `Reclaimed`
  (`build_cache: true` also deletes `<ws_root>/<repo>/.cache`). The daemon deletes only under
  `ws_root`; never anything else.

- [ ] **Step 1: Write the failing tests** in `tests/disk.rs` on `tests/common` (`env_with`,
  `start_task`, `submit`, `edit_greeter`, `call`):
  - merge → gc(retain 0) removes the dir and jj forgets it;
  - undo after gc restores a workspace holding the greeter edit;
  - abandon → reclaimed;
  - open and in-review never reclaimed;
  - retain honoured (`kept`);
  - `dry_run` removes nothing;
  - `/v1/disk` lists the workspace with `bytes > 0` and `reclaimable`;
  - the sweep (retain 0, tick shortened by `with_reclaim_tick`) removes a merged task's dir
    without a `gc` call.
- [ ] **Step 2: Run** `cargo test -p scholion-daemon --test disk`. Expected: FAIL.
- [ ] **Step 3: Implement** config, store queries, `disk.rs`, the sweep, the inline path and the
  restore in `undo`/`revert`.
- [ ] **Step 4: Run** `cargo test -p scholion-daemon`. Expected: PASS
  (`handoff_with_missing_workspace_errors_cleanly` unchanged).
- [ ] **Step 5: Commit**: `daemon: workspaces of merged and abandoned tasks are reclaimed (Plan 11)`.

### Task 3: one build cache per repo (the 24 GB × N fix)

**Files:**
- Create: `crates/scholion-daemon/src/build_cache.rs`
- Modify: `crates/scholion-daemon/src/api.rs` (`import`, `sync_repos`), `wiring.rs`
- Test: `crates/scholion-daemon/tests/disk.rs`

**Interfaces:**
- `ensure_build_cache(cfg, repo, trunk_has_cargo_toml)`: when `AV_BUILD_CACHE` is not `off`
  (`Config.build_cache`, default on) and trunk has `Cargo.toml` at the root
  (`Vcs::list_files(repo, trunk)`), write `<ws_root>/<repo>/.cargo/config.toml`:
  ```toml
  # written by av: one cargo target for every task workspace of this repo.
  # Delete this file, or set AV_BUILD_CACHE=off before `av up`, to opt out.
  [build]
  target-dir = ".cache/cargo-target"
  [profile.dev]
  debug = "line-tables-only"   # as .github/workflows/ci.yml, for the same reason (~450 MB per test binary)
  ```
  Written only when absent or when line 1 is the marker (a user's file is never overwritten).
  Relative path ⇒ `<ws_root>/<repo>/.cache/cargo-target` on the host; no `ws_host_root` needed.
- Called from `api::import` after `init_repo` (api.rs:281) and from `api::sync_repos` at
  start-up for repos imported earlier.
- Document the trade-off: cargo locks the target dir, so parallel agents' builds of one repo
  queue. Python (`uv`) and Node (`npm`/`pnpm`) already keep global caches: nothing to write.

- [ ] **Step 1: Write the failing tests** in `tests/disk.rs`:
  - a fixture repo with `Cargo.toml` → file with the marker;
  - `demo` → none;
  - a pre-existing user file left byte-identical;
  - `build_cache: false` → none;
  - `POST /v1/gc {build_cache: true}` removes `.cache` and reports bytes.
- [ ] **Step 2: Run** `cargo test -p scholion-daemon --test disk`. Expected: FAIL.
- [ ] **Step 3: Implement** `build_cache.rs` and the two call sites.
- [ ] **Step 4: Run** `cargo test -p scholion-daemon`. Expected: PASS.
- [ ] **Step 5: Commit**: `daemon: one cargo build cache per repo`.

### Task 4: the hooks account for what a session creates

**Files:**
- Modify: `crates/scholion-cli/src/hook.rs` (`session-start`, `stop`)
- Create: `crates/scholion-cli/src/footprint.rs` (pure: scan + diff, unit-tested on temp dirs)
- Modify: `crates/scholion-daemon/src/disk.rs`, `lib.rs` (`AppState.footprints`)

**Interfaces:**
- **A reclaimed scratch dir.** `session-start` creates `<ws_root>/<repo>/.scratch/<slug>` (host
  path from the task's `workspace_path` parent) and, when `$CLAUDE_ENV_FILE` is set, appends
  `export AV_SCRATCH=<that dir>`. The pack's Task section gets one line: "Temporary files:
  `$AV_SCRATCH` (removed with the task)". Reclaimed by Task 2. Nothing is done to `TMPDIR`.
- **Footprint snapshot.** `session-start` (every session, task or not, daemon reachable) runs a
  bounded scan: roots `$HOME` and `$TMPDIR` (plus `AV_FOOTPRINT_ROOTS`), depth ≤ 4, collecting
  directories whose name is in the heavy list (`target`, `node_modules`, `.venv`, `venv`, `dist`,
  `build`, `.cache`, `worktrees`, `__pycache__` dropped as noise) with their size (walk, capped
  at `AV_FOOTPRINT_SECS`, default 20, partial results marked). Skips `ws_root` (Task 2 covers it)
  and the build cache. Stores it as `~/.scholion/footprint/<session_id>.json`. `AV_FOOTPRINT=off`
  disables all of this.
- **Footprint diff at Stop.** `stop` rescans, diffs against the snapshot, and when a session added
  more than `AV_FOOTPRINT_WARN_MB` (default 512) outside any task workspace: once per session,
  blocks with `av: this session added 24.3 GB outside the task workspace: ~/dogfood/target-a
  (24.1 GB), /tmp/x (0.2 GB). Build inside the workspace (reclaimed on merge) or the repo cache
  (<path>), move it to $AV_SCRATCH, or remove it; then record where anything kept lives.`
  (Same once-per-session mechanism as the unrecorded-changes check.) It then posts the diff to
  `POST /v1/footprint` `{session, agent, task?, added: [{path, bytes}]}`; the daemon keeps it in
  memory (`AppState.footprints`, bounded) and `GET /v1/disk` shows it under `outside`. The daemon
  never deletes outside `ws_root`; removal of those paths is the human's or the agent's.

- [ ] **Step 1: Write the failing tests:**
  - `footprint.rs` unit tests (heavy-dir detection, diff, time cap);
  - hook tests in `crates/scholion-cli` beside the existing hook tests: snapshot written on
    session-start with a fake daemon; stop blocks once with the message when a `target` dir grew
    past the threshold under a temp `HOME`; `AV_FOOTPRINT=off` is silent; `AV_SCRATCH` line
    appended to a temp `CLAUDE_ENV_FILE`;
  - daemon test for `/v1/footprint` → `/v1/disk.outside`.
- [ ] **Step 2: Run** `cargo test -p scholion-cli` and `cargo test -p scholion-daemon --test disk`.
  Expected: FAIL.
- [ ] **Step 3: Implement** `footprint.rs`, the hook changes and `/v1/footprint`.
- [ ] **Step 4: Run** the same. Expected: PASS.
- [ ] **Step 5: Commit**: `hooks: a session accounts for what it leaves on disk`.

### Task 5: CLI and docs

**Files:**
- Modify: `crates/scholion-cli/src/main.rs` (`Cmd`, near `Repo`/`Reindex`), `local.rs`
- Create: `crates/scholion-cli/src/disk.rs`
- Modify: `docs/reference.md`, `README.md`, `CHANGELOG.md`, `CLAUDE.md`

**Interfaces:**
- `av disk [--json]`: per repo: workspaces (slug, status, size, age, `reclaimable`), scratch,
  build cache, store; then `outside` (what sessions added, by agent/session/task), Postgres dir,
  totals. Human sizes.
- `av gc [--dry-run] [--retain SECS] [--build-cache] [--json]`: prints what was removed and
  `freed`.
- `av up` (`local.rs`): rotate `$AV_HOME/daemon.log` to `daemon.log.1` over 50 MB.
- Docs: `docs/reference.md` section "Disk: what agents create and what av reclaims" (table above,
  `AV_WS_RETAIN_SECS`, `AV_BUILD_CACHE`, `AV_FOOTPRINT*`, `AV_SCRATCH`, `av disk`, `av gc`, cargo
  precedence, the parallel-build trade-off) and env-table rows; `README.md` one line;
  `CHANGELOG.md` Unreleased; `CLAUDE.md` architecture bullets (`disk.rs`, `build_cache.rs`,
  `footprint.rs`, the env vars, "workspaces of merged/abandoned tasks are removed after
  `AV_WS_RETAIN_SECS`, restored by undo/revert"). Agent-facing text says `av`.
  `integrations/claude-code/install.sh` unchanged (same four hooks). No new MCP tools
  (`tests/mcp.rs` size guard).

- [ ] **Step 1: Write the failing CLI tests** for `av disk`, `av gc` and the log rotation.
- [ ] **Step 2: Run** `cargo test -p scholion-cli`. Expected: FAIL.
- [ ] **Step 3: Implement** the commands and the rotation.
- [ ] **Step 4: Update the docs** listed above.
- [ ] **Step 5: Run the full gate, then commit**: `av disk and av gc; docs (Plan 11)`.

### Task 6: bench stops hoarding

**Files:**
- Modify: `crates/scholion-bench/src/runner.rs` (`run_session`, ~640-700), `main.rs`
- Modify: `bench/README.md`

**Interfaces:**
- `--keep-work` (default off). After `verify`: baseline arms remove `<run dir>/ws`; av arms
  remove the scrubbed clone under `import_dir` and abandon the task as its owner so the bench
  daemon's retain policy reclaims the workspace. Logs, `events.jsonl`, `outcome.json` stay.
- `bench/README.md:228-230`: replace "plan on about 1 GB of disk per session".

- [ ] **Step 1: Write the failing test** with the fake `claude`: `ws` gone after a run unless
  `--keep-work`.
- [ ] **Step 2: Run** `cargo test -p scholion-bench`. Expected: FAIL.
- [ ] **Step 3: Implement** `--keep-work` and the clean-up.
- [ ] **Step 4: Update** `bench/README.md`.
- [ ] **Step 5: Run the full gate, then commit**: `bench: remove each session's work unless --keep-work`.

### Later, only if `av disk` shows it matters (not in this PR)

- Store gc via jj-lib `GitBackend::gc` (needs `git` in the daemon's environment and a decision on
  "the daemon never runs repository commands").
- `MAX_NEW_FILE_SIZE` (jj.rs:53, 256 MiB vs jj's 1 MiB) as `AV_MAX_FILE_MB`.
- Sparse checkouts (HANDOFF.md item 5).

## Verification

1. `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
   green (embedded Postgres, no Docker).
2. End to end on a scratch daemon (`av up` with scratch `AV_HOME`, `AV_WS_RETAIN_SECS=0`,
   `AV_WS_ROOT=<scratch>/ws`):
   - `av init` this repo → `<ws>/scholion/.cargo/config.toml` has the marker;
   - `av task start`; in a Claude Code session there, `echo $AV_SCRATCH` resolves and
     `cargo test -p scholion-core` fills `<ws>/scholion/.cache/cargo-target`, no `target/` in the
     workspace;
   - create a 1 GB file under `~/junk/target` and stop → the Stop hook names it once;
   - `av disk` lists workspace, cache and `outside`;
   - record, submit, review, merge → `av disk` shows the workspace and scratch gone, cache kept;
   - `av undo` → workspace back with the edit;
   - `av gc --build-cache` frees the cache; `av down`.
3. `scripts/smoke.sh` against the compose stack still passes (removal across the bind mount, host
   paths in reports).
4. `scholion-bench run bench/tasks-tallyho --reps 1 --arms baseline` with the fake claude (no
   `--confirm`): `runs/<id>/<task>-baseline-r0/ws` absent afterwards, present with `--keep-work`.
