# Handoff — 2026-10-30 (Plan 7 overlay)

State after Plan 7 Tasks 3–4 landed on a PR off `main`. Delete this once you've acted
(measured Task 5, or started the next item).

## 1. Where things stand

- **`main` has** Plans 1–5, Plan 6 in progress, Plan 7 Tasks 0–2 (import jobs, `ScopeView`,
  migration `0021`, ignored differential until reads landed), E4-at-scale harness (#29),
  Community 32/32 (#28).
- **This PR (Plan 7 Tasks 3–4):** overlay **reads** (view filter, BM25 N/df corrections, edge
  union for neighbors/in_degree, `to_task_scope`) and overlay **writes** (start opens empty
  `scope_overlays`, no `clone_scope`; checkpoint/reconcile/sync/merge/abandon; reconcile under
  `lock_task` before start pack, context, bundles, unrecorded). Differential test
  `overlay_reads_match_a_full_index` is green (no longer ignored).
- **`clone_scope`** is a `PgIndex` test helper only — off `CodeIndex` and the daemon.

### Not done (Task 5)

Measure with `scholion-bench scale` on tokio (1, 8, 32, 100) and django (1, 8, 32), rust-lang/rust
import + 1/8 agents, E1 `--reuse-run` means identical at three decimals. Report under
`bench/results/<date>-e2-overlay/`. Success criteria are in
`docs/superpowers/plans/2026-10-30-scholion-plan-7-overlay-index.md`.

### Known gap (acceptable for now)

No shared advisory lock across **merge advancing main** and **another task's reconcile/pack**.
Practical mitigations: reconcile loop + rebuild-once if main moved mid-pack. Full fix is
merge↔reconcile coordination (plan Risks).

### Still open from earlier handoffs

1. **E4-at-scale paid run** — harness on `main`; needs DeepSeek key (`HANDOFF` from #29).
2. Adoption C9/C11/C12 after scale shows a gain.
3. Interactive Claude Code hook verification.

## 2. Kick-start

```bash
git pull origin main
# or this PR branch until merged
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p scholion-index --test overlay_equiv
```

New migration? Next number is **`0022`**. Never edit `0021_overlay.sql`.

## 3. Suggested order

1. Land / merge this overlay PR if CI is green.
2. Task 5 measurement on a machine with the eval repos (tokio, django; rust-lang/rust if disk).
3. E4-at-scale DeepSeek run (product claim).
4. Then adoption or sparse checkouts if workspace checkout dominates start after overlay.
