# agentvcs Plan 1: Foundation & First Context Pack — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A dockerized daemon where an agent can register a repo, start a task in its own jj workspace, checkpoint, record decisions/dead ends, and fetch a token-budgeted, structurally ranked context pack over HTTP and the `av` CLI.

**Architecture:** Cargo workspace of small crates behind traits defined in `av-core`. Only `av-vcs` touches jj-lib; only `av-store` touches Postgres; only `av-index` touches tree-sitter. `av-pack` is pure logic over those traits (no LLM, no network). `av-daemon` wires them behind axum; `av-cli` is a thin HTTP client.

**Tech Stack:** Rust (stable via `rust-toolchain.toml`), jj-lib (exact version pinned in Task 2), sqlx + PostgreSQL 16, tree-sitter (+ `tree-sitter-rust`, `-typescript`, `-python`), `tiktoken-rs`, axum, clap, testcontainers (`testcontainers-modules` postgres), Docker Compose.

**Spec:** `docs/superpowers/specs/2026-10-18-agentvcs-design.md`

## Plan series (this spec is split; each plan ships working software)

1. **This plan** — engine, intent store, code index, structural packer, HTTP + CLI, Docker (daemon + Postgres).
2. Semantic layer — Qdrant + text-embeddings-inference, outbox worker, semantic candidates, `degraded: semantic`, `av reindex`.
3. Collaboration — MCP server, `handoff`/`claim_task`/`submit`/`review`/`merge`/`undo`, record-discipline refusals, Claude Code hooks, git backup push.
4. Benchmark harness — replay ~10 tasks (`bench-repo`, `second-repo`), calibrate token factors.

## Global Constraints

- Rust stable; `cargo fmt --check` and `cargo clippy -- -D warnings` clean before every commit.
- Daemon HTTP port `7420`; Postgres 16; all components free/OSS; **no SQLite anywhere**.
- No LLM or network call inside `av-pack` (deterministic; target < 300 ms warm).
- Task status enum: `open | handed_off | in_review | conflicted | merged | abandoned`. Record kind enum: `intent | decision | attempt | note | handoff | review`. Record `summary` ≤ 200 chars (reject longer). Records are append-only; supersession hides, never deletes.
- Budget shares: map 10% · task/intent 10% · history 20% · code 50% · conventions+live work 10%, with spillover. Token estimate = `tiktoken-rs` cl100k count × per-family factor (default 1.0, configurable) with a 10% safety margin: the pack's estimated tokens never exceed `budget_tokens`.
- Item tiers: `Handle` (one line + id) < `Summary` < `Full`.
- Daemon runs as host UID/GID (`user:` in compose); returned `workspace_path` is the **host** path (`AV_WS_HOST_ROOT`), not the container path.
- Commits end with the trailer `Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>`.

## Review Focus

Inputs the spec implies but no feature task names; each is pinned by a test in the owning task.

1. Pack for a task with **no changes / empty repo** → still returns Task + Map sections, no panic (Task 7).
2. **Unparseable, binary, non-UTF8 or huge (>1 MB) files** → indexer skips with a warning; `checkpoint` still succeeds (Task 4).
3. **Budget smaller than the minimum** (e.g. 150 tokens) → handles only, never exceeds budget, `omitted` lists the rest (Task 6).
4. **Renamed/removed symbol** → anchored record still returned via its path, flagged `anchor_stale` (Task 5).
5. **Two agents checkpointing the same repo concurrently**, and **two tasks with the same title** → no corrupted state; slugs are unique (Tasks 2, 3).
6. **Ignored directories** (`node_modules`, `.venv`, `target`, anything in `.gitignore`) → never indexed (Task 4).

## File Structure

```
Cargo.toml  rust-toolchain.toml
crates/av-core/    src/{lib,types,traits,error}.rs
crates/av-vcs/     src/{lib,jj.rs}   tests/vcs.rs
crates/av-store/   migrations/0001_init.sql  src/{lib,pg.rs}  tests/store.rs
crates/av-index/   src/{lib,extract.rs,graph.rs,store.rs}  tests/{extract,graph}.rs
crates/av-pack/    src/{lib,tokens.rs,budget.rs,score.rs,assemble.rs,render.rs,ledger.rs}  tests/
crates/av-daemon/  src/{main,api.rs,wiring.rs}   tests/api.rs
crates/av-cli/     src/main.rs
deploy/Dockerfile  deploy/docker-compose.yml  scripts/smoke.sh
```

---

### Task 1: Prerequisites, workspace scaffold, `av-core`

**Files:** Create `Cargo.toml`, `rust-toolchain.toml`, `crates/av-core/**`. Test: `crates/av-core/src/types.rs` (unit).

**Interfaces — Produces** (all later tasks depend on these exact names):

```rust
pub struct TaskId(pub uuid::Uuid);  pub struct RecordId(pub uuid::Uuid);
pub struct SymbolId(pub i64);       pub struct ChangeId(pub String);
pub enum TaskStatus { Open, HandedOff, InReview, Conflicted, Merged, Abandoned }
pub enum RecordKind { Intent, Decision, Attempt, Note, Handoff, Review }
pub struct Anchor { pub path: String, pub symbol: Option<String> }
pub struct Task { id, repo: String, slug: String, title, goal, status: TaskStatus, owner_agent: String,
                  base_change: ChangeId, head_change: ChangeId, workspace_path: PathBuf }
pub struct NewRecord { task_id, kind, summary: String, body: String, change_id: Option<ChangeId>,
                       supersedes: Option<RecordId>, author_agent: String, anchors: Vec<Anchor> }
pub struct Record { id, created_at: DateTime<Utc>, /* NewRecord fields */ }
pub enum AgentFamily { Claude, Other }
pub enum Tier { Handle, Summary, Full }                         // ordered Handle < Summary < Full
pub struct NewTask { repo: String, title: String, goal: String, owner_agent: String } // slug generated by the store;
                     // base_change/workspace_path are empty until `set_workspace` (workspace needs the slug first)
pub struct WorkspaceInfo { pub path: PathBuf, pub base_change: ChangeId }
pub struct ConventionId(pub i64);
pub struct Convention { pub id: ConventionId, pub repo: String, pub scope_glob: String, pub text: String }
```

Traits (async fn in traits; `Result<T> = std::result::Result<T, AvError>`):

```rust
pub trait Vcs {
  async fn init_repo(&self, repo: &str, git_source: &Path) -> Result<ChangeId>;   // returns main@ change
  async fn create_workspace(&self, repo: &str, slug: &str) -> Result<WorkspaceInfo>; // {path, base_change}
  async fn snapshot(&self, repo: &str, slug: &str) -> Result<ChangeId>;
  async fn changed_paths(&self, repo: &str, base: &ChangeId, head: &ChangeId) -> Result<Vec<String>>;
  async fn list_files(&self, repo: &str, change: &ChangeId) -> Result<Vec<String>>;  // gitignore-filtered
  async fn read_file(&self, repo: &str, change: &ChangeId, path: &str) -> Result<Vec<u8>>;
}
pub trait IntentStore {
  async fn create_task(&self, t: NewTask) -> Result<Task>;       // generates unique slug from title
  async fn get_task(&self, id: TaskId) -> Result<Task>;
  async fn set_workspace(&self, id: TaskId, path: &Path, base: &ChangeId) -> Result<()>; // also sets head = base
  async fn set_head(&self, id: TaskId, head: &ChangeId, touched: &[String]) -> Result<()>;
  async fn open_tasks_touching(&self, repo: &str, paths: &[String], exclude: TaskId) -> Result<Vec<Task>>;
  async fn add_record(&self, r: NewRecord) -> Result<Record>;    // rejects summary > 200 chars
  async fn task_records(&self, id: TaskId) -> Result<Vec<Record>>;
  async fn records_anchored_to(&self, repo: &str, paths: &[String], symbols: &[String]) -> Result<Vec<Record>>; // excludes superseded
  async fn get_record(&self, id: RecordId) -> Result<Record>;
  async fn add_convention(&self, repo: &str, scope_glob: &str, text: &str) -> Result<ConventionId>;
  async fn conventions_for(&self, repo: &str, paths: &[String]) -> Result<Vec<Convention>>;
  async fn ledger_get(&self, session: &str) -> Result<HashMap<String, Tier>>;
  async fn ledger_put(&self, session: &str, sent: &[(String, Tier)]) -> Result<()>;
}
```

(`CodeIndex` trait is defined in Task 4/5 inside `av-index`, since only `av-pack` and `av-daemon` consume it.)

- [ ] **Step 1: Verify prerequisites.** Run `cargo --version` and `docker compose version`. Neither is installed on this machine as of 2026-10-18: install Rust with rustup (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`); **Docker must be installed by the user** (Docker Desktop with WSL integration, or docker-ce; needs sudo). Expected: both commands print versions. Stop and ask the user if Docker is missing.
- [ ] **Step 2: Failing test** `types::tests::summary_over_200_chars_rejected` — `NewRecord::validate()` returns `Err(AvError::SummaryTooLong)` for a 201-char summary and `Ok` for exactly 200; and `task_status_roundtrips_as_snake_case` (`"handed_off"`).
- [ ] **Step 3:** Create the workspace, `rust-toolchain.toml` (stable, components `rustfmt`, `clippy`), and `av-core` with the types/traits above; `serde` derives; `AvError` via `thiserror`.
- [ ] **Step 4:** `cargo test -p av-core` → PASS. `cargo clippy --workspace -- -D warnings` → clean.
- [ ] **Step 5:** Commit `feat(core): workspace scaffold and domain types`.

### Task 2: `av-vcs` — jj-lib engine

**Files:** Create `crates/av-vcs/src/{lib.rs,jj.rs}`; Test `crates/av-vcs/tests/vcs.rs`.

**Interfaces — Consumes:** `Vcs` trait (Task 1). **Produces:** `JjVcs::new(store_root: PathBuf, ws_root: PathBuf) -> JjVcs` implementing `Vcs`; workspaces live at `{ws_root}/{repo}/{slug}`.

- [ ] **Step 1:** Choose the newest published `jj-lib` release, pin it with `=x.y.z` in `Cargo.toml`, and record the version plus the doc link in a comment atop `jj.rs`. jj-lib's API is unstable and not in this plan; read its docs for that version. Nothing outside this crate may import `jj_lib`.
- [ ] **Step 2: Failing tests** in `tests/vcs.rs`, using a fixture git repo created with `git init` in a tempdir (3 files, one `.gitignore`d `node_modules/x.js`):
  - `init_repo_imports_git_source_and_lists_files` — `list_files` returns the 3 files, **not** `node_modules/x.js`.
  - `create_workspace_makes_isolated_dir_per_slug` — two slugs yield two directories; writing in one is invisible in the other.
  - `snapshot_captures_edits_and_changed_paths_lists_them` — edit `a.py`, add `b.py`; `changed_paths(base, head)` == `["a.py","b.py"]`; `read_file(head,"a.py")` returns the new bytes and `read_file(base,"a.py")` the old.
  - `concurrent_snapshots_on_same_repo_do_not_corrupt` — `tokio::join!` of 8 snapshots across 4 workspaces; all succeed and every `head` resolves.
  - `non_utf8_and_binary_files_snapshot_fine`.
- [ ] **Step 3: Run** `cargo test -p av-vcs` → FAIL (not implemented).
- [ ] **Step 4:** Implement `JjVcs`. Use a per-repo `tokio::sync::Mutex` around operations that write the op log (jj also resolves concurrent ops; the mutex is a belt-and-braces guard). Honor `.gitignore` in `list_files`.
- [ ] **Step 5:** `cargo test -p av-vcs` → PASS. Commit `feat(vcs): jj-lib engine`.

### Task 3: `av-store` — Postgres intent store

**Files:** Create `crates/av-store/migrations/0001_init.sql`, `src/{lib,pg}.rs`; Test `tests/store.rs` (testcontainers Postgres 16).

**Interfaces — Consumes:** `IntentStore` (Task 1). **Produces:** `PgStore::connect(url: &str) -> Result<PgStore>` (runs migrations) implementing `IntentStore`.

Schema tables: `tasks`, `task_touched(task_id, path)`, `records`, `anchors(record_id, path, symbol)`, `conventions`, `ledger(session, item_id, tier)`. `records.supersedes` → self-FK; a partial index hides superseded rows in `records_anchored_to`. Full-text `tsvector` column on `records(summary||body)` (used by Plan 2's degraded mode; create now).

- [ ] **Step 1: Failing tests:**
  - `slug_is_unique_for_same_title` — two `create_task` calls with title "Fix login" give `fix-login` and `fix-login-2`.
  - `records_roundtrip_with_anchors`.
  - `superseded_record_hidden_from_anchor_query_but_still_in_task_records`.
  - `summary_201_chars_rejected_by_store`.
  - `open_tasks_touching_excludes_self_and_non_open` — a `merged` task touching `a.py` isn't returned.
  - `conventions_for_matches_glob` — scope `src/**/*.py` matches `src/a/b.py`, not `docs/x.py`.
  - `ledger_put_then_get_keeps_highest_tier`.
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement with `sqlx` (compile-time queries off; use `query_as` + runtime checks so no live DB is needed at build). Glob via `globset`. **Step 4:** Run → PASS. **Step 5:** Commit `feat(store): postgres intent store`.

### Task 4: `av-index` — symbol extraction (tree-sitter)

**Files:** Create `crates/av-index/src/{lib,extract}.rs`; Test `tests/extract.rs`; fixtures under `tests/fixtures/{py,ts,rs}/`.

**Interfaces — Produces:**

```rust
pub struct Symbol { pub path: String, pub qname: String /* "Class.method" */, pub kind: SymbolKind /* Function|Method|Type */,
                    pub signature: String, pub start_line: u32, pub end_line: u32, pub refs: Vec<String> /* identifiers used in body */ }
pub fn extract(path: &str, bytes: &[u8]) -> Result<Vec<Symbol>>;   // language by extension .py .ts .tsx .rs; others → Ok(vec![])
```

- [ ] **Step 1: Failing tests:** `extracts_python_class_method_and_function` (qnames `Greeter`, `Greeter.hello`, `main`; `Greeter.hello.refs` contains `format_name` when called); `extracts_typescript_interface_class_arrow_function`; `extracts_rust_struct_impl_methods_trait` (`impl Foo { fn bar }` → `Foo.bar`); `unparseable_file_returns_empty_not_error`; `binary_and_non_utf8_return_empty`; `file_over_1mb_is_skipped`; `unknown_extension_returns_empty`.
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement using tree-sitter queries per language; `refs` = distinct identifier nodes inside the body excluding the symbol's own name. Tolerate parse errors (tree-sitter still yields a tree): skip a file only when more than 30% of nodes are `ERROR`. **Step 4:** Run → PASS. **Step 5:** Commit `feat(index): tree-sitter symbol extraction`.

### Task 5: `av-index` — persistence, ref graph, anchor resolution

**Files:** Create `src/{graph,store}.rs`; add migration `0002_index.sql` in `av-store/migrations/` (tables `symbols`, `symbol_refs`, `parse_cache(content_hash, symbols_json)`); Test `tests/graph.rs`.

**Interfaces — Produces:**

```rust
pub trait CodeIndex {
  async fn index_paths(&self, repo: &str, scope: &str /* "main" or task slug */, files: Vec<(String, Vec<u8>)>) -> Result<IndexStats>;
  async fn remove_paths(&self, repo: &str, scope: &str, paths: &[String]) -> Result<()>;
  async fn symbols_in(&self, repo: &str, scope: &str, path: &str) -> Result<Vec<IndexedSymbol>>;   // IndexedSymbol { id: SymbolId, ..Symbol }
  async fn resolve(&self, repo: &str, scope: &str, a: &Anchor) -> Result<AnchorResolution>;        // Symbol(SymbolId) | PathOnly(path) | Stale
  async fn neighbors(&self, repo: &str, scope: &str, seeds: &[SymbolId], hops: u8) -> Result<Vec<(SymbolId, u8)>>; // 1..=2 hops, both directions
}
pub struct PgIndex; impl PgIndex { pub fn new(pool: PgPool) -> Self }
```

Reference resolution rule: an identifier in `refs` links to every symbol in the same `(repo, scope)` whose last `qname` segment equals it; if k > 1 candidates, each edge gets weight 1/k. Parse results are cached by blake3 content hash, so identical content across scopes is parsed once.

- [ ] **Step 1: Failing tests:** `unchanged_content_is_not_reparsed` (second `index_paths` reports `parsed == 0`); `neighbors_two_hops_both_directions` (A→B→C, seeds `[B]` hops=1 returns A,C; seeds `[A]` hops=2 returns B(1),C(2)); `ambiguous_name_links_all_candidates_with_split_weight`; `resolve_returns_stale_when_symbol_removed_and_path_gone`; `resolve_returns_pathonly_when_symbol_renamed_but_path_exists` (**Review Focus 4**); `scopes_are_isolated` (task-scope edit invisible in `main`); `ignored_dirs_never_indexed` — the caller-supplied file list excludes `node_modules`, and `index_paths` itself also drops paths whose any segment is `node_modules|.venv|target|.git` (**Review Focus 6**).
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement `PgIndex`; neighbor query via a recursive CTE capped at 2 hops. **Step 4:** Run → PASS. **Step 5:** Commit `feat(index): persistence, ref graph, anchor resolution`.

### Task 6: `av-pack` — tokens and budget allocation (pure)

**Files:** Create `crates/av-pack/src/{tokens,budget}.rs`; Test `crates/av-pack/tests/budget.rs`.

**Interfaces — Produces:**

```rust
pub fn estimate_tokens(text: &str, family: AgentFamily) -> u32;         // cl100k × factor, rounded up
pub enum Section { Task, Map, History, Code, ConventionsLive }
pub struct Item { pub id: String, pub section: Section, pub score: f32, pub tiers: [String; 3] /* handle, summary, full */ }
pub struct Allocation { pub chosen: Vec<(Item, Tier)>, pub omitted: Vec<String> /* handle lines */, pub used: u32 }
pub fn allocate(items: Vec<Item>, budget: u32, family: AgentFamily) -> Allocation;
```

Algorithm: effective budget = `floor(budget × 0.9)`; split by the Global-Constraints shares; fill sections in order Task, Map, History, Code, ConventionsLive, rolling unused tokens forward; within a section sort by `score` desc and give each item the richest tier that still fits; items that cannot fit even as a handle go to `omitted` (list capped at 20 lines, counted against the budget).

- [ ] **Step 1: Failing tests:** `shares_split_ten_ten_twenty_fifty_ten`; `unused_share_spills_to_next_section`; `never_exceeds_budget` (property test over random items, budgets 50..8000: `used <= budget`); `tiny_budget_returns_handles_only_and_lists_omitted` (budget 150, 40 items → no `Summary`/`Full` chosen; `omitted` non-empty; `used <= 150`) (**Review Focus 3**); `higher_score_gets_richer_tier_first`; `empty_items_ok`.
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement. **Step 4:** Run → PASS. **Step 5:** Commit `feat(pack): token estimate and budget allocation`.

### Task 7: `av-pack` — scoring, assembly, rendering

**Files:** Create `src/{score,assemble,render,ledger}.rs`, `src/lib.rs`; Test `tests/pack.rs` + `tests/golden/*.md`.

**Interfaces — Consumes:** `IntentStore`, `Vcs`, `CodeIndex`. **Produces:**

```rust
pub struct PackRequest { pub task_id: TaskId, pub budget_tokens: u32, pub focus: Option<String>, pub session: String, pub family: AgentFamily }
pub struct Pack { pub markdown: String, pub tokens: u32, pub omitted: Vec<String>, pub degraded: Vec<String> }
pub struct Packer<V, S, I> { /* ctor: Packer::new(vcs, store, index) */ }
impl Packer { pub async fn build(&self, req: PackRequest) -> Result<Pack>;
              pub async fn expand(&self, item_id: &str) -> Result<String>; }   // Full tier; ids "rec:<uuid>" | "sym:<i64>" | "conv:<id>"
```

Rules: seeds = symbols in `changed_paths(base, head)` ∪ symbols matching `focus` (substring on qname or path; semantic matching arrives in Plan 2). Candidates: structural `neighbors(seeds, 2)`; `records_anchored_to` (seed paths + qnames) plus the task's own records; `conventions_for(touched paths)`; `open_tasks_touching` → one-line conflict warnings. Score (Plan 1 weights) = `0.5·(1/(1+hops)) + 0.2·recency(2^(−age_days/30)) + 0.3·kind_priority`, kind priority attempt 1.0 · review 0.9 · decision 0.8 · intent 0.7 · handoff 0.6 · note 0.4; records anchored directly on a seed symbol add +1.0. Ledger: items already sent to this `session` at ≥ the chosen tier render as `↩ sent earlier: <handle>` (costs handle tokens only); after building, `ledger_put` what was sent. Output sections in order `## Task`, `## Map`, `## Decisions & dead ends`, `## Code`, `## Conventions`, `## Live work`, then a footer `tokens: N/budget · omitted: …`. Stale anchors render with `(anchor_stale)`.

- [ ] **Step 1: Failing tests** (fixture: in-memory fake `Vcs`/`CodeIndex` + testcontainers `PgStore`):
  - `empty_task_pack_has_task_and_map_and_no_panic` (**Review Focus 1**).
  - `attempt_on_seed_symbol_outranks_unrelated_note`.
  - `superseded_record_absent_from_pack`.
  - `second_pack_in_same_session_references_instead_of_resending` (second `markdown` shorter; contains `↩ sent earlier`).
  - `live_work_warning_lists_other_open_task_touching_same_path`.
  - `stale_anchor_record_still_returned_with_flag` (**Review Focus 4**).
  - `convention_only_when_scope_matches`.
  - `golden_pack_for_fixture_task` — compare against `tests/golden/basic.md` (generate once, review by eye, commit).
  - `warm_build_under_300ms` (`#[ignore]` by default; run with `--ignored` in the smoke step).
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement; fetch candidate sources concurrently with `tokio::join!`. **Step 4:** Run → PASS. **Step 5:** Commit `feat(pack): scoring, assembly, rendering, ledger`.

### Task 8: `av-daemon` — HTTP API and wiring

**Files:** Create `crates/av-daemon/src/{main,api,wiring}.rs`; Test `tests/api.rs` (axum `oneshot` + testcontainers Postgres + `JjVcs` on a tempdir).

**Interfaces — Produces** (JSON; this is the contract `av-cli` and Plan 3's MCP wrap):

| Method / path | Body → Response |
|---|---|
| `POST /v1/repos` | `{name, git_source}` → `{name, main_change}`; indexes `main` scope |
| `POST /v1/tasks` | `{repo, title, goal, agent, session}` → `{task: Task, pack: Pack}`; `create_task` → `create_workspace(repo, slug)` → `set_workspace`; records an `intent` record from `goal` |
| `POST /v1/tasks/{id}/checkpoint` | `{}` → `{head_change, changed: [path]}`; `snapshot` → `changed_paths` → `index_paths` (task scope) → `set_head`. Unindexable files are skipped with a warning in the response, never an error |
| `POST /v1/tasks/{id}/records` | `NewRecord` fields → `Record` (400 on summary > 200) |
| `GET /v1/tasks/{id}/context?budget=&focus=&session=&family=` | → `Pack` |
| `GET /v1/items/{item_id}` | → `{full: String}` |
| `GET /v1/tasks/{id}` | → `{task, record_count, changed}` |

Config via env: `AV_DATABASE_URL`, `AV_STORE_ROOT`, `AV_WS_ROOT` (container path), `AV_WS_HOST_ROOT` (reported to clients), `AV_LISTEN` (default `0.0.0.0:7420`).

- [ ] **Step 1: Failing tests:** `post_tasks_returns_workspace_pack_and_host_path` (path starts with `AV_WS_HOST_ROOT`); `checkpoint_then_context_reflects_edit` (edit a file, checkpoint, `GET context` mentions the edited symbol); `record_then_context_contains_attempt`; `record_with_201_char_summary_is_400`; `two_tasks_same_title_get_distinct_workspaces`; `checkpoint_with_binary_file_succeeds_with_warning` (**Review Focus 2**); `concurrent_checkpoints_from_two_tasks_succeed` (**Review Focus 5**); `unknown_task_is_404`.
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement handlers and `wiring.rs` (build `JjVcs`, `PgStore`, `PgIndex`, `Packer`); errors map `AvError` → status codes. **Step 4:** Run → PASS. **Step 5:** Commit `feat(daemon): http api`.

### Task 9: `av-cli`

**Files:** Create `crates/av-cli/src/main.rs`; Test `crates/av-cli/tests/cli.rs` (spawns a test daemon on an ephemeral port).

**Interfaces — Consumes:** Task 8 HTTP contract. **Produces:** binary `av` with `repo add <name> <git_source>`, `task start --repo --title --goal`, `checkpoint [task]`, `record --kind --summary [--body] [--anchor path[:symbol]]…`, `context [--budget 4000] [--focus Q]`, `status`. Server from `AV_URL` (default `http://localhost:7420`). The current task is inferred from a `.av-task` file the CLI writes into the workspace root on `task start`; `--task` overrides.

- [ ] **Step 1: Failing tests:** `task_start_writes_av_task_file_and_prints_pack`; `context_uses_task_from_dotfile_in_cwd`; `record_with_anchor_flag_parses_path_and_symbol` (`src/a.py:Greeter.hello`); `unreachable_server_exits_nonzero_with_clear_message`.
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement with `clap` + `reqwest`. **Step 4:** Run → PASS. **Step 5:** Commit `feat(cli): av binary`.

### Task 10: Docker deployment and smoke test

**Files:** Create `deploy/Dockerfile` (multi-stage, release build of `av-daemon`), `deploy/docker-compose.yml`, `scripts/smoke.sh`, `README.md` (run instructions only).

Compose: services `daemon` (`user: "${UID}:${GID}"`, ports `7420:7420`, volumes `av-store:/store`, `${AV_WS_ROOT:-$HOME/ws}:/ws`, `${AV_IMPORT_ROOT:-$HOME/projects}:/import:ro`, env per Task 8 with `AV_WS_HOST_ROOT=${AV_WS_ROOT:-$HOME/ws}`) and `postgres:16` (volume `av-pg`, healthcheck; daemon `depends_on` healthy). Qdrant/TEI are added by Plan 2.

- [ ] **Step 1: Write `scripts/smoke.sh`** (the failing check): `docker compose up -d --build` → `av repo add bench-repo /import/bench-repo` → `av task start …` → append a function to a file in the returned host workspace path → `av checkpoint` → `av record --kind attempt …` → `av context` and assert output contains the new function name and the attempt summary; then assert the new file under `$AV_WS_ROOT` is owned by the current user (`stat -c %u` == `id -u`).
- [ ] **Step 2:** Run `scripts/smoke.sh` before the compose files exist → FAIL.
- [ ] **Step 3:** Write the Dockerfile and compose file.
- [ ] **Step 4:** Run `scripts/smoke.sh` → exits 0. Also run `cargo test -p av-pack -- --ignored warm_build_under_300ms` against the stack.
- [ ] **Step 5:** Commit `feat(deploy): docker compose stack and smoke test`.

---

## Self-review notes

- **Spec coverage (Plan 1 scope):** VCS engine → T2; intent store/records/anchors/conventions/ledger → T3; code index → T4–5; packer (seeds, candidates, score, budget, tiers, ledger, output format) → T6–7; HTTP + CLI → T8–9; Docker + UID + bind mounts → T10. Deferred to later plans by design: Qdrant/embeddings/outbox/degraded mode (Plan 2); MCP, handoff/PR/review/merge/undo, record-discipline refusals, hooks, git backup (Plan 3); benchmark and token-factor calibration (Plan 4).
- **Known risks to confirm in Task 2:** jj-lib API churn (mitigated by pinning and isolating it in one crate); agents on the host see a workspace with no `.git`, so tools that expect a git repo may behave differently (revisit in Plan 3 hooks).
