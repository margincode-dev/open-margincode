# Pre-registration: E4 at scale, full sweep, 2026-11-10

Written and committed before the first paid session of the sweep. The git commit time of this
file, and of the push of the branch `e4-scale-sweep`, is the evidence. Nothing below is changed
after the first paid session; anything that turns out differently is written in `README.md` as a
deviation.

## Question and hypothesis

From `DESIGN.md`: as the number N of irrelevant notes grows, does a flat `NOTES.md` (arm `notes`)
lose solved tasks or cost more per solved task, while Scholion's anchored records (arm `av`) do
not?

- **H1 (the thesis):** from N = 0 to N = 500, `notes` gets worse (fewer accepted tasks or a higher
  cost per accepted task) and `av` does not.
- **H0:** neither arm changes beyond its noise, or both change alike. A flat file is then good
  enough up to 500 notes on this repo, and that is reported as the result.

## Design

- Task set `bench/tasks-relay` (ten tallyho relay tasks, `pool/notes.toml`), as in DESIGN.
- Arms: `cold` (the control, no lessons; does not depend on N, run once), `notes`, `av` (hooks +
  CLI, `--jit-hooks`, no MCP).
- Levels N = 0, 50, 200, 500 (`--noise N`, noise seed 1, the default). Run seed 1 (the default).
- Reps 3 × 10 tasks = **30 sessions per cell**. 1 + 2 × 4 = 9 cells, 270 sessions.
- Model `deepseek-flash` through `https://api.deepseek.com/anthropic`, Claude Code 2.1.318. No
  other model, no other API key.
- Harness, `av` and daemon built from `fbc5bb0` (`main` on 2026-11-10), not the smoke's `669080e`.

## Commands (in this order)

```bash
P="--price-per-mtok 0.30,0.30,0.006,1.20"
OUT=bench/results/2026-11-e4-scale
scholion-bench run bench/tasks-relay --arms cold --jit-hooks --reps 3 $P \
  --session-cap-usd 0.08 --budget-usd 0.60 --out $OUT/cold $COMMON --confirm
for N in 500 0 50 200; do
  scholion-bench run bench/tasks-relay --arms notes,av --noise $N --jit-hooks --reps 3 $P \
    --session-cap-usd 0.08 --budget-usd 1.50 --out $OUT/n$N $COMMON --confirm
done
```

Free checks, done before this file was committed: `reference/check.py` all 10 pairs ok;
`scholion-bench check bench/tasks-relay` all 10 tasks ok; `scholion-bench noise
/root/bench-repos/tallyho --rev main -n 50 --seed 1` byte-identical to `noise-sample-50.toml`;
each command above without `--confirm` printed its plan (`cold` 30 sessions, worst case $2.40 at the
cap; each level 60 sessions, 30 per arm, the noise line, worst case $4.80 at the cap; the worst
cases assume every session reaches the $0.08 cap, which DESIGN's estimate of about $0.013 a
session does not).

`$COMMON` is `--av-url http://e4-daemon:7450 --work-root /root/bench-work --import-dir
/root/bench-work/src --import-mount /root/bench-work/src` (the setup of `smoke/README.md`). Each
command runs on its own, one after another; a rerun of the same command resumes it.

## Prices

DESIGN's **peak** prices `0.30,0.30,0.006,1.20` (input, cache write, cache read, output; $/M
tokens) for the whole sweep. The sweep starts at about 04:00 UTC, outside DeepSeek's off-peak window
(16:30 to 00:30 UTC), so the harness figure should match the bill; any session that falls in the
off-peak window is billed at about half, which makes the harness figure an upper bound there. The
balance delta is reported next to the harness figure.

## Metrics

Primary, per (arm, N) cell:

1. **accepted / 30** (the task's hidden tests pass after the session).
2. **cost per accepted task** (sum of session costs / accepted), at the prices above.

Secondary, as the harness reports them: median session cost, median `tokens_total`, median
tokens to first edit, repeated dead ends, wall time per accepted task, and in the "Memory at scale"
table median input tokens (cache reads included), median memory tokens (cl100k size of `NOTES.md`
or of the av task's first pack) and median turns.

## Statistics

What the harness computes (`crates/scholion-bench/src/report.rs`, `stats.rs`), reported unchanged:

- a percentile bootstrap 95 % interval of the **median session cost** and median `tokens_total`
  per arm (2000 resamples, seed 1);
- relative changes between arms in one run (e.g. `relay: av vs notes`), with no interval.

The run report has **no Wilson interval and no paired bootstrap**. For accepted / 30 I add the
95 % Wilson score interval, computed outside the report with the same formula as
`scholion_bench::stats::wilson_interval` (z = 1.959963984540054), and say so where it appears. No
paired or task-stratified test is done; none is claimed.

No claim of a difference is made for a cell with fewer than 30 valid sessions.

## Decision rule (fixed now)

Both comparisons are N = 0 against N = 500 within one arm:

- `notes` **loses solved tasks** if its accepted rate at N = 500 lies below the lower end of its
  N = 0 Wilson interval.
- `notes` **costs more** if its median session cost at N = 500 lies above the upper end of its
  N = 0 bootstrap interval of the median cost.
- `av` **holds** if its N = 500 accepted rate lies inside its N = 0 Wilson interval and its N = 500
  median session cost lies inside its N = 0 bootstrap interval.

H1 holds if `notes` loses solved tasks or costs more, and `av` holds. Cost per accepted task, the
primary metric, has no interval in the harness; its change is reported as a number and the
decision on cost uses the median-cost interval above. As a stricter check I also report whether
the two levels' intervals overlap at all; only non-overlapping intervals are called a significant
difference. N = 50 and N = 200 show the trend; they do not decide.

If `av` degrades too, the diagnosis DESIGN names is done before the work volume is deleted: the
`pack.md` of failed av sessions and whether the lesson was in it (the pre-read hook's output is not
in the transcript, `smoke/README.md`, so that channel cannot be checked).

## Validity and missing sessions

- A session the harness marks `invalid` (e.g. an av session whose hooks never fired) stays in
  `rows.jsonl` as the harness wrote it; the harness does not retry it. It is listed in the README
  and the cell is reported with its number of valid sessions.
- A session that stops the harness with an error (no row written) is an infrastructure failure: I
  fix the cause and rerun the same command, which resumes it. Its cost, which no row carries,
  shows in the balance delta.
- No row is edited, dropped or re-run by hand. `rows.jsonl`, `meta.json` and `report.md` are
  committed as the harness wrote them.
- Before the rest of the `n500` command runs, its first `av` session is checked: `status=ok`, a
  non-empty `hook-count`, a `pack.md`, and 510 sections in the `notes` sessions' `NOTES.md`.

## Stopping rules

- `--session-cap-usd 0.08` per session. With `--price-per-mtok` the harness does not pass
  `--max-budget-usd` to Claude Code, so the cap is only what the budget check reserves for the next
  session; `max_turns` (30) is the real per-session limit.
- `--budget-usd` per command as in DESIGN: $0.60 for `cold`, $1.50 per level.
- **Balance floor $0.50.** The balance on 2026-11-10 at 03:38 UTC was **$5.82**, so at most $5.32
  is spent. Before each command I read the balance B and pass
  `--budget-usd min(DESIGN's budget, 5.32 − harness spend so far, B − 0.50)`. A level that runs out
  is reported as partial (its `skipped_budget` rows show in the report), never extrapolated.
- If the balance drops below $0.50, the sweep stops.
- The balance is read before the sweep and a few minutes after it (billing lags).

## Amendment 1 (2026-11-10, written before any N = 200 session ran)

On the owner's instruction, passed on by the director while the `n500` level was running
(`cold` complete, `n500` in progress, no `n0`, `n50` or `n200` session started): **the N = 200
level is dropped**, both its `notes` and its `av` cell (60 sessions). The reason is budget: the
DeepSeek balance is kept for other pre-registered benchmarks. The decision was made before any
N = 200 session ran and without looking at any result of this sweep beyond the harness's progress
lines and running cost.

- Levels are now N = 0, 50, 500; `cold` once. 1 + 2 × 3 = **7 cells, 210 sessions**, every cell
  still 3 reps × 10 tasks = 30 sessions.
- The commands are unchanged except that the loop is `for N in 500 0 50` (no `n200` directory).
- Hypotheses, metrics, statistics, decision rule (N = 0 against N = 500), validity and stopping
  rules are otherwise unchanged. N = 50 alone now shows the trend between the two ends.

Note, 2026-11-10: host power loss 04:29–12:00 UTC; the in-flight session was discarded and re-run; no
other change. (`cold` was complete; `n500` had 28 rows written, and the session in flight,
`relay-money-plain · notes · rep 2`, had written none. Rerunning the same `n500` command resumes
the cell and runs that session again, as the rule for a session with no row above says. Postgres
recovered from its WAL; the 14 earlier av tasks still hold 511 records each and Qdrant's
`records` collection holds all 7,192 records, with an empty outbox.)

Note, 2026-11-10: planned host restart about 13:00–13:10 UTC; stack stopped cleanly; the in-flight
session was discarded and re-run; no other change. Details:

- `n500` was complete (60/60). `n0` had 23 rows; the session in flight was
  `relay-customer-region · av · rep 2` (the director's message named `notes · rep 2`, but the log's
  last line was the `av` session). It wrote no row.
- Resuming `n0` failed at that session: its av repo `bn0-relay-customer-region-av-2` had already
  been imported before the stop (`import … failed: repo … already exists`, EXIT 1). Scholion at
  `fbc5bb0` has no way to remove a repo (no `DELETE` route, no `av` subcommand) and
  `scholion-bench run` has no flag for a half-imported repo.
- On the director's instruction, `n50` was run next (60/60, unaffected; the level order became
  500, 50, 0) and the stale repo was then removed by hand, with `e4-harness` idle and
  `e4-daemon` stopped. Its rows and Qdrant point ids were first dumped to
  `n0/stale-repo-removed.json`. Removed: in Postgres, in one transaction, its `repos` row, 2 tasks
  (with their 12 records, 10 anchors and 2 context receipts by cascade), 3 events, 166 symbols
  (with their symbol refs and code chunks), 32 index files, 1 index scope, 2 scope overlays; in
  Qdrant, its 12 `records` points and 166 `code_chunks` points (filter `repo`); its jj store
  directory, workspace and import copy. Nothing else was touched.
- Counts before → after: Postgres `records` 17,495 → 17,483 and Qdrant `records` 17,495 → 17,483
  (−12 each), Postgres and Qdrant `code_chunks` 11,952 → 11,786 (−166 each), outbox empty before
  and after, collections green. The daemon was started again on the same binary (still
  `fbc5bb0`), answered `/healthz`, and no longer lists the repo. `n0` was then resumed with the same
  command and re-ran the interrupted session first.
