# Handoff — 2026-10-30 (Plan 7 Task 5 measured)

Overlay index measured. Delete this once you've acted on the remaining items.

## 1. Where things stand

- **`main` has** Plan 7 Tasks 3–4 (#31): overlay reads/writes, no `clone_scope` on start.
- **This PR (Task 5):** `bench/results/2026-10-30-e2-overlay/` — tokio and django scale after
  overlay. Also a small follow-up: empty-O neighbors/in_degree use the plain main-edge walk
  (packs stay fast), and `scholion-bench scale` skips empty files for `why`.

### Hits vs Plan 7 success criteria

| criterion | result |
|---|---|
| start p50 tokio / django under 1 s + checkout | **96 ms / 635 ms** |
| tokio 100 agents all merge | **100 / 100** in 27 s |
| django 32 agents all merge | **32 / 32** in 27 s |
| context p50 ≤ 1.5× E2 | **35 / 133 ms** (better than E2's 94 / 268) |
| DB proportional to diff | tokio end **183 MB** (was ~11 GB with full scopes) |
| E1 / goldens | retrieval_gate + determinism green; full 42-task `--reuse-run` not re-run (no prior embed) |
| rust-lang/rust import + 1/8 agents | import **job survives**; 4.2k files / ~4 GB at 47 min; not finished here (fan-out cost) |

## 2. What to do next

1. Merge this measurement PR when CI is green.
2. Finish rust-lang/rust import on a bigger box (or cap name fan-out — plan Risks).
3. Optional: full E1 `--reuse-run` against a daemon that already has the 42-task imports+embeddings.
4. **E4-at-scale** DeepSeek run (harness on `main` since #29) — still the product claim.
5. Sparse checkouts if workspace disk (1.4 GB for 161 tokio tasks) becomes the next limiter.

## 3. Reproduce scale

See `bench/results/2026-10-30-e2-overlay/README.md`.
