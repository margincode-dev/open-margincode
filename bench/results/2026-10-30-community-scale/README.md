# Scale after the start queue, 2026-10-30

The same `scholion-bench scale` run on tokio as in [E2](../2026-10-30-e2-scale/README.md), on the
same 4-core machine (release build, Postgres 16, no semantic layer, pool of 16), after #24: index
copies at task start now wait for one of `AV_MAX_CONCURRENT_STARTS` turns (4 here, a quarter of
the pool), and the import runs as a job the bench polls. Raw numbers: `tokio/`.

| agents | merged before | merged now | wall before | wall now | start p50 now | start max now |
|---|---|---|---|---|---|---|
| 1 | 1 / 1 | 1 / 1 | 15 s | 14 s | 13 s | 13 s |
| 8 | 8 / 8 | 8 / 8 | 127 s | 103 s | 51 s | 101 s |
| 32 | **16 / 32** | **32 / 32** | 300 s | 405 s | 216 s | 401 s |

No request failed: no `pool timed out` in the daemon log, where the earlier run lost half the
32-agent starts to it. Everything other than starting stayed fast under load: context packs p50
266 ms at 32 agents, record, submit, validate and review in tens of milliseconds, merges p50 5 s
(they queue on the merge lock).

## What it shows

- **Every start succeeds, slowly.** That is the Community target in Plan 8: starts queue instead of
  failing. Throughput did not change (0.08 merges/s at 8 and 32 agents) because the work did not
  change: each start still copies main's whole index, about 13 s on tokio. Four at a time, 32
  starts take about 400 s.
- **The queue moves the limit, it does not remove it.** At 100 agents the last start would wait
  about 100 / 4 × 13 s ≈ 20 minutes. The bench client (and the CLI) give up on a request after
  600 s (300 s for the CLI), so that run would lose starts again, this time to client timeouts.
  Writing this up found that such a start, given up while queued, left an open task with no
  workspace behind; the start-queue-cancel change fixes that (the wait now comes before anything
  is written). It was not run. The real fix is to stop
  copying: Plan 7's overlay index, where a start writes nothing to the index.
- Lowering each start's cost is the only lever left for Community; raising the limit (more
  concurrent copies) brings back the contention E2 measured (copies slow each other down) and the
  pool exhaustion.

## Not measured

- 100 agents (above), django, rust-lang/rust. The import job removes the reason the rust-lang/rust
  import failed (the client timeout), but its index is several GB and the machine had 14 GB free.
- Daemon memory dropped from 515 MB to 228 MB at the end, but this run did fewer tasks (61 against
  161), so the two are not comparable.

## Reproduce

```bash
cargo build --release -p scholion-daemon -p scholion-bench
# a fresh database and daemon (AV_DATABASE_URL, AV_STORE_ROOT, AV_WS_ROOT, AV_LISTEN=127.0.0.1:7431)
scholion-bench scale ~/eval-repos/tokio --url http://127.0.0.1:7431 --out out-tokio \
  --agents 1,8,32 --samples 20 --daemon-pid $PID --database-url $AV_DATABASE_URL
```
