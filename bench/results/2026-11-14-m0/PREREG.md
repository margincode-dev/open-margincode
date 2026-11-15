# Pre-registration: M0, correction on three tasks with a silent wrong note, 2026-11-14

**Prepared, not run.** This file fixes the question, the arms, the bars and the commands before
any session of M0. Nothing here has been run. Every session needs the owner's go, the owner's
machine and an API key. When the run starts, its two builds and its stack are written under
"Build" and committed before the first paid session. Anything that turns out differently is a
deviation in `README.md`, not an edit here.

Source: Plan 16 (`margincode-cloud`, `docs/superpowers/plans/2026-11-12-agentvcs-plan-16-hivemind-memory.md`),
"Measurement, M0" under Phase 0. The arms, the primary comparison, the guard and its bar are
copied from there. No bar is moved after a result is seen.

## Why this run

Phase 0 (#148, commit `ca625d1`) lets an agent correct a record (`av record --supersedes rec:…`)
and adds a line to the pack: records are earlier agents' claims, to be checked against the code.
Stage 2 of the wrong-lesson cell (`../2026-11-wrong-lesson-3/README.md`) measured it on the one
task it was found on, `relay-quote-split`. There `av` was accepted 12 of 30 (0.246 to 0.577),
against 2 of 30 on the build without it. But the two builds were compared across two runs, on
one task. M0 does what stage 2 could not: both builds run in one run, on three tasks of that
kind.

## Question and outcomes

**Q. On tasks whose wrong note nothing in the repository contradicts, is `av` with Phase 0
accepted more often than `av` without it?**

- **Phase 0 helps:** the lower end of the Phase 0 arm's interval is above the upper end of the
  arm without it.
- **Phase 0 hurts:** the upper end of the Phase 0 arm's interval is below the lower end of the arm
  without it.
- **Otherwise: not shown at this size.** Both intervals are given. Nothing is claimed, and the
  words "equal" and "no effect" are not used.

The plan's rule: "A difference is claimed only if the Wilson intervals are disjoint."

A prediction, for the record: stage 2 suggests a gain on `relay-quote-split`. On the two other
tasks nothing the agent reads contradicts the note either, so the line's "check against the
code" may find nothing to object to.

## Tasks

`bench/tasks-m0` ([`../../tasks-m0/README.md`](../../tasks-m0/README.md)): `relay-quote-split`,
`relay-bus-resubscribe`, `relay-tax-with-rate`. All three use the pool
`bench/tasks-relay/pool/notes-silent.toml` (thirteen notes, eight wrong). Each task's own note
is a confident wrong decision. With the approach it recommends, the repository's own tests pass
and only the hidden test fails (`bench/tasks-m0/check_set.py`).

No task was written for M0. The plan asks for "two new tasks of its kind". Two of the three
silent tasks from the second wrong-lesson run are that, and have never had a paid session
(`../2026-11-wrong-lesson-2/README.md`: "The secondary cell has not run"). The third,
`relay-order-merge`, is left out because its goal states what its hidden test checks. That is
the owner's decision of 2026-11-14.

## Arms and builds

Two builds of this repository's history. Both have the names before the rename (`scholion-bench`,
`scholion-daemon`, `av`):

- **Build A, `av` before Phase 0:** commit `a67f130`, the parent of `ca625d1`: `main` just
  before #148. It is #149, the wrong-lesson cell's results plus the harness's `no_commit` notes,
  which the pool needs. Builds of an earlier commit would put the wrong notes into `git log`.
  The second wrong-lesson run's build, `55afbc8` (a merge of `ea686ec` with that branch), is not
  in this repository's history.
- **Build B, `av` with Phase 0:** commit `90ba62e`, stage 2's build: `ca625d1` (#148) plus #150.
  #150 changes the harness's housekeeping (a refused `--agent-user` preflight cleans up, a run
  closes its directory, results files are private), not what a session is given. It is not
  today's `main`, which also has Phase 1 (standing, `84bb472`) and Phase 2's verdicts
  (`3f9e159`). Those would change the `av` arm beyond Phase 0, and M1 Part 2 measures Phase 1 on
  its own.

| arm | build | sessions | what the agent gets |
|---|---|---|---|
| `cold` | B | 30 | the base tree only |
| `notes` | B | 30 | `NOTES.md` with all thirteen notes, and a `CLAUDE.md` that says to read it and follow its decisions |
| `av-before` | A | 30 | the thirteen notes as anchored records of an earlier task; hooks + CLI, `--jit-hooks`, no MCP; no correction line, no `--supersedes` |
| `av-phase0` | B | 30 | the same records, plus the line under "Decisions & dead ends" and `av record --supersedes` |

Each arm is 3 tasks × 10 repetitions. The harness calls both `av` arms `av`. They are told apart by
their results directory (`m0-av-before/`, `m0-wrong/`). `cold` and `notes` get nothing Phase 0
touches, so they run once, on build B.

**Guard** (build B): the ten true-lesson relay tasks, `bench/tasks-relay` on `pool/notes.toml`,
N = 0, arm `av`, 3 repetitions each: 30 sessions.

**Optional fifth arm, not run.** The plan names one: Phase 0 plus a reworded Stop ask ("Not to
build" 2). No build has that wording. Building it would make a third build in this run, so it
is left out.

**Common to every session:** model `deepseek-flash` through
`https://api.deepseek.com/anthropic`, no other model and no other key; Claude Code at the
version named under "Build" (stage 2 used 2.1.318, and the same is used if it is still
installable); `--agent-user benchagent`; no `--keep-work`; no noise; run seed 1; prices
`--price-per-mtok 0.30,0.30,0.006,1.20` (DeepSeek's peak prices, as in every earlier cell).

## Free checks at the start of a run (before the first paid session, no model)

1. **The bar on this file's commit:** `cargo fmt --all --check`, `cargo clippy --workspace
   --all-targets -- -D warnings`, `cargo test --workspace`. This was done when this file was
   written (the PR's description has the results). Each build commit is taken as it was merged.
2. **The task set:** `bench/tallyho/make-repo.sh`; `reference/check.py` (all 13 pairs ok);
   `bench/tasks-relay-silent/check_set.py`; `bench/tasks-m0/check_set.py <tallyho> <venv python>`
   (M0 set ok, all three silent).
3. **The specs on each build:** `scholion-bench check bench/tasks-m0` and `scholion-bench check
   bench/tasks-relay`, with build A's harness and with build B's: ok. The task files are read from
   this file's tree, not from the build's.
4. **The replay** (`first-packs`, one daemon per build, fresh state each):

   ```bash
   $A/scholion-bench first-packs bench/tasks-m0 --out $OUT/first-packs-a $COMMON_A
   $B/scholion-bench first-packs bench/tasks-m0 --out $OUT/first-packs-b $COMMON_B
   ```

   It passes if, for each of the three tasks on each build, the task's wrong note is in the
   session-start pack, summary whole, with its `rec:` handle. On build B the line ("Claims by
   earlier agents: check against the code; correct wrong ones with `av record --supersedes
   rec:…`") must stand under "Decisions & dead ends". On build A it must be absent. If any of
   these fails, no paid session starts, and the replay is reported as it is. For
   `relay-quote-split` on build B the pack is set beside stage 2's
   (`../2026-11-wrong-lesson-3/first-packs-qs/`), which had four records with the wrong note first.
   Any difference is reported. The pool here has three more notes, so a difference does not stop
   the run.
5. **Isolation and fresh state,** as stage 2's PREREG lists them: the agent user, the modes of
   the bench tree, the results, the stores and the earlier runs' directories, `/tmp` emptied, a
   new database and store per daemon, one task-repository clone and one venv.
6. **Balance** read before command 0 and written down.

## Commands (in this order)

In one harness container, as root, from a `git archive` of this file's commit. `$A` and `$B` are
the two builds' `bin` directories. Each build has its own daemon on its own port, with its own
database and store. The key is passed in at `docker exec` and never written down.

```bash
P="--price-per-mtok 0.30,0.30,0.006,1.20"
ISO="--agent-user benchagent --av-ws-root <ws root>"
COMMON_A="--av-url http://127.0.0.1:<port A> --av-bin $A/av <work-root and import options> $ISO"
COMMON_B="--av-url http://127.0.0.1:<port B> --av-bin $B/av <work-root and import options> $ISO"
OUT=<results root>
# 0. smokes: does an isolated session run at all on each build? (not a result)
$B/scholion-bench run bench/tasks-relay/order-ids.toml --arms cold,notes,av --jit-hooks --reps 1 $P \
  --session-cap-usd 0.08 --budget-usd 0.30 --out $OUT/smoke-b $COMMON_B --confirm
$A/scholion-bench run bench/tasks-relay/order-ids.toml --arms av --jit-hooks --reps 1 $P \
  --session-cap-usd 0.08 --budget-usd 0.10 --out $OUT/smoke-a $COMMON_A --confirm
# 1. the control
$B/scholion-bench run bench/tasks-m0 --arms cold --jit-hooks --reps 10 $P \
  --session-cap-usd 0.08 --budget-usd 0.40 --out $OUT/m0-cold $COMMON_B --confirm
# 2. av before Phase 0
$A/scholion-bench run bench/tasks-m0 --arms av --jit-hooks --reps 10 $P \
  --session-cap-usd 0.08 --budget-usd 0.70 --out $OUT/m0-av-before $COMMON_A --confirm
# 3. notes and av with Phase 0
$B/scholion-bench run bench/tasks-m0 --arms notes,av --jit-hooks --reps 10 $P \
  --session-cap-usd 0.08 --budget-usd 1.40 --out $OUT/m0-wrong $COMMON_B --confirm
# 4. the guard
$B/scholion-bench run bench/tasks-relay --arms av --jit-hooks --reps 3 $P \
  --session-cap-usd 0.08 --budget-usd 0.60 --out $OUT/m0-guard $COMMON_B --confirm
```

A rerun of the same command resumes it. The results directories are copied into this directory
afterwards, as the harness wrote them. After commands 2 and 3, the `records` rows of each
daemon's database whose `supersedes` is set are exported to `m0-av-before/superseded.jsonl` and
`m0-wrong/superseded.jsonl` (secondary measure 3). Each row has its id, task, repository,
author, kind, summary and `supersedes`.

**The smokes** are stage 2's smoke: isolated sessions on `relay-order-ids` with its true lesson.
They pass if each row is `status=ok` and valid, and the event stream shows that the model answered
and made at least one edit. The verify command must have run, `av`'s `hook-count` must not be
empty, and `meta.json` must name `benchagent` under `agent_isolation` and the Claude Code
version. If a smoke fails, the cause is fixed and it runs again in a new directory (`smoke-b-2`,
…). No later command starts before both smokes pass. Smoke rows are not part of any arm.

**The gate after the control.** Commands 2 to 4 start only if `cold` is accepted at least 8 of
10 times on each of the three tasks (stage 2's bar, 10 of 12, scaled down and rounded down). A
task below that is not one the model solves unaided here. The README then reports the control's
rows, and no note arm runs on that task. A replacement task needs an amendment, committed
before any note-arm session.

**Order.** Commands 2 and 3 run back to back, with nothing else between them, in the same
DeepSeek price window. Both run at peak prices unless the owner moves the whole run off-peak. The
prices passed to the harness stay the registered ones either way, so costs stay comparable.

## Metrics and decision rule (fixed now)

**Primary:** accepted / valid sessions, `av-phase0` against `av-before`, pooled over the three
tasks. Accepted means the task's hidden test passes after the session. Each arm gets its 95 %
Wilson score interval, by the formula of `margincode_bench::stats::wilson_interval`. The analysis
script (`analyze.py` of this directory) is written and committed with "Build", before the first
paid session. It reuses `../2026-11-wrong-lesson-3/analyze.py`'s code where that code applies.

- No claim is made unless both `av` arms have 30 valid sessions, 10 per task.
- **Phase 0 helps** if `av-phase0`'s lower end is above `av-before`'s upper end. **Phase 0
  hurts** if `av-phase0`'s upper end is below `av-before`'s lower end. Otherwise it is **not
  shown at this size**. For scale, at 30 sessions each: 2 of 30 has the upper end 0.213, and
  11 of 30 has the lower end 0.219.
- The decision is pooled, and the per-task table is printed beside it. The three tasks are not a
  sample of "silent tasks", and no claim is made beyond these three.

**Guard (the plan's bar):** the guard's `av` arm accepted at least 27 of 30. That is a rate of
0.900, not below 0.886, the lower end of the E4 sweep's interval at N = 0
(`../2026-11-e4-scale/README.md`: `av` 30 of 30, 0.886 to 1.000). A line that teaches distrust
must not cost what true lessons give. If the guard fails, the README says so beside the
primary, whatever the primary shows. Phase 0 is then not shown to be free on true lessons, and
what follows is not decided here.

**Secondary, reported and not used for the decision:**

1. **The gap to the notes file:** `av-phase0` against `notes`, pooled, the same rule. If the upper
   end of `av-phase0`'s interval is below the lower end of `notes`'s, **the gap remains**.
   Otherwise `av` is **not shown worse than `notes` at this size**.
2. **Per task:** accepted / valid for all four arms, with intervals. For `relay-quote-split`, stage 2's
   numbers are set beside it as a description, not a comparison (another pool and another run).
3. **Superseded wrong records, read from the store** (`superseded.jsonl`, not the transcript):
   sessions whose task wrote a record that supersedes that task's own wrong note. Also: sessions
   that superseded another task's wrong note, and sessions that superseded a true note. The last
   is the cost the line can have, and on these tasks it is the only place that cost can show. Both
   `av` arms are counted. Build A's CLI has no `--supersedes`, so its count is expected to be 0.
   Anything else is a finding. The transcript count of stage 2's `supersede_use` is set beside it,
   and with it the two-by-two table "superseded the task's wrong note" against "accepted". The
   table describes; it is not a test.
4. **The wrong edit:** sessions whose diff matches the task's `[[dead_end]]` pattern
   (`repeated_dead_ends` ≥ 1), accepted and not, per arm and task.
5. **Records that restate the wrong edit:** `av` sessions with a record of their own task whose
   text matches the task's `[[dead_end]]` pattern, split by whether that record supersedes the
   task's wrong note. Such a record that supersedes the note is a correction that names the wrong
   call. One that does not is counted as a restatement, an upper bound, since the text alone does
   not tell rejection from repetition. Stage 2's single column could not separate the two.
6. **Delivery:** `notes`, a read of `NOTES.md`; both `av` arms, the task's wrong note whole in
   the session-start hook's output; `av-phase0`, the line as well. `cold`, none.
7. **Cost:** total, per accepted task and median session cost per arm, as the harness reports
   them at the registered prices. The real spend is the balance before and after.
8. **Last edits,** stage 2's `content.py` per task: for every session that was not accepted, the
   last text it wrote to the task's file and whether it holds the note's call.

**Leak check, before the result is read.** `leak.py` and `content.py` of this directory scan
every transcript as stage 2's did. They count as exposing: a hidden test's name, another
session's directory, the bench tree, the results, the stores, and any earlier run's
directories. Both are run and committed before `rows.jsonl` is read for accepted counts. The
primary is computed on all valid sessions. Exposed sessions are listed, and the tables are given
again without them. An exposed session is a deviation and is written up as one.

## Cost estimate

From the per-session costs of the earlier cells at the registered prices. The two silent tasks
have never run, so their costs are a guess from `relay-quote-split`'s:

| command | sessions | per session, from | estimate |
|---|---|---|---|
| smokes | 4 | stage 2's smoke | about $0.05 |
| `cold` | 30 | stage 2 `cold`, $0.1112 / 12 = $0.0093 | $0.28 |
| `av-before` | 30 | second run `av`, $0.3499 / 30 = $0.0117 | $0.35 |
| `notes` | 30 | stage 2 `notes`, $0.4372 / 30 = $0.0146 | $0.44 |
| `av-phase0` | 30 | stage 2 `av`, $0.4393 / 30 = $0.0146 | $0.44 |
| guard | 30 | E4 N = 0 `av`, $0.3751 / 30 = $0.0125 | $0.38 |
| **total** | **154** | | **about $1.9** |

The plan's estimate was $2.0 to $2.5 for 150 sessions. The budgets above add up to $3.50, the
ceiling of what the run can spend.

## Stopping rules

- `--session-cap-usd 0.08` per session. `max_turns` 30 is the real per-session limit.
- `--budget-usd` per command as written above.
- **Balance floor $0.50.** Before each command, its budget is lowered to what the balance leaves
  above the floor, if that is less. An arm that runs out of budget is reported as partial, never
  extrapolated, and the primary is then not assessed.

## Validity and missing sessions

As in stage 2: a session the harness marks `invalid` stays in `rows.jsonl` and is not retried.
A session that stops the harness with an error (no row written) is an infrastructure failure:
the cause is fixed and the same command is rerun. No row is edited, dropped or re-run by hand.
Delivery is checked on the first session of each note arm before the rest is read as valid. If it
fails, that arm stops and nothing of it is reported as a result.

## Confounds named in advance

- **Two builds, two daemons.** The `av` arms differ by #148 and #150 and by their daemon's fresh
  store. The harness also differs, since build A's lacks #150. #150 is housekeeping only. The two arms run back to back and not interleaved, so a change of the
  model's behaviour between commands 2 and 3 would show as a difference. Neither `cold` nor
  `notes` guards against it, since both run on build B with command 3 or before it.
- **Two changes at once.** The line and the command arrive together (Phase 0 is one change).
  A difference cannot be assigned to either.
- **The pool has five other wrong notes and five true ones.** The pack shows a few records per
  task, some of them other tasks' wrong notes that the code does contradict (stage 2: `money-plain`,
  `line-taxes`). Correcting those can teach the agent to distrust its own task's note as well.
  That is part of what is measured, not separated from it.
- **The arms differ in framing,** as in every wrong-lesson run: `CLAUDE.md` tells the `notes`
  agent to follow `NOTES.md`'s decisions; the pack tells the `av-phase0` agent to check claims.
  The pack shows a record's summary, and the notes file shows summary and body.
- **Silence is not the same on the three tasks.** `relay-quote-split`'s note can be seen through
  by joining `Money.allocate`'s docstring to the goal (`../../tasks-m0/README.md`). The other two
  need a fact about Python that the repository does not state.
- One model, one wording of each note, ten repetitions per task. Thirty sessions over three tasks
  are not thirty tasks.

## What later runs copy from this one

M1 Part 2 and M2 stage B (`../2026-11-13-standing-and-dreams/PREREG.md`) run "on the M0 tasks,
model and stack as the M0 run". That means `bench/tasks-m0` on `notes-silent.toml`; the model,
endpoint, Claude Code version and prices named under "Build"; `--agent-user`, `--jit-hooks`, no
`--keep-work`, no noise, run seed 1; and the session cap. Their builds are their own.

## Build

To be filled in, and committed, before the first paid session: for each of builds A and B, the
commit, the tree id, and the sha256 of `av`, `scholion`, `scholion-daemon` and `scholion-bench`.
Also: the container and its image, both daemons' ports and services, the embedder, Claude Code's
version, the model and its prices, the balance, and the results of the free checks above.
