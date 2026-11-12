# Pre-registration: what a confident wrong note does, 2026-11-11

Written and committed after the free replay and before the first paid session. The git commit
time of this file, and of the push of the branch `wrong-lesson-cell`, is the evidence. Nothing
below is changed after the first paid session; anything that turns out differently is written in
`README.md` as a deviation.

## Question and hypotheses

From `DESIGN.md`: on five tasks the model solves without any lesson, does a stored note that
confidently recommends the approach that fails lower the rate of solved tasks, and does it do so
alike for a flat `NOTES.md` (arm `notes`) and for Scholion's anchored records (arm `av`)?

- **H1:** a wrong note hurts. An arm that carries it solves fewer tasks than `cold`.
- **H0:** it does not, at this size. The model weighs the note against the code and the tests and
  both arms stay within `cold`'s interval. That is then reported as the result, and it means this
  task set cannot show an outcome gain from a feature that corrects notes.

No hypothesis is stated about which arm is hurt more. The arms differ in framing (below), so a
difference between them is reported and not explained.

## Design

- Task set `bench/tasks-relay-wrong`: the five relay tasks whose lesson is a dead end, on the pool
  `bench/tasks-relay/pool/notes-wrong.toml` (five wrong notes as decisions, `no_commit`; the five
  true decision lessons unchanged). The texts were fixed in commit `a1ad6c2`, before any session.
- Arms: `cold` (base tree only, no notes, no records: the control), `notes` (`NOTES.md` of all ten
  notes and a `CLAUDE.md` that says to read it and follow its decisions), `av` (the same ten notes
  as anchored records of an earlier task; hooks + CLI, `--jit-hooks`, no MCP).
- No noise (`--noise` not passed). Run seed 1 (the default).
- Reps 6 × 5 tasks = **30 sessions per arm**, 90 sessions.
- Model `deepseek-flash` through `https://api.deepseek.com/anthropic`, Claude Code 2.1.318. No
  other model, no other API key.
- Build: harness, `av` and daemon are release builds of this branch. `crates/*/src` at the commit
  that adds this file is what was built; later commits before the run touch only files under
  `bench/results/`. The packer is the one of pull request #142 (`PACKER_VERSION` 6, changes A
  and B of `../2026-11-first-pack/`), which is not on `main` yet. The `av` arm therefore runs on
  the packer that delivers a task's own note best, which is the case this cell is about.

## Free checks, done before this file was committed

- `python3 bench/tasks-relay/reference/check.py`: all 10 pairs ok (the reference edit passes the
  hidden test, the dead-end edit fails it). Each wrong note recommends that dead-end edit.
- `scholion-bench check bench/tasks-relay-wrong`: all 5 tasks ok.
- `cargo test -p scholion-bench`: passes, with the tests that pin the pool (only the five notes
  differ from `notes.toml`, same pair, path and symbol) and that a `no_commit` note is in
  `NOTES.md` and in no commit message.
- **The wrong note reaches the av agent.** `scholion-bench first-packs bench/tasks-relay-wrong`
  (`first-packs/`): the task's own wrong note is in the session-start hook's pack at the summary
  tier, whole, for **5 of 5** tasks; 7 of 7 relevant files named; no pack degraded. DESIGN's bar
  was 4 of 5.
- `python3 bench/results/2026-11-wrong-lesson/patterns.py`: each task's `[[dead_end]]` pattern
  matches the text of its pair's dead-end edit and none of the text of its reference fix (5 of 5).
- Commands 1 and 2 below without `--confirm` printed their plans: `cold` 30 sessions, worst case
  $2.40 at the cap; `notes,av` 60 sessions (30 per arm), worst case $4.80 at the cap.

## Commands (in this order)

```bash
P="--price-per-mtok 0.30,0.30,0.006,1.20"
OUT=bench/results/2026-11-wrong-lesson
COMMON="--av-url http://127.0.0.1:7450 --work-root /root/bench-work --import-dir /root/bench-work/src --import-mount /root/bench-work/src"
# 0. smoke: does a session run at all in this environment? (not a result)
scholion-bench run bench/tasks-relay/order-ids.toml --arms cold,notes,av --jit-hooks --reps 1 $P \
  --session-cap-usd 0.08 --budget-usd 0.30 --keep-work --out $OUT/smoke $COMMON --confirm
# 1. the control
scholion-bench run bench/tasks-relay-wrong --arms cold --jit-hooks --reps 6 $P \
  --session-cap-usd 0.08 --budget-usd 0.60 --keep-work --out $OUT/cold $COMMON --confirm
# 2. the two arms that carry the wrong note
scholion-bench run bench/tasks-relay-wrong --arms notes,av --jit-hooks --reps 6 $P \
  --session-cap-usd 0.08 --budget-usd 2.00 --keep-work --out $OUT/wrong $COMMON --confirm
```

Each command runs on its own, one after the other; a rerun of the same command resumes it.
`--keep-work` keeps each session's copy of the repo, so what the agent was given can be checked
afterwards; the sweep did not pass it, and it changes nothing a session sees.

### The smoke (command 0)

No agent session has run in this container before: the agent CLI, Node, Python 3.13 and the venv
were installed today. The rules below forbid dropping or re-running rows, so a broken environment
would be frozen in as failed sessions. The smoke is three sessions, one per arm, on
`relay-order-ids`: a task of the original relay set with its **true** lesson, which is not one of
this cell's five tasks and carries no wrong note. It says nothing about the question.

It passes if, for each arm, the row is `status=ok` and valid, the event stream shows the model
answered and made at least one edit, and the verify command ran (accepted or not; `cold` usually
fails this task). For `av`, `hook-count` is not empty. `meta.json` names Claude Code 2.1.318. If
the smoke fails, the cause is fixed and the smoke is run again in a new directory (`smoke-2`, …);
commands 1 and 2 do not start until one smoke passes. Smoke rows are reported in the README and
are not part of any arm.

## Metrics

Primary, per arm:

1. **accepted / 30** (the task's hidden tests pass after the session), with the 95 % Wilson score
   interval, computed outside the report with the formula of
   `scholion_bench::stats::wilson_interval`.

Secondary, reported and not used for the decision:

2. **Sessions that wrote the wrong edit**, for all three arms: sessions whose
   `repeated_dead_ends` is 1 or more (an edit to the task's file matched the spec's
   `[[dead_end]]` pattern, which is the edit the wrong note recommends), split into accepted and
   not accepted. The wrong edit is also the obvious first try, so `cold` sessions will write it
   too. Only the excess over `cold` is put down to the note, and with no interval it is a
   description, not a finding.
3. **Delivery, per session**: for `notes`, sessions whose event stream shows a read of
   `NOTES.md`; for `av`, sessions where the task's wrong note is whole in the session-start
   hook's output in the event stream or in `pre-read.out`. Counted by a script committed with the
   results. Sessions without delivery stay in their arm (the arm is "this memory system holding a
   wrong note", not "an agent that read it"); the count says how much of the arm was exposed.
4. Cost per accepted task, median session cost with the harness's bootstrap interval, median
   turns, as the harness reports them.

## Decision rule (fixed now)

Let `cold` be c / 30 with Wilson interval [L, U].

- An arm **is hurt by the wrong note** if its accepted rate lies below L. (If `cold` is 30 / 30,
  L is 0.886, so an arm is hurt at 26 / 30 or fewer.)
- An arm **is not shown to be hurt** if its rate is L or above. Thirty sessions cannot show a
  small effect; the README says so with the interval.
- **The arms differ** only if their Wilson intervals do not overlap. Otherwise no claim is made
  about `av` against `notes`, whatever the counts.
- H1 holds if at least one arm is hurt; H0 is reported if neither is.

No claim of a difference is made for an arm with fewer than 30 valid sessions.

What follows from each outcome, decided now: if an arm that carries records (`av`) is hurt, the
next step is a visible mark on records that come from reverted work, measured on this same cell.
If neither arm is hurt, that feature is not justified by solved tasks on this task set, and the
secondary measures 2 and 4 say whether it could still be justified by cost.

## Confound named in advance

`notes` and `av` do not only differ in where the note lives. The `notes` arm's `CLAUDE.md` tells
the agent to read `NOTES.md` and follow its decisions. The av pack lists the note under
"Decisions & dead ends" with no instruction to obey it. A difference between the arms may come
from that framing. This cell does not separate the two, and the README will not say it does.

## Validity and missing sessions

- A session the harness marks `invalid` stays in `rows.jsonl` as written and is not retried. It is
  listed in the README and the arm is reported with its number of valid sessions.
- A session that stops the harness with an error (no row written) is an infrastructure failure:
  the cause is fixed and the same command is rerun, which resumes it.
- No row is edited, dropped or re-run by hand. `rows.jsonl`, `meta.json` and `report.md` are
  committed as the harness wrote them.
- Delivery is checked on the first session of each arm of command 2 before the rest is read as
  valid. `notes`: the kept copy's `NOTES.md` holds the task's wrong note and its `git log` does
  not. `av`: `status=ok`, a non-empty `hook-count`, and the task's wrong note whole in the
  session-start hook's output in `events.jsonl` (the `pack.md` beside it is the task-start pack,
  built before the records are embedded, and is not what the agent reads; the free replay shows
  it lacks the note in 3 of 5 tasks). If a check fails, the run is stopped, the cause is written
  down, and nothing of that arm is reported as a result.

## Stopping rules

- `--session-cap-usd 0.08` per session; `max_turns` 30 is the real per-session limit, as in the
  sweep.
- `--budget-usd` $0.30 for the smoke, $0.60 for `cold` and $2.00 for `notes,av`. E4 at scale
  spent $0.39 on 30 `cold` sessions and $0.75 on 60 `notes` and `av` sessions at no noise; the
  larger budget leaves room for sessions that follow the wrong note and take a detour. The
  harness orders sessions by repetition, then task, then arm (shuffled), so a budget cut would
  fall on the last repetitions of both arms, not on one arm.
- **Balance floor $0.50.** The balance on 2026-11-11 at 17:05 UTC was **$3.30**. If it drops
  below $0.50 the run stops. The three budgets sum to $2.90, more than the $2.80 above the
  floor, so before command 2 the balance B is read and its budget is `min(2.00, B - 0.50)`. An
  arm that runs out of budget is reported as partial, never extrapolated.
- The balance is read before the run and a few minutes after it.

## Prices

The sweep's peak prices `0.30,0.30,0.006,1.20` (input, cache write, cache read, output; $ per
million tokens). The run starts after 17:00 UTC, inside DeepSeek's off-peak window (16:30 to
00:30 UTC), so the harness figure is an upper bound on the bill. The balance delta is reported
beside it.

## Differences from the E4-at-scale setup, known now

- One container (`lp-harness`, `rust:1.97.1`, Debian 13) holds the harness, `av`, the agent CLI
  and the daemon (`127.0.0.1:7450`); the sweep had the daemon in its own container. Postgres 16,
  Qdrant v1.13.6 and TEI cpu-1.7.2 with `BAAI/bge-small-en-v1.5` are as in the sweep.
- Python 3.13.5 with pytest 9.2.0 (the sweep: Python 3.11.2, pytest 9.2.0); Node 22.23.5.
- The container has no init process; the harness runs under a small subreaper script.
- `/src` is the host worktree, whose `.git` file points at a Windows path, so `meta.json`'s
  `av_rev` will read `unknown`. The commit is named in the README instead.
- `cold` is run again here, on this build and this environment, so the three arms share both.
  E4 at scale's `cold` (15 of 15 on these five tasks) is not pooled with it.

## Limits known in advance

- Five tasks, one repository, one model, one wording per wrong note, no noise.
- A note that is wrong from the start, alone, with no correct note beside it.
- Six repetitions of a task are not independent tasks. The Wilson interval treats the 30 sessions
  as independent, so it is narrower than the truth; the per-task table is reported beside it.
