# agentvcs Plan 2b: Context receipts, knowledge freshness, hard inclusion, charged expand — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make every context pack auditable (a stored receipt of what was sent and why), stop stale knowledge from looking current, guarantee hard constraints survive budgeting, and make `expand` part of the token budget instead of a free escape hatch.

**Architecture:** All four changes live in `av-store` (one migration) and `av-pack` (assembly, allocation, packer), surfaced through existing daemon routes and the CLI. No new crates, no LLM, `av-pack` stays deterministic.

**Tech Stack:** existing workspace (Rust 1.97.1 pinned, sqlx/Postgres, axum, clap).

**Spec:** `docs/superpowers/specs/2026-10-18-agentvcs-design.md` (context packer, record discipline) plus the adoption note `docs/superpowers/notes/2026-10-18-external-design-review.md` items 1–4.

## Global Constraints

- Everything in Plan 1/2 Global Constraints still holds (fmt + `clippy -D warnings` clean, no SQLite, no LLM in `av-pack`, pack never exceeds `budget_tokens`, warm pack < 300 ms).
- Migrations `0004_receipts.sql` (Task 1), `0005_freshness.sql` (Task 2) and `0006_budget.sql` (Tasks 3–4: conventions `required`, `ledger_sessions`). Plan 3's flow migration becomes `0007_flow.sql`.
- Receipts are append-only. A pack response gains `receipt: "rcp:<8 hex>"`; nothing else in the response shape changes.
- `packer_version`: the constant `av_pack::PACKER_VERSION = 1`, bumped by any change to ranking or rendering.
- Commit trailer as in the handoff docs.

## Review Focus

1. **Budget too small for the required items alone** → required items still appear at handle tier, the pack still never exceeds the budget (if even the handles do not fit, the lowest-priority required handles are omitted and listed in `omitted` with reason `required-but-over-budget`) (Task 3).
2. **Record anchored to a file only (no symbol)** → freshness compares the file's content hash; renamed symbol stays `anchor_stale` (Plan 1 behavior), not "changed" (Task 2).
3. **Receipt write fails** (Postgres error) → the pack is still returned, `degraded` gains `receipt` (Task 1).
4. **Expand of an id already sent in full this session** → returned again but charged once more and noted; expand cap counts tokens, not calls (Task 4).

---

### Task 1: Context receipts

**Files:** Create `crates/av-store/migrations/0004_receipts.sql`; modify `crates/av-store/src/pg.rs`, `crates/av-core/src/traits.rs`, `crates/av-pack/src/{budget,packer,assemble}.rs`, `crates/av-daemon/src/api.rs`, `crates/av-cli/src/main.rs`. Test: `crates/av-store/tests/store.rs`, `crates/av-pack/tests/pack.rs`, `crates/av-daemon/tests/api.rs`.

**Interfaces — Produces:**

```rust
// av-pack budget.rs: why an item is a candidate
pub enum Reason { Seed, Hop(u8), Anchored, Semantic, Pinned, Required, Map, Task }
pub struct Item { /* existing */ pub reason: Reason, pub source: Option<(String, u32, u32)> /* path, start, end */ }
// av-core
pub struct ReceiptItem { pub id: String, pub tier: Tier, pub score: f32, pub reason: String, pub source: Option<(String,u32,u32)>, pub tokens: u32 }
pub struct NewReceipt { pub task_id: TaskId, pub session: String, pub head_change: ChangeId, pub packer_version: u32,
                        pub budget: u32, pub tokens: u32, pub items: Vec<ReceiptItem>, pub omitted: Vec<String> }
// IntentStore
async fn add_receipt(&self, r: NewReceipt) -> Result<String>;          // returns "rcp:<8 hex>"
async fn get_receipt(&self, id_or_prefix: &str) -> Result<serde_json::Value>;
// av-pack
pub const PACKER_VERSION: u32 = 1;
pub struct Pack { /* existing */ pub receipt: Option<String> }
```

Migration: `context_receipts(id uuid PK, task_id uuid REFERENCES tasks ON DELETE CASCADE, session text, head_change text, packer_version int, budget int, tokens int, items jsonb, omitted jsonb, created_at timestamptz DEFAULT now())`, index on `(task_id, created_at)`. Routes: `GET /v1/receipts/{id}`. CLI: `av receipt <id>` prints the manifest table (id, tier, reason, tokens, source). `POST .../records` accepts optional `receipt` and stores it on the record (new nullable column `records.receipt_id`).

- [ ] **Step 1: Failing tests:** `pack_returns_receipt_and_manifest_lists_every_chosen_item` (items == chosen, sum of item tokens ≤ pack tokens, each has a non-empty reason), `receipt_records_head_change_and_packer_version`, `record_can_cite_a_receipt`, `receipt_write_failure_still_returns_pack_with_degraded_receipt` (drop the table in the test DB), `get_receipt_by_prefix`.
- [ ] **Step 2:** Run → FAIL. **Step 3:** Implement; `assemble.rs` sets `reason`/`source` where items are built. **Step 4:** `cargo test --workspace` → PASS. **Step 5:** Commit `feat: context receipts`.

### Task 2: Freshness of anchored knowledge

**Files:** Create `crates/av-store/migrations/0005_freshness.sql` (add `anchors.content_hash text`; 0004 is already applied), `crates/av-store/src/pg.rs`, `crates/av-daemon/src/api.rs` (`add_record` computes hashes), `crates/av-pack/src/{assemble,score}.rs`. Test: `crates/av-pack/tests/pack.rs`, `crates/av-daemon/tests/api.rs`.

**Interfaces — Produces:** when a record is added, each anchor stores the SHA-256 of the anchored symbol's source lines at the task's current head (symbol anchor) or of the whole file (path anchor); `None` when the target does not exist yet. At pack time the same hash is recomputed at the task's head; if it differs, the record renders with ` (changed since recorded)` and its score is multiplied by `CHANGED_FACTOR = 0.7`. `anchor_stale` (target gone) keeps priority over this flag.

- [ ] **Step 1: Failing tests:** `record_on_edited_symbol_is_flagged_changed_since_recorded`, `record_on_untouched_symbol_is_not_flagged`, `path_anchor_uses_file_hash`, `removed_symbol_stays_anchor_stale_not_changed`, `changed_record_ranks_below_unchanged_peer`.
- [ ] **Step 2–5:** FAIL → implement → PASS → commit `feat(pack): flag knowledge whose anchored code changed`.

### Task 3: Hard inclusion

**Files:** Modify `crates/av-store` (conventions get `required boolean NOT NULL DEFAULT false`; `add_convention` gains `required`), `crates/av-pack/src/{budget,assemble}.rs`, CLI `av convention add --required`. Test: `crates/av-pack/tests/budget.rs`, `crates/av-pack/tests/pack.rs`.

**Interfaces — Produces:** `Item.required: bool`. `allocate` first reserves handle-tier cost for every required item (in priority order), then runs the normal share allocation on the remainder; required items may be upgraded like any other. Plan 3's pinned review findings use the same flag.

- [ ] **Step 1: Failing tests:** `required_convention_survives_a_150_token_budget`, `required_items_never_exceed_budget` (Review Focus 1), `required_item_can_still_upgrade_to_full_when_room`.
- [ ] **Step 2–5:** FAIL → implement → PASS → commit `feat(pack): required items are reserved before ranking`.

### Task 4: Charged expand

**Files:** Modify `crates/av-pack/src/{packer,ledger}.rs`, `crates/av-store/src/pg.rs` (ledger gains `expand_tokens bigint NOT NULL DEFAULT 0` per session in a new `ledger_sessions(session text PK, expand_tokens bigint, last_seen timestamptz)` table), `crates/av-daemon/src/api.rs` (expand takes `session`), CLI. Test: `crates/av-pack/tests/pack.rs`, `crates/av-daemon/tests/api.rs`.

**Interfaces — Produces:** `Packer::expand(task_id, item_id, session: &str) -> Result<Expanded>` where `Expanded { full: String, tokens: u32, session_expand_tokens: u64 }`. Env `AV_EXPAND_CAP_TOKENS` (default unset = unlimited): when the session's total would exceed it, `IllegalState`-style refusal ("expand cap reached: N tokens; start a fresh session or raise the cap"). Packs show a footer line `expanded this session: N tokens` when N > 0.

- [ ] **Step 1: Failing tests:** `expand_is_charged_to_the_session`, `expand_over_cap_is_refused_with_reason`, `pack_footer_reports_session_expand_tokens`, `repeat_expand_is_charged_again` (Review Focus 4).
- [ ] **Step 2–5:** FAIL → implement → PASS → commit `feat: expand counts against a per-session budget`.
