# M0, correction on three silent tasks: result

**Phase 0 helps, by the pre-registered rule, and the guard holds.** On Claude Haiku 5.5,
`av-phase0` was accepted **30 of 30** (Wilson 0.886 to 1.000) and `av-before` **11 of 30** (0.219
to 0.545). The intervals are disjoint. On the ten true-lesson tasks the guard was accepted 29 of 30
(0.833 to 0.994). The bar is at least 27. The run cost $0.86 for 154 sessions.

The decision rule, the arms and the bars are `PREREG.md`'s. Its "Build" section was committed
(`223ae29`) before the first paid session. The run moved from DeepSeek to Haiku 5.5 by
`AMENDMENT-1-haiku.md` (`15d3273`), committed before the first Haiku session, and every command
then ran again in full on Haiku. The leak and content checks were committed (`5d57c88`) before
any accepted count was read. No claim is made beyond these three tasks, this model and this
repository (tallyho).

**On a second model, Claude Sonnet 5.5, the primary pair gives the same outcome** (amendment 2,
2026-11-19). `av-phase0` was accepted 30 of 30 (0.886 to 1.000) and `av-before` 10 of 30 (0.192
to 0.512). The intervals are disjoint, so by the rule Phase 0 helps. That run had no `cold`, no
`notes` and no guard; see "Second model: Sonnet 5.5 (amendment 2)" below.

## Results (`checks/haiku-analyze.txt`, `analyze.py` as committed)

| arm | valid | accepted | 95 % Wilson | cost | median session |
|---|---|---|---|---|---|
| `cold` | 30 | 30 | 0.886 to 1.000 | $0.1039 | $0.0037 |
| `notes` | 30 | 12 | 0.246 to 0.577 | $0.1353 | $0.0045 |
| `av-before` (build A) | 30 | 11 | 0.219 to 0.545 | $0.1726 | $0.0058 |
| `av-phase0` (build B) | 30 | 30 | 0.886 to 1.000 | $0.1859 | $0.0063 |
| guard (ten relay tasks, build B) | 30 | 29 | 0.833 to 0.994 | $0.2343 | $0.0069 |

Per task, accepted of 10:

| task | `cold` | `notes` | `av-before` | `av-phase0` |
|---|---|---|---|---|
| `relay-quote-split` | 10 | 2 | 0 | 10 |
| `relay-bus-resubscribe` | 10 | 0 | 1 | 10 |
| `relay-tax-with-rate` | 10 | 10 | 10 | 10 |

- **The gate passes**: `cold` 10 of 10 on each task. Haiku solves all three unaided.
- **Primary: Phase 0 helps.** `av-phase0`'s lower end, 0.886, is above `av-before`'s upper end,
  0.545.
- **The guard holds**: 29 of 30 valid, against the bar of 27. The one miss is
  `relay-refund-event` rep 2, at 24 turns. See the deviation below on how `analyze.py` printed
  this line.
- **Secondary 1, the gap to the notes file.** By the registered wording, `av` "is not shown
  worse than `notes` at this size". Described plainly, `av-phase0` (30/30) is above `notes`
  (12/30), and the intervals are disjoint. Secondary measures are not used for the decision.
- **Where the effect is.** It is on the two tasks whose wrong note leads somewhere. On
  `relay-tax-with-rate` every arm is 10 of 10. The wrong note there does not lead Haiku into the
  wrong edit (secondary 4: 0 sessions in any arm), so that task cannot separate the arms.
- **Secondary 3, superseded records, from the store.** In 15 of 30 `av-phase0` sessions the agent
  wrote a record that supersedes its task's wrong note, and in 1 session one that supersedes a
  true note. `av-before` has 0, as expected: build A's CLI has no `--supersedes`. All 30
  `av-phase0` sessions were accepted, both the 15 that superseded and the 15 that did not.
- **Secondary 4, the wrong edit** (the task's `[[dead_end]]` pattern). `av-before` wrote it in
  18 sessions (9 on quote-split, 9 on bus-resubscribe), `notes` in 15, and `av-phase0` and
  `cold` in none. Every session that wrote it was not accepted.
- **Secondary 5.** Of the `av-phase0` records whose text matches the wrong edit, 10 supersede
  the wrong note: corrections that name the wrong call. `av-before` has 0 of those and 22 that
  do not supersede. That count is an upper bound on restatements of the wrong call.
- **Secondary 6, delivery.** Every `notes` and `av` session got its task's wrong note: 30 of 30
  in each arm. The `av` arms got it in the session-start hook, and every `av-phase0` session also
  got Phase 0's line in the pack. `cold` got none.
- **Secondary 8, last edits** (`checks/haiku-content.txt`). Of the sessions that were not
  accepted, 29 of 37 last wrote the note's wrong call into the task's file: `av-before` 16 of
  19, `notes` 13 of 18. The others ended with an import block, an empty file or a partial
  method.
- **Cost.** `av-phase0` cost about 8 % more than `av-before` ($0.1859 against $0.1726), and less
  per accepted task ($0.0062 against $0.0157).

The leak check (`checks/haiku-leak.txt`, `exposed.json`) found no flagged call in 150 sessions
and 0 exposed sessions. The hidden-test scan (`checks/haiku-content-hidden.txt`) found 0 hits.
The tables without exposed sessions are therefore the same.

## What ran on Haiku 5.5

| command | sessions | rows | harness figure |
|---|---|---|---|
| 0. smoke, build B (`smoke-b/`) | 3 | 3 `ok` | $0.0210 |
| 0. smoke, build A (`smoke-a/`) | 1 | 1 `ok` | $0.0079 |
| 1. `cold` (`m0-cold/`), 00:15 to 00:21 UTC | 30 | 30 `ok` | $0.1039 |
| 2. `av-before`, build A (`m0-av-before/`), 00:21 to 00:39 | 30 | 30 `ok` | $0.1726 |
| 3. `notes` + `av-phase0`, build B (`m0-wrong/`), 00:39 to 01:07 | 60 | 60 `ok` | $0.3213 |
| 4. guard, build B (`m0-guard/`), 01:07 to 01:29 | 30 | 30 `ok` | $0.2343 |
| **total** | **154** | | **$0.8610** |

Commands 1 to 3 ran back to back from one chain (`haiku-chain.sh`), which would have stopped
after command 1 had the gate been missed. No session was skipped for budget, and every row is
valid. The store exports (`superseded.jsonl`, `records.jsonl`) were written after command 3 and
before command 4, from the Haiku databases only.

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

## What ran on DeepSeek (described only, under `deepseek/`)

| command | sessions | rows | harness figure | balance before → after |
|---|---|---|---|---|
| 0. smoke, build B | 3 | 3 `ok` | $0.0539 | $1.15 → $1.14 (with smoke A) |
| 0. smoke, build A | 1 | 1 `ok` | $0.0194 | |
| 1. `cold` | 30 | 30 `ok` | $0.2592 | $1.14 → $1.01 |
| 2. `av-before`, build A | 30 | 30 `ok` | $0.3166 | $0.99 → $0.84 |

On DeepSeek the gate passed 30/30. Command 2 was accepted 1 of 10 on `relay-bus-resubscribe`, 0
of 10 on `relay-quote-split` and 10 of 10 on `relay-tax-with-rate`. That is close to Haiku's
`av-before` (11/30), but it is a description only. Command 3 never ran on DeepSeek, so none of
these rows has a pair to be compared with. The store export is
`deepseek/m0-av-before/superseded.jsonl` (0 rows) and `records.jsonl` (44 rows). The DeepSeek
account keeps $0.84, which this run leaves unspent.

The DeepSeek run paused at 23:48 UTC because the balance did not cover commands 2 and 3 above the
registered $0.50 floor. The owner waived the floor, command 2 ran, and the owner then moved the
run to an Anthropic key (amendment 1). Command 3 was stopped before it started.

## Deviations

- **The run moved to Claude Haiku 5.5** after DeepSeek's command 2 (`AMENDMENT-1-haiku.md`). The
  Haiku run tree differs from `7dd5208` only in the `model` line of 13 task files
  (`haiku-tree.sh`). Prices are Haiku's list prices, the stores are fresh, and the $0.50 balance
  floor is dropped. M1 Part 2 and M2 stage B copy M0's model, so they run on Haiku 5.5. No
  result here compares with the earlier DeepSeek cells: stage 2's `relay-quote-split` numbers
  came from another model as well as another pool.
- **`analyze.py` prints "Guard not assessed: 0 valid sessions of 30".** That line is a bug in the
  script as committed. `main()` passes `decision()` the rows of the three M0 tasks, with the
  guard filtered out (`m0`, line 311), so the guard check there sees no rows. The guard table
  the same script prints, read from all rows, has 30 valid and 29 accepted. The registered rule
  ("of 30 valid sessions, … accepted at least 27") applied to that table is "the guard holds".
  The script is left as committed. The test before the run used stage 2's data, which has no
  guard, so the bug did not show.
  The fix landed later, in its own pull request (branch `m0-analyze-guard-fix`): `decision()`
  now reads the guard from all rows, and on these rows the script prints "Guard holds: 29 of 30".
- **A leak scan ran once too early**, while the guard was at 16 of 30. It read no accepted
  count, and its output was discarded. The committed scan ran after the guard's last session.
- **The DeepSeek run's deviations**: the owner waived the floor, and the preflight refused twice
  (`checks/m0-cold.refused.log`), before any session each time. That was because `first-packs`
  and the smokes left run directories open. They were closed, and the same command ran again.
  `first-packs` ran without `ISO`.

## Second model: Sonnet 5.5 (amendment 2)

`AMENDMENT-2-sonnet.md` registered this run before any Sonnet session. It is M0's primary pair
only, on `claude-sonnet-5-5`: `av-before` on build A and `av-phase0` on build B, 3 tasks × 10
repetitions each, in one run, under a $7.00 ceiling. The amendment, with its Build section, was
committed as `95b22e6` and pushed at 19:02:47 UTC. The smoke's numbers and the resulting cap were
committed as `2a33951` and pushed at 19:05:39 UTC, before the first arm session. No accepted count
was read, and no leak scan of a session was run, before both arms had ended.

### Result (`checks-sonnet/analyze.txt`, `sonnet_analyze.py` as committed)

| arm | valid | accepted | 95 % Wilson | cost | per accepted | median session |
|---|---|---|---|---|---|---|
| `av-before` (build A) | 30 | 10 | 0.192 to 0.512 | $1.7065 | $0.1706 | $0.0564 |
| `av-phase0` (build B) | 30 | 30 | 0.886 to 1.000 | $2.0862 | $0.0695 | $0.0699 |

**Outcome under the claim rule: Phase 0 helps.** `av-phase0`'s lower end, 0.886, is above
`av-before`'s upper end, 0.512. Both arms have 30 valid sessions, 10 per task, so the rule
applies. The intervals are `margincode_bench::stats::wilson_interval`'s (`checks-sonnet/wilson.txt`,
compiled against the run tree's `stats.rs`), and they match `analyze.py`'s copy of the formula.
Accepted means `success` true with `status == "ok"`. Every row is `ok`, and no session was skipped
for budget.

The registered prediction was that a stronger model might see through the note unaided, so that
`av-before` would be high and the outcome "not shown at this size". That did not happen.

Per task, accepted of 10:

| task | `av-before` | `av-phase0` |
|---|---|---|
| `relay-quote-split` | 0 | 10 |
| `relay-bus-resubscribe` | 0 | 10 |
| `relay-tax-with-rate` | 10 | 10 |

- **Where the effect is.** It is on the same two tasks as on Haiku. On `relay-tax-with-rate` both
  arms are 10 of 10, and no session of either arm wrote that task's wrong edit.
- **The wrong edit** (secondary 4). `av-before` wrote it in 20 sessions, every one of them on
  quote-split and bus-resubscribe, and none of the 20 was accepted. `av-phase0` wrote it in none.
- **Sessions that used `--supersedes`**, read from the store (`m0s-phase0/superseded.jsonl`).
  - `av-phase0`: in 28 of 30 sessions the agent wrote a record that supersedes its task's wrong
    note. Two sessions superseded another task's wrong note, and one superseded a true note. In
    the transcripts, 29 sessions ran `av record` naming `--supersedes`, and in 28 the daemon
    accepted a correction of the task's wrong note. All 30 were accepted, both the 28 that
    corrected the note and the 2 that did not.
  - `av-before`: the store holds 0, as expected, since build A's CLI has no `--supersedes`. Two
    `relay-tax-with-rate` transcripts (r3 and r10) count as "tried". In each, the agent piped a
    one-decision batch to `av record --json -`, and the record's summary text said "supersedes
    rec:…" or "This supersedes the earlier note". The batch had no `supersedes` field. Both calls
    were refused for a summary over 200 characters. Each retry recorded a plain decision with a
    shorter summary and no `supersedes` field.
- **Records that restate the wrong edit** (secondary 5). Of `av-phase0`'s records whose text
  matches the wrong edit, those in 21 sessions supersede the wrong note: corrections that name the
  wrong call. In 12 sessions such a record does not supersede it. `av-before` has 0 of the first
  kind and 25 of the second, an upper bound on restatements.
- **Delivery** (secondary 6). Every session of both arms got its task's wrong note whole in the
  session-start hook, 30 of 30 each. Every `av-phase0` session also got Phase 0's line in the pack.
- **Leak check** (`checks-sonnet/leak.txt`, `checks-sonnet/exposed.json`): 60 sessions scanned,
  0 exposed. The tables without exposed sessions are the same.

**Beside Haiku, as a description only** (the two models' rows are not pooled or tested against each
other): Haiku's pair was `av-before` 11/30 and `av-phase0` 30/30, with the same per-task pattern
apart from one accepted `av-before` session on bus-resubscribe. Per session, Sonnet cost about ten
times as much as Haiku ($0.056 to $0.070 median, against $0.006).

### What ran

| command | UTC | sessions | rows | harness | Claude Code | budget |
|---|---|---|---|---|---|---|
| smoke, build A (`smoke-s-a/`) | 19:03:10 to 19:03:47 | 1 | 1 `ok` | $0.0704 | $0.0704 | $0.50 |
| smoke, build B (`smoke-s-b/`) | 19:03:52 to 19:04:30 | 1 | 1 `ok` | $0.0642 | $0.0642 | $0.50 |
| `av-before`, build A (`m0s-av-before/`) | 19:05:48 to 19:21:57 | 30 | 30 `ok` | $1.7065 | $1.7065 | $6.86 |
| `av-phase0`, build B (`m0s-phase0/`) | 19:21:57 to 19:39:23 | 30 | 30 `ok` | $2.0862 | $2.0862 | $5.15 |
| **total** | | **62** | | **$3.9273** | **$3.9273** | ceiling $7.00 |

- The smokes passed (`AMENDMENT-2-sonnet.md`, "The smoke's numbers"). By the registered rule, the
  session cap for the arms was $0.22 and the projection $4.39. The arms ran as `m0schain.sh arms
  0.22 7.00 1`, back to back from one chain (`checks-sonnet/chain.log`).
- No session reached its cap. The most expensive cost $0.0905. The largest single prompt was
  27,124 tokens, and every assistant message was from `claude-sonnet-5-5`
  (`checks-sonnet/spend.txt`). The harness figure at the list prices matched Claude Code's
  `total_cost_usd` in every session.
- The store exports were written after both arms ended, from the Sonnet databases (`m0sa`,
  `m0sb`) only. Then came the leak scan, then the analysis.
- Builds A and B were not in the container and were rebuilt. All eight binaries are byte for byte
  the recorded ones (amendment 2, Build).

### Deviations

- **`check_set.py` on the Sonnet tree** reports its three byte-for-byte spec comparisons as BAD.
  They differ by exactly the `model` line that `sonnet-tree.sh` changes, and on the `7dd5208` tree
  the script prints "M0 set ok" (`checks-sonnet/check-set.txt`). This was known and written down
  in the Build section before the run. It is listed here because the free check does not print "ok"
  on the run tree as it stands.
- **Harness binary.** Each arm ran with its own build's `scholion-bench` (A's for `av-before`, B's
  for `av-phase0`), as `PREREG.md`'s commands and `m0hcmd.sh` did. This was registered in the
  amendment ("Copied from amendment 1"). The brief for this run named build B's harness for both
  arms.
- **A leak-scan load test** ran once before the first paid session, on zero sessions (no Sonnet
  run directory existed yet). It read no row, and its empty `exposed.json` was deleted. The
  committed scan ran after both arms had ended.
- **Delivery was checked after the fact.** `PREREG.md` asks for delivery to be checked on each
  note arm's first session before the rest is read as valid. Here it was computed for all 60
  sessions in the same analysis pass that read the accepted counts. It came out 30 of 30 in both
  arms, so no session is affected.
- The paid-session times above are the commands' bounds from `chain.log`, not per-session
  timestamps.
- Nothing else differs from the amendment: no session was rerun, skipped or edited, and no arm or
  repetition was added.

### Scope

Three tasks, one model (Claude Sonnet 5.5), one repository (tallyho), ten repetitions per task.
The comparison is the primary pair only. With no `cold` arm and no gate, it is not known whether
Sonnet solves these tasks unaided. With no `notes` arm, nothing here compares `av` with a plain
notes file on Sonnet. With no guard, nothing here shows that Phase 0's line costs nothing on
true-lesson tasks on this model. Thirty sessions over three tasks are not thirty tasks, and all
of the difference is on two of them.

## Files

- `PREREG.md` (with "Build"), `AMENDMENT-1-haiku.md`. The scripts `analyze.py`, `leak.py` and
  `content.py` were committed with the Build section. `haiku-tree.sh`, `m0hcmd.sh` and
  `haiku-chain.sh` are the Haiku run's tree and command scripts.
- `smoke-a/`, `smoke-b/`, `m0-cold/`, `m0-av-before/`, `m0-wrong/`, `m0-guard/`: what the harness
  wrote on Haiku. `m0-av-before/` and `m0-wrong/` also hold the store exports.
- `deepseek/`: the DeepSeek sessions, described only.
- `checks/`: the free checks, the harness logs, `haiku-leak.txt`, `haiku-content-hidden.txt`,
  `haiku-content.txt` and `haiku-analyze.txt`. `exposed.json` is written by `leak.py`.
- `first-packs-a/`, `first-packs-b/`: the free replay.
- Sonnet run (amendment 2): `AMENDMENT-2-sonnet.md`; the scripts `sonnet-tree.sh`,
  `m0sdaemon.sh`, `m0scmd.sh`, `m0schain.sh`, `sonnet_spend.py`, `sonnet-export.sh`,
  `sonnet_leak.py`, `sonnet_analyze.py` and `sonnet-wilson.rs`; `smoke-s-a/`, `smoke-s-b/`,
  `m0s-av-before/` and `m0s-phase0/` (with the store exports) as the harness wrote them;
  `first-packs-s-a/` and `first-packs-s-b/`; `checks-sonnet/` (free checks, logs, spend, leak
  scan, analysis). Its transcripts stay in `m2b-harness` under `/m2b/m0s/bench-work/runs`, closed
  to the agent user.
- Transcripts are not committed. They stay in `lp-harness` under `/m0/bench-work/runs`, closed to
  the agent user.
