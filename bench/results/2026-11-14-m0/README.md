# M0, correction on three silent tasks: run in progress

**Status, 2026-11-15 23:48 UTC: the smokes and command 1 have run, and the gate passes.
Commands 2 to 4 have not started: the DeepSeek balance does not cover them above the
pre-registered $0.50 floor.** Nothing here is a result for M0's question yet. The decision rule,
the arms and the bars are `PREREG.md`'s, and its "Build" section was committed (`223ae29`)
before the first paid session.

## What has run

| command | sessions | rows | harness figure | balance before → after |
|---|---|---|---|---|
| 0. smoke, build B (`smoke-b/`) | 3 | 3 `ok` | $0.0539 | $1.15 → $1.14 (with smoke A) |
| 0. smoke, build A (`smoke-a/`) | 1 | 1 `ok` | $0.0194 | |
| 1. `cold` (`m0-cold/`) | 30 | 30 `ok` | $0.2592 | $1.14 → $1.01 |

**The smokes pass**, on both builds. Each row is `status=ok` and valid. In every session the model
answered and made at least one edit, and verify ran. `av`'s `hook-count` is not empty
(session-start, pre-read, post-edit, stop), and `meta.json` names `benchagent` under
`agent_isolation` and Claude Code 2.1.318. Smoke rows are not part of any arm.

**The gate after the control passes**: `cold` was accepted 10 of 10 on `relay-quote-split`,
10 of 10 on `relay-bus-resubscribe` and 10 of 10 on `relay-tax-with-rate` (pooled 30/30, Wilson
0.886 to 1.000). The bar is at least 8 of 10 on each. The model solves all three tasks unaided
here.

## Why the run stops here for now

The PREREG's stopping rule keeps a $0.50 floor and lowers a command's budget to what the balance
leaves above it. With $1.01, command 2 (budget $0.70) would get $0.51 and command 3 (budget $1.40)
nothing. Both `av` arms would then be partial, and the primary would not be assessed. Commands 2
and 3 must run back to back in one price window, so neither starts until a top-up covers both.
With the budgets as registered ($0.70 + $1.40 + $0.60) plus the floor, the account needs about
$3.20. Then no budget is lowered.

The harness figure and the account moved differently. Command 1 was counted at $0.2592, and the
balance fell $0.13. The second run saw the same kind of gap, and stage 2 did not.

## Deviations so far

- **The harness's preflight refused to start twice, before any session.** The first time, before
  smoke B, `first-packs` had left its run directories (`fp2d30`, `fp2e64`) and their kept
  workspaces open to the agent user. The second time, before command 1, the smokes' run
  directories were open. Each time they were closed (`chmod 700`), and the same command ran
  again in the same directory. No session had started, and nothing was spent. The refused
  output of command 1 is `checks/m0-cold.refused.log`. Stage 2 met the same thing once.
- **`first-packs` ran without `ISO`**, since it takes no `--agent-user` or `--av-ws-root` (noted under "Build").

## Files

- `PREREG.md`, with "Build". `analyze.py`, `leak.py` and `content.py` were committed with it.
- `first-packs-a/`, `first-packs-b/`: the free replay. `checks/`: the free checks' output and the
  harness's logs.
- `smoke-a/`, `smoke-b/`, `m0-cold/`: what the harness wrote (`rows.jsonl`, `report.md`, `meta.json`).
- Transcripts are not committed. They stay in `lp-harness` under `/m0/bench-work/runs`, closed to
  the agent user.
