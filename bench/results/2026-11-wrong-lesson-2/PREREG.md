# Pre-registration: the wrong-lesson cell again, on the closed harness, 2026-11-12

Written and committed after the free checks and before the first paid session of this run. The
git commit time of this file, and of the push of the branch `wrong-lesson-rerun`, is the
evidence. Nothing below is changed after the first paid session; anything that turns out
differently is written in `README.md` as a deviation.

## Why this run

The first run (`../2026-11-wrong-lesson/README.md`) gave no clean answer. By its rule neither arm
was hurt. After the run, one task stood out: on `relay-quote-split`, where neither the code nor
the visible tests contradict the wrong note, the `av` arm was accepted 0 of 6 times and the
`notes` and `cold` arms 6 of 6. That was six sessions per arm, picked out after seeing the data,
on a harness that let sessions read grading tests (on another task). This run tests that one
observation, named in advance, at 30 sessions per arm, with the agent isolated.

## Question and hypotheses

On a task where a stored note confidently recommends an approach that fails the hidden test, and
nothing the agent can read contradicts the note, is an agent that gets the note as a Scholion
record (`av`) accepted less often than an agent that gets the same text in a notes file
(`notes`)?

- **H1 (the observation holds):** on `relay-quote-split`, `av` is accepted less often than
  `notes`.
- **H0:** it is not, at this size. The six-session difference was chance or an artefact of the
  open harness.

No hypothesis is stated about why. The arms differ in framing and in what the agent can do about
a note (below).

## Design

**Primary cell.** The task `bench/tasks-relay-wrong/quote-split.toml`, unchanged, on the pool
`bench/tasks-relay/pool/notes-wrong.toml`, unchanged (texts fixed in commit `a1ad6c2`).

| arm | sessions | what the agent gets |
|---|---|---|
| `cold` | 12 | the base tree only |
| `notes` | 30 | `NOTES.md` with all ten notes, and a `CLAUDE.md` that says to read it and follow its decisions |
| `av` | 30 | the same ten notes as anchored records of an earlier task; hooks + CLI, `--jit-hooks`, no MCP |

**Secondary cell (not part of the decision; run later, see "Order and prices").** The three new
tasks of `bench/tasks-relay-silent` on the pool `notes-silent.toml` (the ten notes above plus one
wrong note per new task). Each was shown without a model to have the property the primary task
has: the note's approach passes the repository's own tests and fails the hidden test, and a
search of the repository found nothing that contradicts the note (`bench/tasks-relay-silent/README.md`,
`check_set.py`, `reference/check.py`). Known difference: the goal of `relay-order-merge` states
the tested behaviour outright, so that note is contradicted by the task text. `cold` 6 per task,
`notes` and `av` 10 per task: 18, 30 and 30 sessions.

Common to both: no noise, run seed 1, model `deepseek-flash` through
`https://api.deepseek.com/anthropic`, Claude Code 2.1.318, no other model and no other key.

**Isolation, new in this run.** `--agent-user benchagent`: the agent CLI runs as an unprivileged
user with no home, which can read only its own working copy; the harness's preflight refuses to
start otherwise. No `--keep-work`. The bench tree, the task repository, the results and the
daemon's store are closed to that user. Still open, as `bench/README.md` ("Agent isolation")
says: the daemon answers any session about any repository; the container's network; directories
every user can write (`/tmp`); verify runs what the agent wrote as root; the agent inherits the
harness's environment.

**Build.** Harness, `av` and daemon are release builds of the commit named under "Free checks",
which is the first run's branch with `main` (the isolation, #144) and the two reviewed follow-ups
(#145: a superseded search hit takes no place under the record cap, `PACKER_VERSION` 7; #146: the
preflight covers results directories and the agent user's home). No record in this benchmark is
superseded, so #145 changes no pack here.

## Free checks, done before this file was committed

- **Build.** Commit `55afbc8` of `wrong-lesson-rerun` (the merge of `ai-work` at `ea686ec`, which
  is `main` + #146 + #145), copied into `/bench/tree` with `git archive`; the tree id of the copy
  was computed in the container and is the commit's (`a9a738d`). `cargo build --release --locked
  -p scholion-cli -p scholion-daemon -p scholion-bench`. sha256 of the binaries the run uses
  (`/bench/bin`): `av` `29b70d0c07b747f5919f36d0a0373136cf6fca3de90c50a31e172f125e599f17`,
  `scholion-bench` `b539657243a45d69ca099e8a830b31f05f0232b5b5d4711a59f20e7e7353b341`,
  `scholion-daemon` `659fb7fae363e0fbd98edddd051e39378cef8e610732ab690dd53f48aadadb7a`. Claude
  Code 2.1.318. The full test suite was not run on this merge commit:
  #146's branch passed the bar with the gate run as root (1082 passed) and each pull request passed
  CI before it was merged; CI on the merged tree is running on #147 and is not waited for.
- **Specs.** `scholion-bench check` on the build: `relay-quote-split`, `relay-order-merge`,
  `relay-bus-resubscribe`, `relay-tax-with-rate`, `relay-order-ids`: ok.
- **The wrong note reaches the av agent** (`scholion-bench first-packs`, no agent, kept as
  `first-packs-qs/` and `first-packs-silent/`): `relay-quote-split`, 10 records seeded, its wrong
  note is the first record of the session-start hook's pack, summary whole (pack 709 of 800
  tokens, 4 records, none degraded). The three silent tasks, 13 records seeded: each task's wrong
  note is in the hook pack at the summary tier, 3 of 3.
- **Isolation, checked by hand as `benchagent`** (uid 999, no home, no login): it can run
  `claude`, `av` and the venv's `pytest`; it cannot list `/bench/tree`, `/bench/results`,
  `/bench/store`, `/bench/bench-repos`, `/bench/bench-work`, its `runs`, `src` and `ws`, the
  first run's `/root` and the source mount `/src`, and cannot read the daemon's log or
  environment. `/tmp` was emptied (the first run's two copies of grading tests went with it).
  The harness's own preflight runs at the smoke.
- **Fresh state.** A new database and a new store for the daemon; the first run's daemon is not
  running; the task repository is a clone of the first run's with identical refs (`main` =
  `11c17fd`), and the venv has the same package versions (pytest 9.2.0).
- **Balance** read at 04:41 UTC on 2026-11-12: $2.67.

## Commands (in this order)

In `lp-harness`, as root, `HOME=/bench`, from `/bench/tree` (a `git archive` of the build commit;
`~` in the task specs is `/bench`), each under the subreaper, the key passed in with `docker exec
-e` and never written down:

```bash
P="--price-per-mtok 0.30,0.30,0.006,1.20"
ISO="--agent-user benchagent --av-ws-root /bench/bench-work/ws"
COMMON="--av-url http://127.0.0.1:7450 --work-root /bench/bench-work --import-dir /bench/bench-work/src --import-mount /bench/bench-work/src $ISO"
OUT=/bench/results
# 0. smoke: does an isolated session run at all? (not a result)
scholion-bench run bench/tasks-relay/order-ids.toml --arms cold,notes,av --jit-hooks --reps 1 $P \
  --session-cap-usd 0.08 --budget-usd 0.30 --out $OUT/smoke $COMMON --confirm
# 1. primary, the control
scholion-bench run bench/tasks-relay-wrong/quote-split.toml --arms cold --jit-hooks --reps 12 $P \
  --session-cap-usd 0.08 --budget-usd 0.40 --out $OUT/qs-cold $COMMON --confirm
# 2. primary, the two arms that carry the wrong note
scholion-bench run bench/tasks-relay-wrong/quote-split.toml --arms notes,av --jit-hooks --reps 30 $P \
  --session-cap-usd 0.08 --budget-usd 1.40 --out $OUT/qs-wrong $COMMON --confirm
# 3. secondary, the control
scholion-bench run bench/tasks-relay-silent --arms cold --jit-hooks --reps 6 $P \
  --session-cap-usd 0.08 --budget-usd 0.40 --out $OUT/silent-cold $COMMON --confirm
# 4. secondary, the two arms that carry the wrong notes
scholion-bench run bench/tasks-relay-silent --arms notes,av --jit-hooks --reps 10 $P \
  --session-cap-usd 0.08 --budget-usd 1.20 --out $OUT/silent-wrong $COMMON --confirm
```

A rerun of the same command resumes it. The results directories are copied into this directory
afterwards, as the harness wrote them.

**The smoke** is three isolated sessions, one per arm, on `relay-order-ids` with its true lesson.
It passes if each row is `status=ok` and valid, the event stream shows the model answered and
made at least one edit, the verify command ran, `av`'s `hook-count` is not empty, and `meta.json`
names `benchagent` under `agent_isolation` and Claude Code 2.1.318. The environment is the first
run's (`ANTHROPIC_BASE_URL`, `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_DEFAULT_HAIKU_MODEL`,
`ANTHROPIC_SMALL_FAST_MODEL` and `CLAUDE_CODE_SUBAGENT_MODEL` set to `deepseek-flash`; the specs
name the model). If it fails, the cause is fixed and
it is run again in a new directory (`smoke-2`, …); no other command starts before one smoke
passes. Smoke rows are not part of any arm.

## Order and prices

Prices as in the first run: `0.30,0.30,0.006,1.20` ($ per million tokens: input, cache write,
cache read, output), DeepSeek's peak prices. Commands 0 to 2 run now, outside the off-peak window
(16:30 to 00:30 UTC), so the bill is about the harness figure. Commands 3 and 4 wait for the
off-peak window, where the bill is about half. If they have not run when the README is written,
the README reports the primary alone and says so.

## Metrics and decision rule (fixed now)

**Primary:** accepted / valid sessions on `relay-quote-split`, per arm, with the 95 % Wilson
score interval (formula of `scholion_bench::stats::wilson_interval`, computed by a script
committed with the results).

- **H1 is reported** only if `av` and `notes` each have 30 valid sessions and the upper end of
  `av`'s interval is below the lower end of `notes`'s. (For scale: 30 of 30 has the interval
  0.886 to 1.000, and 21 of 30 has the upper end 0.83.)
- **Otherwise H0 is reported**: the observation is not shown at this size, whatever the counts,
  with both intervals.
- `cold` (12 sessions) says whether the task is still solved without any note. If `cold` is
  below 10 of 12, the task is not one "the model solves unaided" here and the README says the
  comparison is confounded, whatever the arms show.
- No claim is made for an arm with fewer than 30 valid sessions.

**Secondary, reported and not used for the decision:**

1. The same table per task for the three silent tasks, and pooled over them (30 sessions per
   note arm), with intervals. A pooled difference is described, with the per-task table beside
   it; the three tasks are not independent samples of "silent tasks" and no general claim is made
   from them.
2. Sessions that wrote the wrong edit (`repeated_dead_ends` ≥ 1), accepted and not.
3. What the agent did with the note, counted by the first run's `behaviour.py`: sessions that
   edited `NOTES.md`; `av` sessions that recorded the wrong approach again; `av` sessions that
   recorded an `attempt`. In this build an agent has no command to supersede a record (`av record`
   and the MCP tool do not expose it; the store does), which is named here as a difference
   between the arms: a notes agent can correct the file, an `av` agent cannot correct the record.
4. Delivery per session, as in the first run: `notes`, a read of `NOTES.md` in the event stream;
   `av`, the task's wrong note whole in the session-start hook's output or in `pre-read.out`.
5. Cost per accepted task and median session cost, as the harness reports them.

**Leak check, before the result is read.** The first run's `leak.py` classifies every tool call
of every session (hidden tests, other sessions' files, the source tree, the web, the daemon over
HTTP). The primary is computed on all valid sessions, as registered. Exposed sessions are listed,
and the same table is given without them. An exposed session on the primary task is a deviation
and is written up as one.

## What follows, decided now

- H1: the next step is the change already being built, measured on this same cell with the same
  rule: an agent can supersede a record (`av record --supersedes`), and the pack says a record is
  an earlier agent's claim to check against the code. It gets its own pre-registration.
- H0: that change is not justified by solved tasks on this cell. It may still be built as a
  missing capability, and the README will say that the evidence for it is the first run's six
  sessions and nothing more.

## Confounds named in advance

- The arms differ in framing: the `notes` arm's `CLAUDE.md` says to read `NOTES.md` and follow
  its decisions; the pack lists the note under "Decisions & dead ends" with no instruction.
- They differ in what can be done about a note (secondary measure 3).
- The pack shows a record's summary; the notes file shows summary and body.
- One task, one wrong note, one wording, one model. Thirty repetitions of one task are not thirty
  tasks: the interval describes this task under this note, not wrong notes in general.

## Validity and missing sessions

As in the first run: a session the harness marks `invalid` stays in `rows.jsonl` and is not
retried; a session that stops the harness with an error (no row written) is an infrastructure
failure, the cause is fixed and the same command is rerun; no row is edited, dropped or re-run by
hand. The host has crashed several times this week: a crash in the middle of a command is such
an infrastructure failure, and the rerun resumes it. Delivery is checked on the first session of
each note arm of command 2 before the rest is read as valid (measure 4); if it fails, the run
stops and nothing of that arm is reported as a result.

## Stopping rules

- `--session-cap-usd 0.08` per session; `max_turns` 30 is the real per-session limit.
- `--budget-usd` per command as written above: $0.30, $0.40, $1.40, $0.40, $1.20.
- **Balance floor $0.50.** The balance is read before command 0 and after command 2, and again
  around commands 3 and 4. Before each command its budget is lowered to what the balance leaves
  above the floor, if that is less. An arm that runs out of budget is reported as partial, never
  extrapolated.

## Limits known in advance

- Everything under "Confounds".
- The isolation closes file-system routes only (see "Isolation").
- The first run's `cold`, `notes` and `av` rows are not pooled with this run's.
