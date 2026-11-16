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
5. **One `av` per command.** The harness runs the hooks with `--av-bin`, but an `av` the agent
   types in Bash is looked up on `PATH` (`runner.rs`, `settings` and `claude_command`, the same at
   both commits). So the two builds live in two directories, `$A` and `$B`. Each command runs with
   only its own build's directory on `PATH`, and the other build's directory is mode 700 for the
   whole command (closed to `benchagent`). Checked as `benchagent` with each command's `PATH`:
   `av record --help` lists `--supersedes` on build B and does not on build A, and `$B/av` (on
   A's command) or `$A/av` (on B's) cannot be run. Without this, build B's `av` could reach
   `av-before`: build A's daemon already accepts `supersedes` over HTTP; only its CLI lacks the
   flag.
6. **Isolation and fresh state,** as stage 2's PREREG lists them: the agent user, the modes of
   the bench tree, the results, the stores and the earlier runs' directories, `/tmp` emptied, a
   new database and store per daemon, one task-repository clone and one venv.
7. **Balance** read before command 0 and written down.

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

`PATH` and the closed directory are set per command as free check 5 says. A rerun of the same
command resumes it. The results directories are copied into this directory afterwards, as the
harness wrote them.

**The store export** (secondary measure 3). After commands 2 and 3, each daemon's database is
read once and written to `m0-av-before/superseded.jsonl` and `m0-wrong/superseded.jsonl`.
Records survive a session: the harness abandons the session's task, it deletes none
(`remove_work`), and `records.task_id` cascades only on a delete. Every session has a
repository of its own, `b<run id>-<task>-<arm short>-<rep>` (`runner.rs`, the same at both
commits), so a row is joined to its session by `tasks.repo`. The query, on the schema of both
builds (`0001_init.sql`: `records.supersedes`, `tasks.repo`):

```sql
SELECT r.id, t.repo, r.task_id, r.author_agent, r.kind, r.summary, r.created_at,
       r.supersedes, s.summary AS superseded_summary
FROM records r JOIN tasks t ON t.id = r.task_id JOIN records s ON s.id = r.supersedes
WHERE r.supersedes IS NOT NULL ORDER BY t.repo, r.created_at;
```

The superseded record is known by its summary: it is the session's task's own wrong note when
`superseded_summary` is that pair's `summary` in `notes-silent.toml`, another task's wrong note
when it is another `no_commit` note's, and a true note otherwise.

**The smokes** are stage 2's smoke: isolated sessions on `relay-order-ids` with its true lesson.
They pass if each row is `status=ok` and valid, and the event stream shows that the model answered
and made at least one edit. The verify command must have run, `av`'s `hook-count` must not be
empty, and `meta.json` must name `benchagent` under `agent_isolation` and the Claude Code
version. If a smoke fails, the cause is fixed and it runs again in a new directory (`smoke-b-2`,
…). No later command starts before both smokes pass. Smoke rows are not part of any arm.

**The gate after the control.** Commands 2 to 4 start only if `cold` is accepted at least 8 of
10 times on each of the three tasks (stage 2's bar, 10 of 12, scaled down and rounded down). A
task below that is not one the model solves unaided here. If any task misses the gate, commands
2 to 4 do not run at all. The README reports the control's rows, and the run stops. A replacement
task needs an amendment, committed before any note-arm session, and the control is then run
again on the new set.

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

**Guard (the plan's bar):** of 30 valid sessions, the guard's `av` arm accepted at least 27.
With fewer than 30 valid sessions, the guard is not assessed, and the README says so. That is a rate of
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

**Leak check, before the result is read.** `leak.py` and `content.py` of this directory do not
exist yet. Like `analyze.py`, they are stage 2's scripts with this run's directories, written and
committed with "Build" before the first paid session. They scan every transcript as stage 2's
did. They count as exposing: a hidden test's name, another
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

### Filled in 2026-11-15, before the first paid session

- **This file's tree.** The tasks, pool, repository script and checks are read from a `git
  archive` of `main` at `7dd5208` (tree `012def994ea9a8d2f0a6e7b553fd138e7434be62`) in
  `/m0/tree`. The commit that adds this section changes only this directory.
- **Build A**, commit `a67f130`, tree `e197872d30634a90c5731f952a6f24fd1f8c47c9`, from a `git
  archive` in `/m0/srcA`: `cargo build --release --locked -j 8 -p scholion-cli -p
  scholion-daemon -p scholion-bench` in the harness container (`CARGO_TARGET_DIR=/target/m0a`),
  copied to `/m0/A/bin`:
  - `av` `29b70d0c07b747f5919f36d0a0373136cf6fca3de90c50a31e172f125e599f17`
  - `scholion` `4058eb1e676a96f22bdb2de522108b605c5367a2c739f85d43570128de4cd811`
  - `scholion-daemon` `659fb7fae363e0fbd98edddd051e39378cef8e610732ab690dd53f48aadadb7a`
  - `scholion-bench` `22bc607f4a08b8efb41b1f2fb95ce4bb05f58242fdc7befc76bdcf984d2783d0`

  The first three are byte for byte the second run's binaries in `/bench/bin` (its build `55afbc8`
  adds no code to `a67f130`'s `av` and daemon). The harness differs, since `a67f130` has #149.
- **Build B**, commit `90ba62e`, tree `391e83cd3c74ff4da19423f96eb666195c5d0bb6`: stage 2's
  binaries from `/b2/bin`, copied to `/m0/B/bin`. Their sha256 are the ones stage 2's PREREG
  records: `av` `021ad167…6ac956`, `scholion` `f52b8b01…0f68f90b`, `scholion-daemon`
  `461f0e66…2bd47`, `scholion-bench` `ead477b8…bf32a891`.
- **Container and services**: `lp-harness` (`rust:1.97.1`, `sha256:b1b3c9c0…dec813f`; node
  22.23.5, git 2.47.3, Python 3.13.5), the container of the second run and stage 2, in a new root
  `/m0`, on the network `lp-bench` with Postgres 16 (`lp-pg`, `sha256:ca0bd484…dbecb641d`), Qdrant
  1.13.6 (`lp-qdrant`) and TEI cpu-1.7.2 (`lp-tei`) serving `BAAI/bge-small-en-v1.5` (float32, cls
  pooling).
- **Daemons**: build A's on `127.0.0.1:7461` (new database `m0a`, store `/m0/A/store`, workspace
  root `/m0/bench-work/ws-a`); build B's on `127.0.0.1:7462` (new database `m0b`, store
  `/m0/B/store`, workspace root `/m0/bench-work/ws-b`). Both use `AV_QDRANT_URL=http://lp-qdrant:6334`
  and `AV_EMBED_URL=http://lp-tei:80`. The second run's and stage 2's daemons are stopped.
- **Options**: `HOME=/m0`; `--work-root /m0/bench-work --import-dir /m0/bench-work/src
  --import-mount /m0/bench-work/src`; `ISO` is `--agent-user benchagent --av-ws-root` with the
  command's daemon's workspace root; `$A` = `/m0/A/bin`, `$B` = `/m0/B/bin`; `OUT=/m0/results`.
  Task repository `bench/tallyho/make-repo.sh` into `/m0/bench-repos/tallyho` (`main` =
  `43cd7b4`, tallyho 0.4.0); venv `/m0/bench-work/venv` with the second run's package versions
  (pytest 9.2.0, pluggy 1.7.0, iniconfig 2.3.1, packaging 26.3, Pygments 2.21.0).
- **Agent**: Claude Code 2.1.318, as in stage 2. Model `deepseek-flash` through
  `https://api.deepseek.com/anthropic`, the second run's environment (`ANTHROPIC_BASE_URL`,
  `ANTHROPIC_AUTH_TOKEN`, and `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_SMALL_FAST_MODEL`,
  `CLAUDE_CODE_SUBAGENT_MODEL` set to `deepseek-flash`). The key is passed into the container by
  name at `docker exec` and never written down. Prices `--price-per-mtok 0.30,0.30,0.006,1.20`.
- **Balance** read at 23:21 UTC on 2026-11-15: **$1.15**. Above the $0.50 floor that leaves $0.65,
  less than the run's $1.9 estimate. By the stopping rules, the smokes and command 1 run on what is
  there. Commands 2 and 3 must run back to back, so they start only after a top-up that covers
  both, or not at all. The balance is read again before every command and written down in
  `README.md`.
- **Scripts**: `analyze.py`, `leak.py` and `content.py` of this directory, committed with this
  section. Tested on stage 2's `relay-quote-split` rows relabelled as this run's directories: they
  reproduce stage 2's tables (`cold` 12/12, `notes` 28/30, `av` 12/30).
- **One export beyond the registered query**, for secondary measure 5, which needs records that
  do not supersede anything. After commands 2 and 3, each daemon also writes `records.jsonl`:

  ```sql
  SELECT r.id, t.repo, r.task_id, r.author_agent, r.kind, r.summary, r.body, r.created_at,
         r.supersedes, s.summary AS superseded_summary
  FROM records r JOIN tasks t ON t.id = r.task_id LEFT JOIN records s ON s.id = r.supersedes
  WHERE r.author_agent <> 'agent-a' AND r.kind IN ('decision', 'attempt', 'note')
  ORDER BY t.repo, r.created_at;
  ```

  (`agent-a` is the author of the seeded pool, `runner.rs` at both builds.)

Free checks, all before this section was committed:

1. The bar on `7dd5208`: fmt, clippy and the workspace suite are `main`'s CI and #14's checks.
   The two build commits are taken as merged.
2. Task set: `reference/check.py` "all 13 pairs ok (3 of them silent)";
   `tasks-relay-silent/check_set.py` "silent set ok"; `tasks-m0/check_set.py` "M0 set ok", all
   three tasks silent (base: own tests pass and the hidden test fails; reference: both pass; dead
   end: own tests pass and only the hidden test fails).
3. Specs: `scholion-bench check` of `bench/tasks-m0` and `bench/tasks-relay` with build A's and
   build B's harness: 26 of 26 `ok`.
4. Replay (`first-packs`, kept as `first-packs-a/` and `first-packs-b/`): passes. On both builds
   each task's wrong note is in the session-start hook's pack, summary whole, with its `rec:`
   handle. On build B the line stands directly under "Decisions & dead ends", and on build A it is
   absent. In the pack at task start, `relay-tax-with-rate`'s note is missing on both builds
   (2 of 3), which the PREREG does not test. `relay-quote-split` on build B has, as in stage 2, four records
   under "Decisions & dead ends" with the wrong note first, the same four. Further down, among the
   related records, the three extra notes of this pool appear (`order-merge`'s first). `first-packs`
   takes no `--agent-user` or `--av-ws-root`, so it ran without `ISO`.
5. One `av` per command, as `benchagent` with each command's `PATH` and the other build's `bin`
   at mode 700: build A `av record --help` names no `--supersedes`, build B names it, and the other
   build's `av` is refused on each.
6. Isolation and fresh state: `/m0` 711; `/m0/tree`, `/m0/srcA`, `/m0/bench-repos`, both stores,
   `/m0/results`, `/m0/checks` and `/m0/bench-work/src` 700; `/m0/bench-work`, its `runs`,
   `ws-a`, `ws-b` 711. The earlier runs' trees, stores, results, repositories and run directories
   under `/bench` and `/b2` are 700, as are `/root` and `/src`. `/tmp` was emptied, and it is
   emptied again before the first smoke.
7. Balance: $1.15, above.
