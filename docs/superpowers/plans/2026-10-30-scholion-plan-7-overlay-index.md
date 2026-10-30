# Scholion Plan 7: Overlay index scopes — a task stores only what it changed

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Starting a task writes nothing to the code index, and a task's index rows follow its own
diff, so many agents can share one large repo. The server keeps one index of `main`. Each request
reads that index through the task's overlay, the files the task's tree has differently. Nothing
is copied per task.

**Why (E2, `bench/results/2026-10-30-e2-scale/`):** `start` copies `main`'s whole index into a
task scope (`clone_scope`): symbols, the name-joined reference graph and code chunks.
- **Time.** 13.5 s per start on tokio (814 code files), 24.5 s on django (2,939).
- **Concurrency.** At 32 and 100 agents on tokio, half and 83 % of starts failed with
  `pool timed out`. On django, 4 agents at once merged nothing.
- **Space.** The tokio database reached 11 GB, about 280 MB per open django task.
- **Large repos.** rust-lang/rust never finished importing.

Packs (94–268 ms), `why` (2–4 ms), records, review and merges (1–3 s) are not the problem.

**Architecture:** One new concept in `scholion-index`: a *view*, which is `main` plus a task's
overlay. All read methods of `CodeIndex` take a view instead of a scope. `scholion-pack` and
`scholion-daemon` pass the task's view where they pass `task.slug` today. No new crates. One
migration.

**Tech stack:** Rust 1.97.1 (pinned), sqlx/Postgres, the existing `TestDb`, the
`scholion-bench scale` and `retrieval` commands.

## Definitions

- **B**: the task's base (the trunk it started from, or last synced to).
- **H**: the task's head.
- **T**: the change `main`'s index is recorded at (`scope_change(repo, "main")`), normally trunk.
- **Overlay paths O**: every indexable path whose content in H differs from T. This is
  `changed(B, H)`, the task's own edits, plus `changed(B, T)`, what `main` merged since B.
  O holds both because the task's tree has the old content of files `main` has since changed.
- **Overlay rows**: index rows in the task's scope for the paths in O, read from H. A path deleted
  in H has a *tombstone* row and no symbols.
- **View(task)**: `main`'s rows whose path is not in O, plus the task's overlay rows. By
  construction it equals the index of H. That equality is the invariant this plan is built on.

## Invariant and how it is guarded

**For any task, every `CodeIndex` read through View(task) returns what the same read returns on a
full index of H.** Ids may differ; content and order must not.

Guard: a differential test (Task 2) builds both, a full scope of H (today's `clone_scope` plus
`index_change`, kept for tests only) and the overlay, over random edit sequences. It compares
each read method's output with ids replaced by `(path, qname)`. It runs with fixed seeds in CI,
like `merge_fuzz`. The pack-level tests (`determinism.rs`, the goldens, the E1 CI gate) must stay
green, and E1 on the 42 public tasks must give identical means.

## Design

### Data model (migration `0021_overlay.sql` here, `0022_overlay.sql` in agentvcs, where 0021 is taken; check the default branch's highest number first)

- `index_files` gains `deleted boolean NOT NULL DEFAULT false`, the tombstone. A task scope's
  `index_files` rows are exactly O. `main`'s scope never has tombstones.
- New table `scope_overlays (repo, scope, main_at text NOT NULL, PRIMARY KEY (repo, scope))`.
  `main_at` is the T the overlay was last reconciled against.
- No other table changes: symbols, `code_chunks` and `symbol_refs` of a task scope hold only
  overlay rows.
- Add a line for the migration to `tests/migrations_frozen.rs`.

### Reads through a view

`ScopeView { repo, scope }`. For `main` the view is plain `main`. For a task, the masked paths are
`SELECT path FROM index_files WHERE repo = $1 AND scope = $task`, small, so they are fetched once
per request and bound as `$masked text[]`. Each query becomes:

```sql
... WHERE repo = $1 AND (
      scope = $task
   OR (scope = 'main' AND NOT (path = ANY($masked))))
```

Per method:

| method | through a view |
|---|---|
| `symbols_in`, `resolve`, `search_symbols` | the filter above; ordering unchanged (`path COLLATE "C"`, `start_line`) |
| `get_symbols`, `symbol_scope` | by id, unchanged (ids are global) |
| `search_code_lexical` | BM25 over the view. N and each term's df: `main`'s counts minus the masked paths' rows plus the overlay's. Both corrections are small queries over O, so no full scan |
| `neighbors`, `in_degree` | see "References" |
| `list_code_chunks`, `chunk_exists` | the outbox and reindex read these per scope; unchanged |

### References

Edges are joined by name: a symbol links to every other symbol in its scope whose last qname
segment is one of its `refs`. Weights are stored but nothing ranks by them: `neighbors` uses hop
distance and `in_degree` counts. So a view's edges are:

1. `main`→`main` edges whose two ends are both unmasked, stored as today;
2. out-edges of overlay symbols, from their `refs` to view symbols of those names;
3. in-edges of overlay symbols, from view symbols whose `refs` contain an overlay symbol's name
   (`symbols_refs_gin` already indexes `refs`).

Edges 2 and 3 are computed in the query, not stored. They touch only overlay symbols, a task's
diff. `neighbors` (a recursive CTE over `symbol_refs` today) takes the union of the three as its
edge relation. `in_degree` counts over the same union. Stored edges for task scopes go away, which
is most of today's per-task space.

### Writes

- **start**: `create_workspace`, then insert `scope_overlays(main_at = T)` with O empty when
  B = T. If B ≠ T (main moved during the start), reconcile (below). No `clone_scope`.
- **checkpoint / snapshot** (`snapshot_and_index`): for the paths `changed(B, H)` changed since
  the last snapshot, index them into the task scope from H: rows, or a tombstone when deleted.
  A path whose H content equals T's content leaves O: delete its overlay rows. That is what
  happens when a task undoes its own edit.
- **reconcile** (main moved: `main_at` ≠ T): for `changed(main_at, T)`, index each path into the
  overlay from H, unless it is already in O. Then set `main_at = T`. This runs lazily under
  `lock_task`, before any read for the task. Its cost follows what `main` merged since, not the
  repo size. Idle tasks cost nothing.
- **sync** (rebase onto trunk): B moves to T. Recompute O as `changed(T, H')` and replace the
  overlay with those paths. Today this runs `index_change` over the whole tree (every file
  read and hashed, the changed ones parsed).
- **merge / abandon**: `drop_scope` deletes only overlay rows (small) and the `scope_overlays` row.
- **main** itself never has an overlay. Its index is updated by `reindex_changed` on merge,
  revert and undo, as today.

### Semantic layer

Vectors exist for `main`'s chunks only. `clone_scope` enqueues nothing either, so this does not
change. `to_task_scope` already maps `main` hits to the task's symbols by `(path, qname)`; it
maps through the view instead. A hit on a masked path maps to the overlay symbol of the same
qname or is dropped.

## Tasks

### Task 0: Baseline numbers and the import-as-job fix

The overlay does not fix this one, so it lands first, on its own branch.

- [ ] `POST /v1/repos` runs inside the request, so a client disconnect cancels a large import
  half done. E2 lost the rust-lang/rust import this way. Make it a job:
  - `POST` returns `202 {job}` at once and runs the import on a spawned task;
  - `GET /v1/repos/{name}/import` returns `{state: running|done|failed, indexed, total, error}`;
  - `scholion-bench` (`runner`, `retrieval`, `scale`), the CLI and `scripts/smoke.sh` poll it;
  - a failed import removes the half-made repo, as `init_repo` does today.

  Tests: a disconnect does not stop the import; a failure is reported, not hung.
- [ ] Re-run `scholion-bench scale` on tokio and django with today's code. Keep the reports as
  the "before" (the 2026-10-30 runs are already this).

### Task 1: `ScopeView` and the migration, no behaviour change

- [x] Migration `0021_overlay.sql` (tombstone column, `scope_overlays`) and its line in
  `migrations_frozen.rs`.
- [x] `ScopeView` in `scholion-index`. Every read method of `CodeIndex` takes `&ScopeView`. For a
  view with no overlay row the SQL is today's. Callers in `scholion-pack` and `scholion-daemon`
  build the view from the task.
- [x] `cargo test --workspace` green with no other change; the goldens are untouched.

### Task 2: The differential test (write it before the overlay)

- [x] `crates/scholion-index/tests/overlay_equiv.rs`, proptest like `merge_fuzz`. Generate a
  `main` tree of Python, Rust and TypeScript files with shared names (so names fan out), then a
  sequence of task edits (modify, add, delete, revert to `main`'s content) and `main` merges.
  After each step, build the reference index of H with full indexing in a fresh scope. Assert
  equal results for `symbols_in`, `resolve`, `search_symbols`, `search_code_lexical`,
  `neighbors` (1 and 2 hops) and `in_degree`, ids mapped to `(path, qname)`.
- [x] It failed until Task 3 landed (`#[ignore]`d); Task 3 removed the `ignore`. The
  overlay is written by hand in the test (rows for differing paths, tombstones, a
  `scope_overlays` row) until Task 4's writes exist. A second test runs the same comparison
  between two full indexes of one tree, checking the harness itself. Both assert that the cases
  produced masked paths, deletions and fanned-out names.

### Task 3: Overlay reads

- [x] The filter for symbols, `resolve` and `search_symbols`.
- [x] Lexical: N and df corrections, then the score query over the view. Ties stay by path and line.
- [x] References: the edge union in `neighbors` and `in_degree`.
- [x] `to_task_scope` through the view.
- [x] The differential test passes for the read side on hand-built overlays.

### Task 4: Overlay writes; remove `clone_scope` from start

- [x] start, checkpoint, reconcile, sync, merge and abandon as in "Writes".
- [x] `clone_scope` stays only behind `#[cfg(test)]` or in the test util, for the differential
  test's reference. Remove it from the daemon.
- [x] API tests: a task started, edited, synced, merged and abandoned leaves no rows; a task idle
  while `main` merges reconciles on its next read; a task that undoes its edit has an empty O.
- [x] Full differential test green; `cargo test --workspace` green; goldens unchanged.

### Task 5: Measure

- [x] `scholion-bench scale` on tokio (1, 8, 32, 100 agents) and django (1, 8, 32); rust-lang/rust
  import job runs past E2's failure point (report notes; full tree + 1/8 agents not finished here).
- [x] Packer identity: retrieval_gate + determinism green (full 42-task `--reuse-run` needs a prior
  embed run — not available in the measure environment).
- [x] Report in `bench/results/2026-10-30-e2-overlay/`; README measured section updated.

## Success criteria

| | before (E2) | target |
|---|---|---|
| start p50, 1 agent (tokio / django) | 13.5 s / 24.5 s | under 1 s plus the workspace checkout |
| tokio, 100 agents at once | 17 / 100 merged | 100 / 100 |
| django, 32 agents at once | not run (4 failed) | 32 / 32 |
| database per open task | ≈ 200–280 MB | proportional to the task's diff (KBs for a one-file task) |
| rust-lang/rust | import never finished | import completes; 8 agents merge |
| context p50 | 94 / 268 ms | no worse than 1.5× |
| E1 means, pack goldens, determinism tests | — | identical |

The workspace checkout (jj `create_workspace`) is measured separately in Task 5. If it dominates
start on large repos, sparse checkouts are the next plan. They are not this one.

## Risks

- **Read cost moves to query time.** Every read now filters masked paths and computes edges 2
  and 3. O is a task's diff, normally a few files, but a long-lived task that never syncs
  accumulates `main`'s merges in O. Reconcile cost and O's size are reported in Task 5. `sync`
  shrinks O back to the task's own edits.
- **Lexical statistics.** BM25 over a view must equal BM25 over H's full index. The df/N
  corrections are the part most likely to be subtly off; the differential test compares scores,
  not only order.
- **Concurrency.** Reconcile writes under `lock_task`, and reads for a task take the same lock.
  `main`'s index advances under the merge lock. A read must not see `main` at T′ with an overlay
  reconciled to T: it checks `main_at` against `scope_change(main)` inside one transaction.
- **Name fan-out stays in `main`.** One copy of `main`'s edges is still about 900k rows for tokio
  and millions for rust-lang/rust. That is a one-time cost now, not one per task. Capping fan-out
  (for example, not linking names with more than N candidates) is a separate retrieval change,
  measured with E1.

## Out of scope

- Sparse or lazy workspaces (checkouts are 9–73 MB per task on the measured repos; the index
  copy was 4–20× that).
- The semantic layer's vectors (already `main`-only).
- Merge queue changes (1–3 s per merge; queueing showed only at 32+ agents).
