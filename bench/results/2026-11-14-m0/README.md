# M0, correction on three silent tasks: run in progress

**Status, 2026-11-16 00:10 UTC: M0 moves to Claude Haiku 5.5 and runs again in full
(`AMENDMENT-1-haiku.md`, committed before the first Haiku session).** On DeepSeek, the smokes and
command 1 ran (the gate passed 30/30), and command 2 ran. Command 3 never started on DeepSeek.
Those sessions are a described-only record under `deepseek/` and go into no M0 table. Nothing here
is a result for M0's question yet. The decision rule, the arms and the bars are `PREREG.md`'s.
Its "Build" section was committed (`223ae29`) before the first paid session.

## What ran on DeepSeek (described only)

| command | sessions | rows | harness figure | balance before → after |
|---|---|---|---|---|
| 0. smoke, build B (`smoke-b/`) | 3 | 3 `ok` | $0.0539 | $1.15 → $1.14 (with smoke A) |
| 0. smoke, build A (`smoke-a/`) | 1 | 1 `ok` | $0.0194 | |
| 1. `cold` (`m0-cold/`) | 30 | 30 `ok` | $0.2592 | $1.14 → $1.01 |
| 2. `av-before`, build A (`m0-av-before/`) | 30 | 30 `ok` | $0.3166 | $0.99 → $0.84 |

These directories are under `deepseek/`. Command 2 was accepted 1 of 10 on
`relay-bus-resubscribe`, 0 of 10 on `relay-quote-split` and 10 of 10 on `relay-tax-with-rate`.
That is a description only. Command 3 (`notes`, `av-phase0`) never ran on DeepSeek, so no arm
here has a pair to be compared with. Its store export is `deepseek/m0-av-before/superseded.jsonl`
(0 rows: no `av-before` session superseded anything) and `records.jsonl` (44 rows). The account
keeps $0.84, which this run leaves unspent.

**The DeepSeek smokes passed**, on both builds. Each row is `status=ok` and valid. In every session the model
answered and made at least one edit, and verify ran. `av`'s `hook-count` is not empty
(session-start, pre-read, post-edit, stop), and `meta.json` names `benchagent` under
`agent_isolation` and Claude Code 2.1.318. Smoke rows are not part of any arm.

**The DeepSeek gate passed**: `cold` was accepted 10 of 10 on `relay-quote-split`,
10 of 10 on `relay-bus-resubscribe` and 10 of 10 on `relay-tax-with-rate` (pooled 30/30, Wilson
0.886 to 1.000). The bar is at least 8 of 10 on each. The model solves all three tasks unaided
here.

## What has run on Haiku 5.5

| command | sessions | rows | harness figure | Claude Code's own figure |
|---|---|---|---|---|
| 0. smoke, build B (`smoke-b/`) | 3 | 3 `ok` | $0.0210 | $0.0210 |
| 0. smoke, build A (`smoke-a/`) | 1 | 1 `ok` | $0.0079 | $0.0079 |

**The smokes pass**, under `PREREG.md`'s conditions and amendment 1's. Every row is `status=ok`
and valid. The model answered, made at least one edit (one to three), and verify ran. `av`'s
`hook-count` shows session-start, pre-read, post-edit and stop. `meta.json` names `benchagent`,
Claude Code 2.1.318 and `models: ["claude-haiku-5-5"]`, and every assistant message in the event
streams is from `claude-haiku-5-5`. The largest single request's prompt was 34,957 tokens, under
the 100K price threshold. The highest session cost $0.0092, under the $0.04 bar. The harness
figure at the list prices matches Claude Code's own `total_cost_usd` to the cent.

The Haiku daemons are new, on new databases (`m0ha`, `m0hb`), as amendment 1 says. The DeepSeek
daemons were stopped at 00:11 UTC, and their stores, workspace roots and run directories were
moved under `/m0/results/deepseek/`.

## Why the DeepSeek run paused at 23:48 UTC (superseded by amendment 1)

The PREREG's stopping rule keeps a $0.50 floor and lowers a command's budget to what the balance
leaves above it. With $1.01, command 2 (budget $0.70) would get $0.51 and command 3 (budget $1.40)
nothing. Both `av` arms would then be partial, and the primary would not be assessed. Commands 2
and 3 must run back to back in one price window, so neither starts until a top-up covers both.
With the budgets as registered ($0.70 + $1.40 + $0.60) plus the floor, the account needs about
$3.20. Then no budget is lowered.

The harness figure and the account moved differently. Command 1 was counted at $0.2592, and the
balance fell $0.13. The second run saw the same kind of gap, and stage 2 did not.

## Deviations so far

- **The owner waived the $0.50 balance floor** at 23:48 UTC, after the gate and before command 2:
  "spend the DeepSeek money to zero, then go on with the Claude key". Commands 2 and 3 run with
  their registered budgets ($0.70, $1.40) on what the account holds ($0.99 before command 2). If
  the account runs out during a command, its sessions fail at the API. They are reported as
  they are, and nothing is filled in on another model. A different model for the missing sessions
  would make the arms differ by model, and the primary would not be assessed.
- **Price window.** Command 1 and commands 2 and 3 started inside what looks like DeepSeek's
  off-peak window: the balance fell half the harness figure, and the window ends at 00:30 UTC.
  The registered prices are passed to the harness either way, so the harness figures stay
  comparable with earlier cells. Commands 2 and 3 run back to back as registered. Command 3 may
  cross the window's end, so the arms may differ in what DeepSeek charged. That is a cost
  difference, not a difference in what the sessions were given.

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
