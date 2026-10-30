# E2 after Plan 7 overlay, 2026-10-30

Same `scholion-bench scale` as [E2](../2026-10-30-e2-scale/README.md) and
[Community](../2026-10-30-community-scale/README.md), on this 4-core / 15 GB machine (release
build, embedded Postgres 16, no semantic layer, `AV_MAX_CONCURRENT_STARTS=4`). After #31 a task
start opens an empty overlay on `main` instead of `clone_scope`. Raw numbers: `tokio/`, `django/`.

## Summary vs before

| | before (E2) | after overlay | target |
|---|---|---|---|
| start p50, 1 agent (tokio) | 13.5 s | **96 ms** | under 1 s + checkout |
| start p50, 1 agent (django) | 24.5 s | **635 ms** | under 1 s + checkout |
| context p50 (tokio / django) | 94 / 268 ms | **35 / 133 ms** | ≤ 1.5× before |
| tokio, 100 agents | 17 / 100 | **100 / 100** in 27 s | 100 / 100 |
| django, 32 agents | not run (4 failed) | **32 / 32** in 27 s | 32 / 32 |
| DB end (tokio, 161 workspaces) | ~11 GB with many full scopes | **183 MB** | ~task diff |
| DB end (django) | ~2.3 GB / 8 scopes | **428 MB** | — |

### Tokio levels

| agents | merged before | merged now | wall now | start p50 now |
|---|---|---|---|---|
| 1 | 1 / 1 | 1 / 1 | 0.6 s | 0.08 s |
| 8 | 8 / 8 | 8 / 8 | 2.2 s | 0.25 s |
| 32 | 16 / 32 (E2) / 32/32 (queue) | **32 / 32** | 6.9 s | 1.2 s |
| 100 | 17 / 100 | **100 / 100** | 27.4 s | 8.0 s |

### Django levels

| agents | merged | wall | start p50 |
|---|---|---|---|
| 1 | 1 / 1 | 1.8 s | 0.52 s |
| 8 | 8 / 8 | 9.3 s | 2.0 s |
| 32 | 32 / 32 | 26.6 s | 9.1 s |

## What it shows

- **Start is the workspace checkout**, not an index copy. Tokio start p50 fell from 13.5 s to
  96 ms; django from 24.5 s to 635 ms.
- **Concurrency holds.** 100 agents on tokio and 32 on django all merge. The Community queue is
  no longer the limiter; throughput is merges/s of real work (~1.5–2 on tokio).
- **Space follows one copy of main.** End DB sizes are hundreds of MB, not tens of GB of per-task
  graphs. Merged tasks still leave full checkouts on disk (1.4 GB for 161 tokio workspaces) —
  sparse checkouts remain a later plan.
- **Empty-O packs stay fast** when neighbors/in_degree take the plain main-edge walk (follow-up
  in this branch). Without that, empty overlays paid for a full-graph CTE and context p50 jumped
  to ~600 ms on tokio.

## Guards (E1 / packer)

- `cargo test -p scholion-bench --test retrieval_gate` green (lexical/structural floors).
- `cargo test -p scholion-pack --test determinism` green; goldens untouched.
- Full E1 on the 42 public tasks with `--reuse-run` was **not** re-run here (needs a prior
  import+embed into the same daemon). The CI gate is the stand-in for packer identity.

## rust-lang/rust

Shallow clone: 64 078 tracked files, 39 524 code. The import **job** keeps running (Plan 7
Task 0): after ~47 minutes it had indexed **4 200** files into a ~4 GB database — past the E2
failure point (client timeout at ~2 000 files). It did not finish in this session: rebuilding
`symbol_refs` for common names takes ~2 minutes per batch (~10M edges). Throughput ~90 files/min
implies many hours for the full tree. The job API reports `state: running`; kill/restart on a
larger machine (or after capping name fan-out) to finish agents 1 and 8.

## Reproduce

```bash
cargo build --release -p scholion-daemon -p scholion-bench
# lite daemon on :7431, AV_MAX_CONCURRENT_STARTS=4
scholion-bench scale ~/eval-repos/tokio --url http://127.0.0.1:7431 --out out-tokio \
  --agents 1,8,32,100 --samples 20 --daemon-pid $PID --database-url $AV_DATABASE_URL
scholion-bench scale ~/eval-repos/django --url http://127.0.0.1:7431 --out out-django \
  --agents 1,8,32 --samples 20 --daemon-pid $PID --database-url $AV_DATABASE_URL
```
