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
- **A leak scan ran once too early**, while the guard was at 16 of 30. It read no accepted
  count, and its output was discarded. The committed scan ran after the guard's last session.
- **The DeepSeek run's deviations**: the owner waived the floor, and the preflight refused twice
  (`checks/m0-cold.refused.log`), before any session each time. That was because `first-packs`
  and the smokes left run directories open. They were closed, and the same command ran again.
  `first-packs` ran without `ISO`.

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
- Transcripts are not committed. They stay in `lp-harness` under `/m0/bench-work/runs`, closed to
  the agent user.
