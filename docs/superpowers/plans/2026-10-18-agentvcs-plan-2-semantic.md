# agentvcs Plan 2: Semantic Layer — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a derived Qdrant index and a local CPU embedding server so context packs can rank records, code chunks, and conventions by semantic similarity, degrade to Postgres full-text when that layer is down, and be rebuilt with `av reindex`.

**Architecture:** Postgres stays the source of truth. `add_record`, `add_convention`, and `index_paths` write an `outbox` row in the same transaction as the entity. An `av-daemon` worker drains that outbox, embeds via the text-embeddings-inference HTTP API, and upserts Qdrant. `av-pack` only calls a `Semantic` trait (no HTTP, no LLM). Qdrant or the embedder failing becomes `degraded: ["semantic"]` plus `records.tsv` search; the outbox retries until it catches up.

**Tech Stack:** Existing workspace (Rust stable, sqlx, PostgreSQL 16, axum, `postgresql_embedded` via `av-testutil`). New: `qdrant-client` 1.13 against the compose image `qdrant/qdrant:v1.13.6` (gRPC, `AV_QDRANT_URL=http://qdrant:6334`), `reqwest` 0.12 with `rustls-tls` (no native-tls), Hugging Face text-embeddings-inference `cpu-1.7.2` serving `BAAI/bge-small-en-v1.5` (384-d cosine).

**Spec:** `docs/superpowers/specs/2026-10-18-agentvcs-design.md` (semantic slice only; Plan 1 is already implemented in this worktree)

## Plan series

1. Foundation — done (`docs/superpowers/plans/2026-10-18-agentvcs-plan-1-foundation.md`).
2. **This plan** — Qdrant, TEI, outbox worker, semantic candidates, `degraded: semantic`, `av reindex`.
3. Collaboration — MCP, handoff/review/merge, hooks, git backup.
4. Benchmark harness.

## Global Constraints

- Rust pinned to 1.97.1 by `rust-toolchain.toml` (do not bump to stable; see the dev-environment note); `cargo fmt --check` and `cargo clippy --workspace -- -D warnings` clean before every commit.
- **Embed each piece of code once, not once per task.** A scope is a complete view of the repo, so a task's base import duplicates every symbol of `main`. Code chunks are enqueued for embedding only (a) when scope is `main`, and (b) when a task scope is re-indexed after a checkpoint, for the changed paths only. The task's initial base import never enqueues. Search results for code chunks whose `task_id` is another task's are dropped (unmerged work in progress of other tasks is not context).
- Migration numbering: this plan owns `0003_semantic.sql`; Plan 3's migration is `0004_flow.sql`.
- No SQLite. No LLM and no HTTP client inside `av-pack`. `av-pack` gains `tokio` (time) only. Embedding calls live behind `av_core::Semantic`.
- Pack stays deterministic and under 300 ms warm. Semantic search is capped at **150 ms** from the start of `gather`; on timeout or error the pack still returns.
- Collections, exact names: `records`, `conventions`, `code_chunks`. Distance **Cosine**. Payload keys on every point, JSON null when absent: `repo`, `task_id`, `record_id`, `symbol_id`, `convention_id`, `path`, `change_id`, `superseded`. `convention_id` is the one key beyond the spec list; a convention hit cannot be applied without it. Do not store `score` or `kind` in the payload (`kind` is the collection).
- Point ids: record = hyphenated UUID (`RecordId`'s `Display`), convention = `u64` of `ConventionId`, code chunk = `u64` of `SymbolId`.
- Search filter, both implementations: `repo` equals the query repo **and** `superseded = false`.
- The degraded marker is the string `semantic` inside `Pack.degraded` (one entry). That is the spec's `degraded: semantic`. Do not push `"degraded: semantic"`.
- Superseded records are **marked**, not deleted, in Qdrant. Symbol removal **deletes** the code-chunk point (the symbol id is gone).
- `Packer::new` and `CodeIndex::index_paths` keep their current signatures. New entry points are `Packer::with_semantic` and `CodeIndex::index_paths_at`.
- Embedder HTTP: `POST {AV_EMBED_URL}/embed` body `{"inputs":[...],"truncate":true}`; response a JSON array of arrays of numbers; `GET {AV_EMBED_URL}/health` is 200. Client timeout 10 s. `reqwest` features: `json`, `rustls-tls`, **default features off**.
- Optional `AV_EMBED_API_KEY`. A non-empty value is sent on `POST /embed` and `GET /health` as `Authorization: Bearer <key>`. Unset or blank sends no `Authorization` header. The key is not logged. A missing key does not disable the semantic layer; only a missing `AV_EMBED_URL` does.
- Embedder choice at daemon startup: if the `ollama` CLI is on `PATH` and `ollama list` shows a `qwen3-embedding` model (bare name, then `:latest`, then any other tag such as `qwen3-embedding:8b`), use Ollama `POST {AV_OLLAMA_URL:-http://127.0.0.1:11434}/api/embed` with that model and no API key. If the CLI is absent, use `AV_EMBED_URL` with `AV_EMBED_API_KEY`. `AV_EMBED_PROVIDER=api` forces the API path. No CLI and no URL leaves embedding off.
- When `AV_QDRANT_URL` or `AV_EMBED_URL` is unset, the daemon behaves as Plan 1 (no `semantic` degradation). When both are set, the daemon boots even if Qdrant or TEI is down.
- Score weights (sum to 1; seed bonus stays outside): `W_STRUCT = 0.35`, `W_SEM = 0.30`, `W_RECENCY = 0.15`, `W_KIND = 0.20`, `SEED_BONUS = 1.0`. Similarity is clamped to `0.0..=1.0` before weighting.
- Chunk text stored for embedding is at most `MAX_EMBED_CHARS = 2000` Unicode scalars.
- Commits end with the trailer `Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>`.
- `cargo test` must pass with no Docker and with `AV_TEST_QDRANT_URL` unset.

## Review Focus

These five inputs are the ones most likely to corrupt the index or the pack. Each is pinned by a test in the owning task.

1. **Embedder returns a vector whose length is not the probe dimension** (model swap, truncated JSON) → that row is not upserted, the collection dim is unchanged, the outbox row stays for retry (Tasks 4 and 6).
2. **Outbox upsert whose record (or symbol, or convention) is already gone** → drop the row, do not embed, do not create a Qdrant point (Task 6).
3. **Duplicate delivery** (crash after the Qdrant upsert, before the outbox delete; two workers) → one point per entity, and a redelivered upsert does **not** clear `superseded` once Postgres says the record is superseded (Task 6).
4. **Empty or whitespace query** → `Ok([])`, zero embedder calls, and the pack does not report `semantic` just because the goal and focus are blank (Tasks 5 and 7).
5. **Huge input** → a file over 1 MB produces no chunk upsert; a symbol body inside a legal file is stored truncated to 2000 chars and is what the worker embeds (Task 3).

## Cannot verify in this environment

No Docker here, and this plan was not executed (`cargo` was not run).

- **Task 5, last test only** (`qdrant_roundtrip_filters_repo_and_superseded`) needs a real Qdrant at `AV_TEST_QDRANT_URL`. Unset, the test returns without asserting. A green `cargo test` does not cover it.
- **Task 10 entirely** (image pull, TEI model download, healthchecks, `docker compose up`, `scripts/smoke.sh`). Do not claim the compose file was run.

Tasks 1–4, 6–9, and the in-memory half of Task 5 use embedded Postgres, an in-process HTTP server, and `MemoryIndex`.

## File Structure

```
crates/av-core/src/{types,traits,error}.rs          SemanticHit, OutboxOp, Semantic, AvError::Semantic
crates/av-store/migrations/0003_semantic.sql        outbox + code_chunks
crates/av-store/src/{pg,outbox}.rs                  enqueue, claim, fts, list for reindex
crates/av-index/src/store.rs                        chunk text + outbox inside index transactions
crates/av-semantic/src/{lib,embed,vector,search}.rs TEI client, Qdrant, memory, SemanticSearch
crates/av-semantic/tests/{embed,qdrant}.rs
crates/av-pack/src/{score,assemble,packer}.rs       semantic term, gather merge, degraded fts
crates/av-daemon/src/{worker,reindex,wiring,lib,api,main}.rs
crates/av-cli/src/main.rs                           av reindex
deploy/{docker-compose.yml,Dockerfile}  README.md
```

`av-semantic` depends on `av-core` only (plus reqwest and qdrant-client). `av-index` may depend on `av-store` for `enqueue` (no cycle: `av-store` does not depend on `av-index`). `av-pack` does not depend on `av-semantic`.

---

### Task 1: `Semantic` trait and outbox types

**Files:** Modify `crates/av-core/src/{types,traits,error}.rs`. Test: `types.rs` unit module.

**Interfaces — Produces:**

```rust
// types.rs — str_enum! so as_str/FromStr match the SQL CHECK values
str_enum!(SemanticKind { Record => "record", Convention => "convention", CodeChunk => "code_chunk" });
str_enum!(OutboxOp { Upsert => "upsert", Delete => "delete", Supersede => "supersede" });

pub struct SemanticHit {
    pub kind: SemanticKind, pub score: f32, pub repo: String,
    pub task_id: Option<TaskId>, pub record_id: Option<RecordId>,
    pub symbol_id: Option<SymbolId>, pub convention_id: Option<ConventionId>,
    pub path: Option<String>, pub change_id: Option<ChangeId>, pub superseded: bool,
}
pub struct NewOutbox {
    pub kind: SemanticKind, pub op: OutboxOp, pub entity_id: String, pub repo: String,
    pub task_id: Option<TaskId>, pub record_id: Option<RecordId>,
    pub symbol_id: Option<SymbolId>, pub convention_id: Option<ConventionId>,
    pub path: Option<String>, pub change_id: Option<ChangeId>,
    pub superseded: bool, pub text: Option<String>,
}
pub struct OutboxRow { pub id: i64, pub attempts: i32, pub last_error: Option<String>, pub row: NewOutbox }

// traits.rs
#[async_trait]
pub trait Semantic: Send + Sync {
    /// `limit` is per collection. Whitespace-only `query` → Ok(vec![]), no embed.
    async fn search(&self, repo: &str, query: &str, limit: usize) -> Result<Vec<SemanticHit>>;
}

// error.rs
#[error("semantic error: {0}")]
Semantic(String),
```

- [ ] **Step 1: Failing test** `semantic_hit_json_has_payload_keys` — a `SemanticHit` of kind `CodeChunk`, `symbol_id: SymbolId(7)`, `superseded: false` serializes with exactly the keys `kind, score, repo, task_id, record_id, symbol_id, convention_id, path, change_id, superseded`; `kind` is `"code_chunk"`; `symbol_id` is the JSON number `7`. And `outbox_op_and_kind_match_sql_checks`: `OutboxOp::Supersede.as_str() == "supersede"`, `SemanticKind::CodeChunk.as_str() == "code_chunk"`, both `FromStr` round-trip, `"drop"` fails `FromStr`.
- [ ] **Step 2:** `cargo test -p av-core --lib types::` → FAIL (types missing).
- [ ] **Step 3:** Add the types, the trait, and `AvError::Semantic`. `SemanticHit` derives `Debug, Clone, PartialEq, Serialize, Deserialize`. Tuple newtypes stay serde newtypes (a `TaskId` is a UUID string, a `SymbolId` is a number).
- [ ] **Step 4:** `cargo test -p av-core` → PASS. Clippy clean.
- [ ] **Step 5: Commit**

```bash
git add crates/av-core
git commit -m "$(cat <<'EOF'
feat(core): semantic trait and outbox types

Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>
EOF
)"
```

### Task 2: Outbox rows in the record and convention transactions, plus full-text

**Files:** Create `crates/av-store/migrations/0003_semantic.sql`, `crates/av-store/src/outbox.rs`. Modify `crates/av-store/src/{lib,pg}.rs`, `crates/av-core/src/traits.rs` (`IntentStore`). Test: `crates/av-store/tests/outbox.rs` (TestDb, no Docker).

**Interfaces — Consumes:** `NewOutbox`, `OutboxRow`, `OutboxOp`, `SemanticKind` (Task 1). **Produces:**

```rust
// IntentStore (new method; PgStore implements it)
async fn search_records_fts(&self, repo: &str, query: &str, limit: usize) -> Result<Vec<Record>>;

impl PgStore {
    pub async fn enqueue<'a, E: sqlx::Executor<'a, Database = sqlx::Postgres>>(executor: E, row: &NewOutbox) -> Result<()>;
    pub async fn claim_outbox(&self, limit: i64) -> Result<Vec<OutboxRow>>;   // id ascending
    pub async fn complete_outbox(&self, id: i64) -> Result<()>;                // DELETE
    pub async fn fail_outbox(&self, id: i64, error: &str) -> Result<()>;       // backoff, row kept
    pub async fn clear_outbox(&self, repo: Option<&str>) -> Result<u64>;
    pub async fn outbox_count(&self, repo: Option<&str>) -> Result<i64>;
    pub async fn list_records_for_embed(&self, repo: Option<&str>) -> Result<Vec<Record>>; // includes superseded
    pub async fn list_all_conventions(&self, repo: Option<&str>) -> Result<Vec<Convention>>;
}
```

`0003_semantic.sql`:

```sql
CREATE TABLE outbox (
    id            bigserial PRIMARY KEY,
    kind          text NOT NULL CHECK (kind IN ('record','convention','code_chunk')),
    op            text NOT NULL CHECK (op IN ('upsert','delete','supersede')),
    entity_id     text NOT NULL,
    repo          text NOT NULL,
    task_id       uuid,
    record_id     uuid,
    symbol_id     bigint,
    convention_id bigint,
    path          text,
    change_id     text,
    superseded    boolean NOT NULL DEFAULT false,
    text          text,
    attempts      integer NOT NULL DEFAULT 0,
    last_error    text,
    locked_until  timestamptz,
    created_at    timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX outbox_claim ON outbox (id);

CREATE TABLE code_chunks (
    symbol_id bigint PRIMARY KEY REFERENCES symbols(id) ON DELETE CASCADE,
    repo      text NOT NULL,
    scope     text NOT NULL,
    path      text NOT NULL,
    task_id   uuid,
    change_id text,
    text      text NOT NULL
);
CREATE INDEX code_chunks_repo ON code_chunks (repo);
```

(`symbols` already exists from `0002_index.sql`. `code_chunks` is written in Task 3; creating it here keeps one migration.)

Claim (one statement, so `FOR UPDATE` holds):

```sql
WITH picked AS (
  SELECT id FROM outbox
  WHERE locked_until IS NULL OR locked_until < now()
  ORDER BY id
  LIMIT $1
  FOR UPDATE SKIP LOCKED
)
UPDATE outbox o
SET attempts = o.attempts + 1,
    locked_until = now() + interval '60 seconds'
FROM picked
WHERE o.id = picked.id
RETURNING o.*;
```

`fail_outbox`: `locked_until = now() + LEAST(attempts, 60) * interval '5 seconds'` (cap 300 s), `last_error = $2`. Do not delete the row.

FTS (`search_records_fts`): whitespace-only query returns `Ok(vec![])` without a query. Otherwise `records.tsv @@ plainto_tsquery('english', $2)`, join `tasks` on repo, `NOT EXISTS` a row that `supersedes` this id, `ORDER BY ts_rank(...) DESC, r.id`, `LIMIT $3`.

Enqueue rules inside the **existing** `add_record` transaction (the `tx` that starts before the record insert) and inside a new transaction around `add_convention` (today it auto-commits on the pool):

- Record upsert: `entity_id` = hyphenated record UUID, `text` = `summary` plus, when `body` is non-empty, `"\n{body}"`, `superseded` false, `repo` from `tasks`, `task_id` set.
- When `supersedes` is `Some(old)`: also a `Supersede` row for `old` (`text` null, `superseded` true, same repo). Look up `old` in the same transaction; if it is missing the FK already fails the insert.
- Convention upsert: `entity_id` = decimal convention id, `convention_id` set, `text` = the convention text, `superseded` false, other ids null.

- [ ] **Step 1: Failing tests** in `tests/outbox.rs`:
  - `record_and_outbox_commit_or_roll_back_together` — install `BEFORE INSERT ON outbox` that `RAISE EXCEPTION`. `add_record` returns `Err`, `records` count is 0, drop the trigger, `add_record` then leaves exactly one `upsert` row whose `text` is `"sum\nbody"`.
  - `supersede_enqueues_upsert_and_supersede` — two rows; the supersede row's `entity_id` is the old UUID and `text` is null.
  - `convention_outbox_rolls_back_with_the_convention` — same trigger trick; after a successful add, one convention upsert.
  - `fts_finds_repo_record_and_hides_superseded_and_other_repo` — body `"exponential backoff for the embedder"` is found by `"embedder backoff"`; a superseded copy and another repo's copy are not; `""` and `"the"` return empty.
  - `claim_is_fifo_and_holds_the_row` — three rows; `claim_outbox(2)` returns the two smallest ids and commits `locked_until` 60 s ahead; a second `claim_outbox(2)` returns only the third. `fail_outbox` leaves the row (still one row, `last_error` set); `complete_outbox` removes it. Concurrent `SKIP LOCKED` is asserted in Task 6, where two claims overlap.
- [ ] **Step 2:** `cargo test -p av-store --test outbox` → FAIL.
- [ ] **Step 3:** Migration, `enqueue` / claim / fts, and the transaction edits in `add_record` and `add_convention`. `list_records_for_embed` returns superseded rows too (reindex must mark them) and selects `RECORD_COLS` (the same list `get_record` uses). `superseded_by` is a subquery, not a column; `SELECT *` will not compile against `Record`.
- [ ] **Step 4:** `cargo test -p av-store` → PASS (existing store tests still pass).
- [ ] **Step 5: Commit** `feat(store): outbox and record full-text search` with the trailer.

### Task 3: Code chunks and outbox rows inside `index_paths`

**Files:** Modify `crates/av-index/src/{lib,store}.rs`, `crates/av-index/Cargo.toml` (depend on `av-store`), `crates/av-daemon/src/indexing.rs` (pass the change id). Test: `crates/av-index/tests/chunks.rs`.

**Interfaces — Consumes:** `PgStore::enqueue` (Task 2). **Produces:**

```rust
pub const MAX_EMBED_CHARS: usize = 2000;
pub struct CodeChunk {
    pub symbol_id: SymbolId, pub repo: String, pub scope: String, pub path: String,
    pub task_id: Option<TaskId>, pub change_id: Option<ChangeId>, pub text: String,
}
// CodeIndex — index_paths stays; default it to index_paths_at(..., None, ...)
async fn index_paths_at(&self, repo: &str, scope: &str, change: Option<&ChangeId>, embed: bool, files: Vec<(String, Vec<u8>)>) -> Result<IndexStats>;
// `embed == false`: symbols and code_chunks rows are written but no outbox row is enqueued.
async fn list_code_chunks(&self, repo: Option<&str>) -> Result<Vec<CodeChunk>>;
async fn chunk_exists(&self, symbol_id: SymbolId) -> Result<bool>;
```

In the **same** transaction `index_paths` already opens: after each inserted symbol, `INSERT INTO code_chunks` and `enqueue` an upsert. Text is `{path}\n{qname}\n{signature}\n{body}` where `body` is the 1-based inclusive line slice `start_line..=end_line`. Truncate to 2000 scalars; if truncated, the last char is `…`. `task_id` = `tasks.id` for `(repo, slug=scope)` when `scope != "main"` and that task exists, else null. `change_id` from the argument. `entity_id` = decimal symbol id.

When symbols for a path are deleted (`index_paths` replace, or `remove_paths`), `enqueue` a `Delete` per returned id (text null) in that same transaction. `ON DELETE CASCADE` clears `code_chunks`. Unchanged content hash: no new outbox row. A file `extract` turns into zero symbols (over `MAX_FILE_BYTES`, binary, bad parse) still deletes previous symbols for that path, and therefore enqueues deletes, but enqueues no upsert.

`index_change` (full import) calls `index_paths_at(..., embed = (scope == "main"), ...)`; `reindex_changed` (checkpoint, merge reindex) calls it with `embed = true`. Both pass the `ChangeId` they already have. Every other caller keeps `index_paths` (which means `embed = true`, `change = None`). Deleting a symbol always enqueues a `Delete`, regardless of `embed`, so a task scope that later becomes embedded never leaves orphan points.

- [ ] **Step 1: Failing tests** (`chunks.rs`, TestDb):
  - `index_enqueues_one_upsert_and_second_pass_enqueues_nothing` — `def hello():\n    return 1\n` → one `code_chunk` upsert, `code_chunks.text` starts with `a.py\nhello\n`, `chunk_exists` true; indexing the same bytes again leaves the outbox count unchanged (**unchanged path**).
  - `symbol_body_over_2000_chars_is_truncated` — one function whose body is 5_000 `'x'` chars; `text.chars().count() == 2000` and `text.ends_with('…')` (**Review Focus 5**).
  - `file_over_1mb_enqueues_no_upsert` — bytes longer than `MAX_FILE_BYTES` → no upsert row. Index a small file first, then replace it with that huge file → one `delete` row and `list_code_chunks` empty (**Review Focus 5**).
  - `rewriting_a_file_enqueues_delete_then_upsert` — old symbol id's op is `delete`, new id's op is `upsert`, ids differ.
  - `remove_paths_enqueues_deletes` — and a `BEFORE INSERT ON outbox` exception during `index_paths` rolls back the `symbols` insert (count 0).
  - `task_base_import_enqueues_no_upserts_but_checkpoint_does` — `index_paths_at(repo, "fix-x", None, false, all_files)` writes symbols and `code_chunks` rows and leaves the outbox empty; re-indexing one changed file with `embed = true` enqueues exactly that file's symbols; main-scope full import enqueues all.
  - `task_scope_stores_task_id_and_change` — a task slug that exists gets `task_id`; `index_paths_at(..., Some(&ChangeId("abc".into())), ...)` stores `change_id = "abc"`. Scope `main` with no task leaves `task_id` null.
- [ ] **Step 2:** `cargo test -p av-index --test chunks` → FAIL.
- [ ] **Step 3:** Implement. Do not change symbol/graph behavior. Existing `tests/graph.rs` must keep passing (they now also write outbox rows).
- [ ] **Step 4:** `cargo test -p av-index` → PASS.
- [ ] **Step 5: Commit** `feat(index): code chunk outbox in the index transaction` with the trailer.

### Task 4: TEI embedder client

**Files:** Create `crates/av-semantic/` (`src/{lib,embed}.rs`, `tests/embed.rs`). Workspace member via `crates/*`.

**Interfaces — Produces:**

```rust
#[async_trait]
pub trait Embedder: Send + Sync {
    fn dimension(&self) -> usize;
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
}
pub struct TeiEmbedder;
impl TeiEmbedder {
    /// Does not connect. `dimension()` embeds `"ping"` on first use and caches that vector's length.
    /// No `Authorization` header.
    pub fn new(base_url: impl Into<String>) -> Self;
    /// `api_key` `None` or blank: no `Authorization` header. Otherwise `Bearer <key>` on `/embed` and `/health`.
    pub fn with_api_key(base_url: impl Into<String>, api_key: Option<String>) -> Self;
}
/// Test double. `embed` of `n` texts returns `n` vectors of `dimension()`, derived from the text,
/// unless `fail` is set (then `AvError::Semantic`) or `wrong_dim` is set (then length `dimension()-1`).
pub struct ScriptedEmbedder { /* dimension, fail, wrong_dim, calls: Arc<AtomicUsize> */ }
```

`embed` of an empty slice returns `Ok(vec![])` and makes no HTTP call. Otherwise POST `{base}/embed` with `{"inputs": texts, "truncate": true}` and a 10 s timeout. Error if the response is not one vector per input, or (after the first success) any vector's length differs from the cached dimension. Map transport and HTTP errors to `AvError::Semantic`. When constructed with `with_api_key` and a non-empty key, `/embed` and `/health` include `Authorization: Bearer <key>`. `new` and a blank key omit that header. Do not log the key.

- [ ] **Step 1: Failing tests** against an in-process axum fake (not Docker):
  - `posts_inputs_and_truncate_true` — request JSON has `truncate: true` and the same `inputs`; returned vectors match the body.
  - `empty_inputs_do_not_call_http` — fake's counter stays 0.
  - `embed_rejects_vector_whose_length_is_not_the_probe_dim` — first call returns 4-d and `dimension()` is 4; second call returns 3-d → `Err(AvError::Semantic(_))`, no panic (**Review Focus 1**).
  - `rejects_a_count_mismatch` — two inputs, one vector → `Err`.
  - `scripted_embedder_can_fail_and_lie_about_dim` — `fail` / `wrong_dim` behave as the struct comment says (the worker tests reuse this).
  - `sends_authorization_bearer_when_api_key_is_set` — `with_api_key(url, Some("sk-test".into()))` sends `Authorization: Bearer sk-test` on `/embed`.
  - `omits_authorization_when_api_key_is_blank` — `new` and `with_api_key(url, Some("  ".into()))` send no `Authorization` header.
- [ ] **Step 2:** `cargo test -p av-semantic --test embed` → FAIL.
- [ ] **Step 3:** Implement `TeiEmbedder` and `ScriptedEmbedder`. Dependency: `reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }`.
- [ ] **Step 4:** `cargo test -p av-semantic --test embed` → PASS.
- [ ] **Step 5: Commit** `feat(semantic): text-embeddings-inference client` with the trailer.

### Task 5: Vector index and `Semantic` search

**Files:** Create `crates/av-semantic/src/{vector,search}.rs`, `tests/qdrant.rs`. Modify `src/lib.rs`.

**Interfaces — Consumes:** `Semantic`, `SemanticHit`, `Embedder` (Tasks 1, 4). **Produces:**

```rust
pub const COLLECTION_RECORDS: &str = "records";
pub const COLLECTION_CONVENTIONS: &str = "conventions";
pub const COLLECTION_CODE_CHUNKS: &str = "code_chunks";

pub struct UpsertPoint { pub entity_id: String, pub vector: Vec<f32>, pub payload: SemanticHit }

#[async_trait]
pub trait VectorIndex: Send + Sync {
    async fn ensure(&self, dim: usize) -> Result<()>;
    async fn upsert(&self, kind: SemanticKind, points: &[UpsertPoint]) -> Result<()>;
    async fn set_superseded(&self, kind: SemanticKind, entity_id: &str) -> Result<()>; // flag true; missing point is Ok
    async fn delete_points(&self, kind: SemanticKind, entity_ids: &[String]) -> Result<()>; // missing is Ok
    async fn search(&self, kind: SemanticKind, repo: &str, vector: &[f32], limit: usize) -> Result<Vec<SemanticHit>>;
    async fn delete_repo(&self, repo: &str) -> Result<()>;
    /// Drop and recreate all three collections at `dim`. Used only by a full reindex.
    async fn recreate(&self, dim: usize) -> Result<()>;
}
pub struct MemoryIndex;
impl MemoryIndex { pub fn new() -> Self; }
pub struct QdrantIndex;
impl QdrantIndex { pub fn new(grpc_url: impl Into<String>) -> Self; } // lazy connect
pub struct SemanticSearch<E, V> { /* embedder, index */ }
impl<E: Embedder, V: VectorIndex> Semantic for SemanticSearch<E, V> { /* search */ }
```

`ensure`: create a missing collection at Cosine `dim`; if it exists at another dim, `Err(AvError::Semantic)` naming the collection, the stored dim, and `dim`. Do not delete it. `search` on `VectorIndex` applies `repo` + `superseded = false` and returns hits with that payload and the cosine score (higher is better). `set_superseded` only flips the flag.

`Semantic::search`: trim the query; empty → `Ok(vec![])` and do not call `embed`. Otherwise embed the one string, require `dimension()` matches, `tokio::join!` the three collections at `limit`, drop any hit that is superseded or in another repo (defense in depth), sort by `score` desc then `entity_id` asc (`record_id`, else `symbol_id`, else `convention_id`, display).

`QdrantIndex` uses `qdrant-client` 1.13 and the gRPC URL. Payload map is built field by field (nulls included), not `serde_json::to_value(&hit)`, so `score` and `kind` are not stored. If the crate's default features link OpenSSL, turn on its rustls/tonic feature instead; Task 8's `ldd` check is the gate.

- [ ] **Step 1: Failing tests** in `av-semantic` (memory, always run):
  - `search_filters_repo_and_superseded` — points in repo `a` (one `superseded: true`) and repo `b`; `SemanticSearch` over `MemoryIndex` with a `ScriptedEmbedder` returns only the live `a` hit.
  - `empty_query_does_not_embed` — `search("a", "  ", 8)` is `Ok([])` and `ScriptedEmbedder` calls stay 0 (**Review Focus 4**).
  - `ensure_rejects_a_dim_change` — `ensure(4)` then `ensure(3)` is `Err` and a following search still uses the 4-d collection.
  - `set_superseded_on_a_missing_point_is_ok` — and a second `upsert` of the same `entity_id` replaces the payload (one point).
- [ ] **Step 2:** `cargo test -p av-semantic` → FAIL.
- [ ] **Step 3:** Implement `MemoryIndex`, `QdrantIndex`, `SemanticSearch`. Export them from the crate root. `MemoryIndex` keeps its points in an interior `Mutex` and is `Clone` (clones share that mutex) so a test can retain a clone, pass `Arc<dyn VectorIndex>` into the worker, and `search` the same points. `search` fills `SemanticHit.score` from the query cosine; it ignores whatever score was on the upserted payload.
- [ ] **Step 4:** `cargo test -p av-semantic` → PASS with `AV_TEST_QDRANT_URL` unset.
- [ ] **Step 5: Gated test** `qdrant_roundtrip_filters_repo_and_superseded` in `tests/qdrant.rs`. If `AV_TEST_QDRANT_URL` is unset, print `skip: AV_TEST_QDRANT_URL unset` and return. If set (e.g. `http://127.0.0.1:6334`), run the same filter/supersede/replace assertions as the memory test against `QdrantIndex` with a unique repo name, then `delete_repo`. Re-run `cargo test -p av-semantic`; with the variable unset it still PASSes, and that pass does **not** exercise Qdrant. **This environment cannot run the live path** (no Docker). Do not `#[ignore]` the test.
- [ ] **Step 6: Commit** `feat(semantic): qdrant and in-memory vector index` with the trailer.

### Task 6: Outbox worker

**Files:** Create `crates/av-daemon/src/worker.rs`. Modify `crates/av-daemon/src/lib.rs` and `Cargo.toml` (`av-semantic`). Test: `crates/av-daemon/tests/worker.rs`.

**Interfaces — Consumes:** Tasks 2–5. **Produces:**

```rust
pub struct DrainStats { pub done: usize, pub failed: usize }
pub struct OutboxWorker { /* PgStore, PgIndex, Arc<dyn Embedder>, Arc<dyn VectorIndex> */ }
impl OutboxWorker {
    pub fn new(store: PgStore, index: PgIndex, embedder: Arc<dyn Embedder>, vectors: Arc<dyn VectorIndex>) -> Self;
    /// Claims up to 32 rows, applies them in ascending id, sequentially. One row's error does not stop the batch.
    pub async fn drain_once(&self) -> Result<DrainStats>;
}
```

Apply, in id order, one row at a time:

| op | behavior |
|---|---|
| `Delete` | `delete_points`, then `complete_outbox`. Missing point is success. |
| `Supersede` | `set_superseded`, then complete. Missing point is success. |
| `Upsert` | If the entity is gone, complete **without** embed and without upsert: record → `get_record` is `NotFound`; code chunk → `chunk_exists` is false; convention → that id is absent from `list_all_conventions`. Otherwise embed `row.text` (empty string if null). If `vector.len() != embedder.dimension()`, `fail_outbox` and do not upsert. For a record, set payload `superseded` from `record.superseded_by.is_some()`, **not** from the outbox column. Then upsert and complete. |

Any other `Err`: `fail_outbox` with the error string, increment `failed`, continue. `ensure(dim)` once per `drain_once` before the first upsert, using `embedder.dimension()` after the first successful embed. A dim mismatch from `ensure` fails the row and does not recreate the collection.

- [ ] **Step 1: Failing tests** (TestDb + `ScriptedEmbedder` + `MemoryIndex`). Keep a `MemoryIndex` clone (or `Arc<dyn VectorIndex>`) outside the worker so assertions can `search` it:
  - `upsert_for_missing_record_is_dropped_without_a_point` — outbox upsert whose `record_id` does not exist; after `drain_once`, outbox is empty, embedder calls are 0, memory index has no point. Same for a `code_chunk` upsert whose symbol is absent (**Review Focus 2**).
  - `wrong_dimension_does_not_upsert_and_keeps_the_row` — `ScriptedEmbedder.wrong_dim = true`; `failed >= 1`, the row is still claimed-or-present (`outbox_count` is 1), no point (**Review Focus 1**).
  - `applying_the_same_upsert_twice_leaves_one_point` — drain, re-enqueue the same upsert, drain again; one point, same payload (**Review Focus 3**).
  - `redelivered_upsert_does_not_clear_superseded` — drain the original upsert (`superseded` false), supersede the record in Postgres and drain the supersede row (point flag true), enqueue another upsert copied from the first row with `superseded: false`, drain; the point stays `superseded: true` (**Review Focus 3**).
  - `two_workers_claim_disjoint_rows` — 10 upserts, `tokio::join!` of two `drain_once`; `done` sums to 10, embedder calls are 10, 10 points.
  - `embedder_down_keeps_the_row_until_a_later_drain` — `fail` set, then cleared, `locked_until` moved to the past via `pool()`; the second drain completes the row.
  - `supersede_is_applied_before_a_later_upsert_of_another_entity` — ids increase; a supersede of A and an upsert of B both land.
- [ ] **Step 2:** `cargo test -p av-daemon --test worker` → FAIL.
- [ ] **Step 3:** Implement `drain_once` as the table above. No sleep loop in this task (Task 8 spawns it).
- [ ] **Step 4:** `cargo test -p av-daemon --test worker` → PASS.
- [ ] **Step 5: Commit** `feat(daemon): outbox embedding worker` with the trailer.

### Task 7: Semantic candidates and the degraded full-text fallback

**Files:** Modify `crates/av-pack/src/{score,assemble,packer}.rs`, `crates/av-pack/Cargo.toml` (add `tokio.workspace = true` to `[dependencies]`). Test: extend `crates/av-pack/tests/pack.rs` and `src/score.rs` tests. Do not add `reqwest` or `av-semantic`.

**Interfaces — Consumes:** `Semantic`, `SemanticHit` (Task 1), `IntentStore::search_records_fts` (Task 2). **Produces:**

```rust
pub const W_STRUCT: f32 = 0.35;
pub const W_SEM: f32 = 0.30;
pub const W_RECENCY: f32 = 0.15;
pub const W_KIND: f32 = 0.20;
pub const SEMANTIC_LIMIT: usize = 8;
pub const SEMANTIC_TIMEOUT: Duration = Duration::from_millis(150);

pub fn record_score(kind: RecordKind, hops: u8, age_days: f64, on_seed: bool, semantic: f32) -> f32;
pub fn code_score(hops: u8, semantic: f32) -> f32;
pub fn convention_score(semantic: f32) -> f32; // 0.5 + W_SEM * clamp(semantic); 0.5 when semantic is 0

impl Packer {
    pub fn new(vcs, store, index) -> Self; // unchanged: semantic = None
    pub fn with_semantic(vcs, store, index, semantic: Option<Arc<dyn Semantic>>) -> Self;
}
```

`record_score` = `W_STRUCT*structural + W_SEM*clamp(semantic) + W_RECENCY*recency + W_KIND*kind_priority + (SEED_BONUS if on_seed)`. `code_score` = `W_STRUCT*structural + W_SEM*clamp(semantic) + W_RECENCY + W_KIND*0.5`, then the existing `TEST_CODE_FACTOR`. Pass `0.0` at every existing call site.

`gather` query = trimmed `focus` if it is non-empty, else trimmed `task.goal`. If that is empty, do not call `Semantic`. Otherwise, if `semantic` is `Some`, start `search(repo, query, SEMANTIC_LIMIT)` immediately and `timeout` it at `SEMANTIC_TIMEOUT` measured from the start of that call (overlap it with the VCS/index reads; do not wait an extra 150 ms after a slow index). `Ok` → merge. `Err` or elapsed → push `"semantic"` once and call `search_records_fts(repo, query, SEMANTIC_LIMIT)`. FTS `Err` is ignored; `build` still returns `Ok`. FTS hits are extra records with semantic similarity `0.0` (do not pretend `ts_rank` is cosine). `semantic == None` → no FTS and no `"semantic"` entry (Plan 1).

Merge rules:

- Match existing items by id (`rec:` + first 8 hex chars of `Uuid::simple`, `sym:{symbol_id}`, `conv:{convention_id}` — the ids `assemble.rs` already uses). Replace that item's score using the hit's clamped similarity. Do not duplicate the item.
- A record hit not already present: load it with `get_record`. Skip it when `superseded_by` is set, or when `repeats_goal` would have skipped it. Otherwise add a History item (or Task, for the task's own non-repeated intent) with `hops = FAR_HOPS` unless anchors say better.
- A code hit not already present: `get_symbols` and add a Code item at `FAR_HOPS`. Render the excerpt only from the **current task head**; a missing path keeps the signature summary. Do not read another workspace.
- A convention hit is added or rescored only when its glob matches the task's paths (the Plan 1 scope rule). Drop it otherwise.
- Drop any code hit whose `task_id` is `Some` and not the current task's id (other tasks' unmerged work). Hits from `main` (`task_id` null) and from the current task are kept.
- Cap **new** semantic code items at 8. Sort hits by score desc, then id, before merging so ties cannot flap.

- [ ] **Step 1: Failing tests**
  - In `score.rs`: `weights_sum_to_one`; `semantic_similarity_is_clamped` (`semantic: 2.0` equals `1.0`, `-1.0` equals `0.0`); `attempt_on_seed_beats_perfect_semantic_note` — `record_score(Attempt, 0, 0.0, true, 0.0) > record_score(Note, FAR_HOPS, 0.0, false, 1.0)`. Update the existing score tests to pass `0.0`.
  - In `pack.rs`, a local `FakeSemantic` (`hits`, `fail`, `delay`, `calls`):
    - `semantic_hit_on_unanchored_other_task_record_is_included` — the other task's unanchored summary appears; `Packer::new` (no semantic) does not include it.
    - `semantic_failure_reports_semantic_and_uses_fts` — `fail = true`, FTS query matches that same record, `degraded == ["semantic"]` (exactly one), `build` is `Ok`, the summary is in the markdown.
    - `blank_focus_and_blank_goal_does_not_search` — goal `"  "`, focus `None`; calls stay 0; `degraded` does not contain `"semantic"` (**Review Focus 4**).
    - `superseded_hit_is_not_shown` — fake returns a hit for a record that Postgres has superseded; the summary is absent.
    - `two_builds_with_the_same_hits_are_byte_identical`.
    - `slow_semantic_degrades_quickly` — `delay` 2 s; `build` finishes in under 1 s and `degraded` contains `"semantic"`.
  - Existing `attempt_on_seed_outranks_unrelated_note` and `superseded_record_absent_from_pack` stay as they are (`Packer::new`).
- [ ] **Step 2:** `cargo test -p av-pack` → FAIL on the new tests.
- [ ] **Step 3:** Implement. If `golden_pack_for_fixture_task` fails, regenerate `tests/golden/basic.md` only when the sections are the same set and the diff is order or tier from the rebalanced weights; read the diff before committing it. Do not weaken the golden assertion.
- [ ] **Step 4:** `cargo test -p av-pack` → PASS. `cargo test -p av-daemon -p av-cli` → PASS (constructor signatures unchanged).
- [ ] **Step 5: Commit** `feat(pack): semantic candidates and degraded full-text` with the trailer.

### Task 8: Wire the worker and `POST /v1/reindex`

**Files:** Create `crates/av-daemon/src/reindex.rs`. Modify `src/{main,lib,api,wiring,error}.rs`. Test: `crates/av-daemon/tests/api.rs` (add cases; keep the existing suite).

**Interfaces — Consumes:** Tasks 5–7. **Produces:**

```rust
pub struct ReindexReport { pub enqueued: u32, pub upserted: u32, pub degraded: Vec<String> }
pub const SEMANTIC_LOCK_KEY: i64 = 7_420_002;

// POST /v1/reindex   { "repo": Option<String> }
// → 200 { "repo", "enqueued", "upserted", "degraded" }
```

`Config` gains `qdrant_url: Option<String>` (`AV_QDRANT_URL`), `embed_url: Option<String>` (`AV_EMBED_URL`), and `embed_api_key: Option<String>` (`AV_EMBED_API_KEY`, trimmed; blank is `None`). Both URLs empty → `semantic` stays `None`, no worker, reindex still rebuilds the **outbox** and returns `upserted: 0`, `degraded: ["semantic"]`. A missing API key does not do that. `TeiEmbedder::with_api_key(embed_url, embed_api_key)`.

```rust
pub struct Built { pub state: AppState, pub worker: Option<OutboxWorker> }
pub async fn build_state(cfg: &Config) -> Result<Built>;

impl AppState {
    /// Rebuilds the packer. Does not start a worker.
    pub fn with_semantic(self, semantic: Arc<dyn Semantic>) -> Self;
    /// Stores the pair, sets `semantic` to `SemanticSearch` over them, for reindex tests.
    pub fn with_index_backend(self, embedder: Arc<dyn Embedder>, vectors: Arc<dyn VectorIndex>) -> Self;
}
```

`AppState` holds `semantic: Option<Arc<dyn Semantic>>`, `embedder: Option<Arc<dyn Embedder>>`, and `vectors: Option<Arc<dyn VectorIndex>>`. `AppState::new` leaves all three `None`.

Both URLs set → `embedder = TeiEmbedder::new`, `vectors = QdrantIndex::new` (neither dials at startup), `semantic = SemanticSearch` over that pair, and `worker = Some(OutboxWorker::new(...))`. `main` spawns `loop { tick; sleep 1s }` and aborts the task when axum's serve returns. A `drain_once` error is logged and the loop continues.

The lock is session-scoped, so a tick and a reindex must each hold **one** `pool.acquire()` connection across the critical section and run `pg_advisory_unlock` on that same connection before dropping it. A tick uses `pg_try_advisory_lock(SEMANTIC_LOCK_KEY)` and skips the tick when it returns false. Reindex uses blocking `pg_advisory_lock` on its connection for the whole handler. Returning a connection to the pool without unlocking leaks the lock onto a random pooled session.

`POST /v1/reindex`:

1. `pg_advisory_lock(SEMANTIC_LOCK_KEY)` for the whole call (blocks a worker tick).
2. Build upsert rows from `list_records_for_embed`, `list_all_conventions`, and `list_code_chunks` for that repo (or all repos). Records keep `superseded: record.superseded_by.is_some()` and `text` = summary + body, same as Task 2. Chunks use the stored `code_chunks.text`.
3. `clear_outbox(repo)` and `enqueue` those rows in one transaction.
4. If the vector layer is configured: `delete_repo` for one repo, or `recreate(embedder.dimension())` when `repo` is omitted (this is the model-swap recovery). Then `drain_once` until the outbox for that scope is empty or a call returns `failed > 0 && done == 0`. A poison row stops the loop, stays queued, and the response is still **200** with `degraded: ["semantic"]` and `upserted` equal to the rows completed. If the layer is not configured, skip this step.
5. Unlock.

`AvError::Semantic` maps to 500 only when the Postgres half of reindex fails. `GET .../context` stays 200 when search fails (Task 7 already swallowed it). One repo whose collection dim does not match the embedder: reindex returns 200 `degraded: ["semantic"]` and does **not** call `recreate` (that would drop other repos). The error string tells the caller to run `av reindex` with no repo.

- [ ] **Step 1: Failing tests** (existing `tests/api.rs` TestDb daemon, no Qdrant):
  - `reindex_without_qdrant_enqueues_and_reports_semantic` — after `POST /v1/repos` and a record, `POST /v1/reindex` `{"repo":"demo"}` is 200, `enqueued >= 1`, `upserted == 0`, `degraded == ["semantic"]`, `outbox_count` equals `enqueued`.
  - `context_with_failing_semantic_is_200_and_marks_degraded` — `AppState::with_semantic` of a `Semantic` that always returns `Err`; `GET` context is 200 and `degraded` contains `"semantic"`. Add `pub fn with_semantic(self, Arc<dyn Semantic>) -> Self` which rebuilds the packer.
  - `reindex_with_memory_index_upserts_and_clears_outbox` — build a fresh `AppState` (not the `Env` helper; its fields are private). Keep `let vectors = Arc::new(MemoryIndex::new()) as Arc<dyn VectorIndex>` and pass `vectors.clone()` to `with_index_backend`. Response has `upserted == enqueued`, `degraded` empty, `outbox_count == 0`. `vectors.search(SemanticKind::Record, "demo", query_vector, 8)` returns the live record and not a superseded one.
  - `unknown_route_unchanged` — existing API tests still pass, including `AppState::new` with no semantic.
- [ ] **Step 2:** `cargo test -p av-daemon --test api` → FAIL on the new cases.
- [ ] **Step 3:** Implement wiring, the lock, the route, and the spawn loop. Then `cargo build -p av-daemon && ldd target/debug/av-daemon` must **not** list `libssl`. If it does, the qdrant or reqwest feature flags are wrong (rustls only).
- [ ] **Step 4:** `cargo test -p av-daemon` → PASS. `cargo clippy --workspace -- -D warnings` → clean.
- [ ] **Step 5: Commit** `feat(daemon): reindex endpoint and semantic wiring` with the trailer.

### Task 9: `av reindex`

**Files:** Modify `crates/av-cli/src/main.rs`. Test: `crates/av-cli/tests/cli.rs`.

**Interfaces — Consumes:** `POST /v1/reindex` (Task 8). **Produces:** `av reindex [--repo NAME]`. No task id required. Prints `reindex: enqueued N, upserted M`. Each `degraded` entry goes to stderr as `av: note: {entry}` (same loop `context` already uses). Non-2xx exits non-zero, same as the other commands.

- [ ] **Step 1: Failing test** `reindex_prints_counts_and_exits_zero` — against `start_daemon()` (semantic not configured): `av reindex --repo demo` after `repo add` exits 0, stdout contains `enqueued` and `upserted 0`, stderr contains `semantic`. `reindex_unknown_server_exits_nonzero` — `AV_URL=http://127.0.0.1:1` exits non-zero.
- [ ] **Step 2:** `cargo test -p av-cli --test cli reindex` → FAIL.
- [ ] **Step 3:** Add the clap variant and the POST. Do not require `.av-task`.
- [ ] **Step 4:** `cargo test -p av-cli` → PASS.
- [ ] **Step 5: Commit** `feat(cli): av reindex` with the trailer.

### Task 10: Compose — TEI, healthchecks, env

**Files:** Modify `deploy/docker-compose.yml`, `deploy/Dockerfile` (add `curl` to the runtime `apt-get` line so the daemon healthcheck has a client), `README.md`. Do not make `scripts/smoke.sh` wait on TEI.

**Interfaces — Consumes:** `AV_QDRANT_URL` (already `http://qdrant:6334`) and `AV_EMBED_URL` (Task 8). **Produces:** service `tei` and the env/healthchecks below. No Rust signature changes.

**Not verifiable here.** No Docker. Do not say this task passed.

Compose additions, exact values:

- Service `tei`. Image `ghcr.io/huggingface/text-embeddings-inference:cpu-1.7.2` (pinned; if that tag does not exist when someone can pull, use the newest published `cpu-1.7.*` and write the chosen tag in a comment — do not use `latest`). Command `["--model-id", "BAAI/bge-small-en-v1.5", "--port", "80"]`. Volume `av-tei:/data`. Env `HUGGINGFACE_HUB_CACHE=/data`. Port `127.0.0.1:${AV_TEI_PORT:-8080}:80`. Healthcheck: `curl -fsS http://127.0.0.1:80/health`, interval 10 s, timeout 5 s, retries 40, `start_period` 120 s (the image downloads the model on first start). Comment that the tag was not pulled in the environment that wrote this plan.
- Daemon env **keeps** `AV_QDRANT_URL: http://qdrant:6334`. Add `AV_EMBED_URL: http://tei:80` and `AV_EMBED_API_KEY: ${AV_EMBED_API_KEY:-}` (host key forwarded; default empty; never a literal secret). Daemon `depends_on` stays postgres + `store-init` only. Do not wait on `tei` or `qdrant`; a down embedder must not block `av checkpoint`.
- Daemon healthcheck, now that `curl` is in the image: `curl -fsS http://127.0.0.1:7420/healthz`, interval 5 s, timeout 3 s, retries 20.
- Qdrant's existing healthcheck stays as it is. Add volume `av-tei` next to `av-qdrant`.
- README: the stack is daemon + Postgres 16 + Qdrant + text-embeddings-inference. Delete the sentence that says Qdrant is unused. Document `av reindex [--repo]`, `AV_TEI_PORT` (default 8080), and that a pack note `semantic` means Qdrant or TEI was unreachable and the pack used Postgres full-text. `scripts/smoke.sh` is unchanged: its assertions must pass while the model is still downloading.

- [ ] **Step 1:** The check is `docker compose -f deploy/docker-compose.yml config` then `scripts/smoke.sh`. **Cannot run here.** Record that in the commit message body.
- [ ] **Step 2:** Write the compose, Dockerfile, and README edits above. `rg` the repo for `AV_EMBED_URL` and `BAAI/bge-small-en-v1.5` and confirm both appear.
- [ ] **Step 3:** An implementer who has Docker runs `docker compose config`, then `scripts/smoke.sh`, and confirms `/healthz` and (once the model is down) `curl -fsS http://127.0.0.1:${AV_TEI_PORT:-8080}/health`. That confirmation is outside this environment.
- [ ] **Step 4: Commit** `feat(deploy): text-embeddings-inference service` with the trailer. Body notes the compose file was not started here.

---

## Self-review notes

- **Spec coverage (this plan's slice):** outbox in the entity transaction → Tasks 2–3; worker, TEI, Qdrant upsert → Tasks 4–6; payload and the three collections → Tasks 1 and 5; `Semantic` trait and Qdrant impl → Tasks 1 and 5; `gather` + score term → Task 7; `degraded: semantic` and `records.tsv` → Tasks 2 and 7; catch-up after an outage → Task 6 backoff plus Task 8's lazy connect; superseded marked, not deleted → Tasks 2, 5, 6; `av reindex` and `POST /v1/reindex` rebuild from Postgres → Task 8–9; compose TEI, healthchecks, `AV_QDRANT_URL`, `AV_EMBED_URL` → Task 10. MCP, handoff, and the benchmark stay in Plans 3 and 4.
- **Plan 1 compatibility:** `Packer::new`, `AppState::new`, and `index_paths` keep their signatures so existing tests stay source-compatible. Score weights do change; Task 7 regenerates the golden pack only if the rebalance moves tiers.
- **Known limits:** the gated Qdrant test and all of Task 10 cannot be executed without Docker. `bge-small-en-v1.5` is 384-d; the running dim is whatever the probe vector's length is, and a mismatch fails the row instead of writing it. A full `av reindex` (no repo) is the only path that recreates collections after a model change.
