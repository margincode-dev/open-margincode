# Note for Claude: E4 relay on the rebuilt bench image

2026-10-31, after you rebuilt `scholion-daemon:dev`. This is the run to cite.
The harness table is `report.md` in this directory. Pre-registration is `PREREG.md`.

## What this measured

E4 knowledge relay (`bench/tasks-relay`), not Plan 8 routing.

Question registered before the run: on this image, do `notes` and `av` still accept more
relay tasks than `cold`, as on 2026-10-29 (cold 18/30, notes 30/30, av 30/30)? Secondary:
does `av` cost less per accepted task than `notes` (that run was −1.8%)?

- Image: `scholion-daemon:dev`, created 2026-10-31T16:04:52Z,
  `sha256:3f4cdea532cfabb54ad2d176bdc4922ad80f4bf2ca65d77094109fe4a0c9bc50`.
  It booted on the existing `scholion-bench` database. `/healthz` returned ok.
  Log line: semantic layer on, embedder `http://tei:80` (`bge-small-en-v1.5`, 384-d).
  Qdrant collections were already 384-d.
- Tree the hooks and the harness used: `plan-8-routing` at `af16465`.
- Claude Code 2.1.316, model `deepseek-flash`, seed 1, 10 tasks × 3 reps.
- Arms: `cold`, `notes`, `av`. `av` is hooks plus `--jit-hooks`, not MCP.
- Prices passed to the harness, off-peak Saturday Flash: `0.15,0.15,0.003,0.60`
  (input, cache write, cache read, output, USD per million). DeepSeek bills cache
  writes as input. The 2026-10-29 report used the peak pair `0.30` / `1.20`, so its
  dollar columns are about twice this run's. Compare rates and tokens, not raw dollars.
- Workspaces for every arm: `/mnt/c/Users/vinhq/bench-work`. The container sees imports
  at `/import` and tells clients `ws_host_root=/mnt/c/Users/vinhq/bench-work/ws`.
- Daemon: `http://127.0.0.1:7430` (compose project `scholion-bench`). The host
  `target/debug/scholion-daemon` that had been holding 7430 was stopped so this
  container could bind.
- Budget `$1.10`, session cap `$0.08`. All 90 launched. None skipped.
- 90/90 sessions `status=ok`. Valid sessions: cold 30, notes 30, av 30.
  All 30 `av` session dirs have a non-empty `hook-count`.
- Balance `$2.27` before the smoke check, `$1.70` after the 90. Harness spend for
  the 90 is `$0.590`. The smoke session is separate and cost `$0.005`.

jj logs `executable bit support` / `NotFound` on the Windows mount. Sessions still
finished and the hidden pytest is what accepted them.

## Result

| arm | accepted | cost / accepted | wall / accepted | median tokens to first edit |
|---|---|---|---|---|
| cold | 16/30 | $0.012 | 79 s | 17264 |
| notes | 30/30 | $0.007 | 37 s | 18008 |
| av | 30/30 | $0.006 | 38 s | 16701 |

Harness attribution, `av` against `notes`: cost / accepted −2.5%, median tokens
+0.9%, tokens to first edit −7.3%, success +0.0%. Median cost intervals overlap
(`notes` $0.005–$0.007, `av` $0.005–$0.006).

`cold` misses: customer-region 0/3, order-ids 0/3, product-slug 0/3, line-taxes 1/3,
refund-event 1/3, ledger-refund 2/3. The other four tasks are 3/3. `notes` and `av`
are 3/3 on every task. Repeated dead ends: 0 in every arm.

Pre-registered decision: `cold` accepted fewer than `notes`, so the tasks still
measure the relay. `av` did not beat `notes` on accepts. The cost edge is inside
the median interval, so this run does not show a cost improvement either. The
stable `av` signal is starting the edit sooner, and that was already true on
2026-10-29 (−9.9% tokens to first edit, success also tied 30/30).

Against that earlier run, `cold` went from 18/30 to 16/30. Per task, order-ids
went 2/3 to 0/3, line-taxes 2/3 to 1/3, refund-event 0/3 to 1/3. n=3, so that
shift is the control moving, not `av` improving. `notes` and `av` stayed 30/30.
Wall per accepted task is 37 s and 38 s, the same as 36 s and 38 s on 29 October.
`cold` wall went 63 s to 79 s. Every arm this time was on `/mnt/c`, so the
within-run wall comparison is fair and the cross-date `cold` wall is not.

## Do not cite

- `bench/results/2026-10-31-relay-embed/` stopped at 60/90 because the previous
  image exited with `migration 20 was previously applied but has been modified`.
  A host binary was pointed at the bench Postgres, Qdrant, and TEI. Those rows
  are not a result.
- `bench/results/2026-10-31-relay-deepseek/partial-no-semantic/` has no semantic
  layer. Not a result.
- `bench/results/2026-10-31-relay-image-pilot/` is one `av` smoke session
  (`relay-quote-split`, accepted, hooks present, embeddings grew). Not in the 90.

## Still open

Plan 8 routing (E-G1 through E-G4) was not run. There is still no runner adapter,
no `bench/tasks-routing/`, and no `scripts/route-demo.sh`. E-G0 (Grok Build 1.0.70
as an `av` agent) is written up in `bench/results/2026-10-31-routing-grok/conformance.md`
and the last paragraph of `integrations/grok/README.md`. That was subscription
Grok, not this DeepSeek key.

The bench containers were left up: daemon on 7430, Postgres 5433, Qdrant REST 6343
/ gRPC 6344, TEI 8081. A second daemon on 7430 will fail to bind.

If the next question is whether `av` can beat `notes` once `NOTES.md` is too big
to read whole, that is `bench/tasks-relay-scale`, not another lean relay.
