# Pre-registration: standing (M1) and raw against dreamed memory (M2 stage B), 2026-11-13

**Prepared, not run.** This file fixes the questions, the bars and the commands before any
session of either measurement. Nothing here has been run. The free replay (M1, part 1) needs the
bench stack, and every paid session needs the owner's machine or an API key. When a run starts,
its build is written under "Build" and committed before the first session. Anything that turns
out differently is a deviation in `README.md`, not an edit here.

Source: Plan 16 (`margincode-cloud`, `docs/superpowers/plans/2026-11-12-agentvcs-plan-16-hivemind-memory.md`),
"Measurement, M1" under Phase 1 and "Measurement, M2" under Phase 2. The bars are copied from
there. No bar is moved after a result is seen.

## M1: does standing take a wrong note from reverted work out of the lead?

Phase 1 (standing) gives each record the origin of its task (`merged`, `open`, `abandoned`,
`reverted`) and multiplies a `reverted` record's rank by 0.4. Before this change the bench could
not seed a note as reverted work: every relay note was a record of one open earlier task. The
harness now does it with `--wrong-reverted`. The pool's `no_commit` notes (the wrong-lesson
pool's wrong notes) become the records of a second earlier task. That task adds one file
(`AV_BENCH_REVERTED.md`), is submitted, validated, approved and merged, and its merge is then
reverted, all before the agent's task starts. Trunk ends with the base tree again, and the
agent's workspace check (`tree_matches`) still holds. The other notes stay the records of an
open earlier task. Test: `a_wrong_note_can_be_seeded_as_the_record_of_a_reverted_task`
(`crates/margincode-bench/tests/runner.rs`).

### Part 1, free (no agent, no money)

```bash
OUT=bench/results/2026-11-13-standing-and-dreams
# the wrong-lesson pool, the wrong note as reverted work, and the same without the flag (same build)
margincode-bench first-packs bench/tasks-relay-wrong --wrong-reverted --out $OUT/m1-reverted <the stack's options>
margincode-bench first-packs bench/tasks-relay-wrong                  --out $OUT/m1-open     <the stack's options>
# the true-lesson relay set, unchanged by standing: N = 0, 50, 500
for n in 0 50 500; do
  margincode-bench first-packs bench/tasks-relay --noise $n --out $OUT/m1-true-$n <the stack's options>
done
```

**Bars (from the plan, fixed):**

- **B1.** In `m1-reverted`, the task's own (wrong) record is **not** at the `summary` or `full`
  tier of the hook pack in at least **4 of 5** tasks. `first-packs.md`, column "hook pack".
- **B2.** On the true-lesson set, every lesson keeps what it had after #142: shown in full
  (`first-packs.md`, "Lesson shown in full") **10 of 10** at N = 0, 50 and 500.
- `m1-open` is reported beside `m1-reverted` as the same build without the flag. It is not a
  bar: it shows how much of B1 is the flag and how much is the build.

**Decision.** If B1 or B2 fails, Part 2 is not run and no money is spent (plan, "Phase 1 next,
behind its free replay"). The failing rows are reported as they are.

### Part 2, paid, only if Part 1 passes

- Tasks: the M0 tasks (`relay-quote-split` plus the two new tasks of its kind, written for M0),
  with the wrong note from a reverted task (`--wrong-reverted`).
- Arms: `av` with Phase 0, and `av` with Phases 0 and 1. The two builds are named under "Build".
- 30 sessions per arm, 60 in all. Primary metric: the accepted rate on those tasks (hidden
  pytest). No claim below n = 30 per arm.
- Model and stack: as the M0 run that precedes it. Its PREREG names them; this run copies them.

## M2 stage B: raw memory against dreamed memory

**Question.** On the M0 tasks, is an agent accepted more often when the store it reads has had
one verified dream pass than when it reads the raw records? This is the article's
raw-against-dreamed comparison, on our own failure: wrong notes carried forward.

- Arm **raw**: `av` with Phases 0 and 1, the store as seeded.
- Arm **dreamed**: the same seeded store after one `av dream` pass and one `av dream --verify`
  pass by another agent. It is frozen and copied into every session of the arm: the same verdicts
  for all 30 sessions, so the arm measures one dreamed memory and not dream-to-dream variance.
- 30 sessions per arm. Primary metric: the accepted rate. Reported beside it: the dream runs'
  own cost (both passes), and the verdicts the passes ended with (which wrong notes ended
  `disputed` and effective, and whether any true lesson ended `disputed` or `outdated`).
- **Order.** Stage A (precision: the wrong-lesson pool plus 50 noise notes, dreamed 6 times from
  the same start) runs first. Stage B runs only if stage A meets its bars: at least 4 of 5 wrong
  notes end `disputed` and effective in at least 5 of 6 passes, no true lesson ends `disputed` or
  `outdated` in any pass, and every stored quote re-checks.
- Estimate (plan, **estimate**): 36 short dream runs and 60 sessions, about $2.

**The harness for stage B** (built after this file was first written, before any session; no
question or bar above changed). `margincode-bench dream-freeze` seeds each task's store as a
session does, runs the two passes once with the persona commands named under "Build", and writes
what they ended with to `dreamed.json`; `run --dreamed dreamed.json` gives every session of the
dreamed arm those verdicts and reviews in its own freshly seeded store, before its agent starts
(the daemon checks every quote again and the harness checks each verdict's state). That is the
"frozen and copied into every session" above: the same verdicts in all 30 sessions, written
through the daemon's routes rather than copied as a database. Stage A runs `dream-freeze` six
times from the same start, one `--out` each, and reads its tally (wrong notes `disputed` and
effective; true lessons `disputed` or `outdated`; each note counted once, by its newest
effective verdict, beside how many notes any verdict judged). Each of its passes is
`--until-empty`: the plan's "each time until the worklist is empty". Its "(3 runs and 3 verify
runs at the default limit)" was an estimate of what that takes, and an earlier draft of this
paragraph read it as the pass itself (`--rounds 3`); the owner settled it on 2026-11-14. Three
rounds do not empty the worklist: a judged record stays on it, and a wrong note from reverted
work keeps its +3 and a place in every round, so a true lesson could go unjudged and the bar "no
true lesson ends `disputed` or `outdated`" pass untested. Under `--until-empty` the review runs
come back to back at the default limit, each handed records no run has judged yet, until the
worklist is empty (or holds only records the agent left undecided, which it is handed again);
then the verify runs, until none waits. A note a run was handed and left out is *undecided*, as
`DREAM.md` asks ("leave an item out when you cannot tell"): it is reported, and the bars read it
as what it ended (not `disputed`, not `outdated`). A pass in which a wrong note or a true lesson
was never handed to any run (`--max-runs` was reached, or undecided records filled the worklist
ahead of it) is not covered: the command exits 1, and the pass is reported but not counted as
meeting the bars. "The same start" is one
store: the five tasks of `bench/tasks-relay-wrong` share one pool (`notes-wrong.toml`, all five
wrong notes and five true lessons) and seed the same records, so each pass dreams one of them
(`--only relay-quote-split`), not five copies. The bars themselves are unchanged. The tally also
reports the plan's "share of items returned *undecided*": `undecided / handed`, the records a
review run was handed that no run gave a verdict (reported, as in the plan; it is not a bar, and
it decides Phase 5). With about 61
records that is about 4 review runs and 4 verify runs a pass, about 48 dream runs for stage A
where the plan estimated 36.
Test:
`a_frozen_dream_is_replayed_into_a_session_of_the_dreamed_arm`
(`crates/margincode-bench/tests/runner.rs`). The commands:

```bash
OUT=bench/results/2026-11-13-standing-and-dreams
# stage A: six passes from the same start (wrong-lesson pool plus 50 noise notes)
for i in 1 2 3 4 5 6; do
  margincode-bench dream-freeze bench/tasks-relay-wrong --only relay-quote-split \
    --wrong-reverted --noise 50 --until-empty \
    --dream-cmd '<dreamer command>' --verify-cmd '<verifier command>' \
    --out $OUT/m2-stage-a/dreamed-$i.json <the stack's options>
done
# stage B, only if stage A meets its bars: one frozen dream, then both arms on the M0 tasks
margincode-bench dream-freeze <M0 tasks> --wrong-reverted \
  --dream-cmd '<dreamer command>' --verify-cmd '<verifier command>' \
  --out $OUT/m2-stage-b/dreamed.json <the stack's options>
margincode-bench run <M0 tasks> --arms av --wrong-reverted --out $OUT/m2-raw ...
margincode-bench run <M0 tasks> --arms av --wrong-reverted --dreamed $OUT/m2-stage-b/dreamed.json \
  --out $OUT/m2-dreamed ...
```

## Build

To be filled in, and committed, before the first session of each part: the commit, the tree id,
the sha256 of `av`, `margincode` and `margincode-bench`, the daemon image, and the model and its
prices.

### Part 1 (M1, free replay), filled in 2026-11-13 before the first `first-packs` command

- **Commit** `850359b990829f9514f6f107de1a28e522f5c8fd` (branch `bench-wrong-reverted`), tree
  `e7fcaa3ace6acd6f58effa15930fd4965409f5f6`, clean working tree. One build serves all five
  commands (`m1-open` is the same build without the flag).
- **Binaries**, `cargo build --release --locked -j 6 -p margincode-daemon -p margincode-cli
  -p margincode-bench` in `rust:1.97.1` (`sha256:b1b3c9c0…dec813f`), `CARGO_TARGET_DIR=/target`:
  - `margincode` `e15a7bbb054af0650f478f4c666283e225943a34b679f62dc0870c8e6b276586`
  - `av` `130ef9bf2d1cb49746751edfaab459a7809e327d0987443fd7bd6291f8767b98`
  - `margincode-daemon` `7b818d43c5fa506f214c39b40d22fd31ea92af017669c0db4a3f66410aa7705f`
  - `margincode-bench` `b201129c032873d7863a3d5ee9aa26ea6a2808972c6228d0a3c26be04b913818`
- **Daemon**: no image. The release binary above, run inside the harness container `m1-harness`
  (`rust:1.97.1`, this checkout bind-mounted at `/src`) on the Docker network `m1-net`, which
  also holds Postgres 16 (`m1-pg`, `postgres@sha256:ca0bd484…dbecb641d`), Qdrant v1.13.6
  (`m1-qdrant`) and TEI cpu-1.7.2 (`m1-tei`), the recipe of `2026-11-first-pack/README.md`:

  ```bash
  cd /root
  AV_DATABASE_URL=postgres://av:<password>@m1-pg:5432/bench AV_LISTEN=127.0.0.1:7450 \
  AV_STORE_ROOT=/root/store AV_WS_ROOT=/root/bench-work/ws \
  AV_QDRANT_URL=http://m1-qdrant:6334 AV_EMBED_URL=http://m1-tei:80 \
  nohup /target/release/margincode-daemon >> /root/daemon.log 2>&1 &
  ```

  Fresh database, store, Qdrant and workspace root: nothing from an earlier run.
- **Embedder**: TEI cpu-1.7.2 (`sha a69cc2e`) serving `BAAI/bge-small-en-v1.5` (float32, cls
  pooling). Noise seed 1 (the default).
- **Options** for every command ("the stack's options"): `--av-url http://127.0.0.1:7450
  --work-root /root/bench-work --import-dir /root/bench-work/src --import-mount
  /root/bench-work/src`. Tasks' repo: `bench/tallyho/make-repo.sh` into `~/bench-repos/tallyho`
  (from `git archive HEAD bench/tallyho`, so the files keep their modes); venv
  `~/bench-work/venv` (Python 3.13.5, pytest 9.2.0).
- **Model and prices**: none. Part 1 starts no agent and calls no model.

### M2 stage A, filled in 2026-11-15 before its first pass

- **Commit** for the record: the commit that adds this section, on `bench-dream-freeze` (PR #7).
  The binaries were built at `56968ed`; `git diff 56968ed HEAD -- crates Cargo.lock Cargo.toml
  rust-toolchain.toml` is empty. The tasks, pool and repo come from a `git archive 0a3a141 bench`
  snapshot (`/root/m2src`); `bench/tasks-relay`, `bench/tasks-relay-wrong` and `bench/tallyho` are
  unchanged from `0a3a141` to this commit.
- **Binaries**, `cargo build --release --locked -j 6 -p margincode-daemon -p margincode-cli
  -p margincode-bench` in `rust:1.97.1` (`sha256:b1b3c9c0…dec813f`), copied to `/root/m2bin`:
  - `margincode` `e15a7bbb054af0650f478f4c666283e225943a34b679f62dc0870c8e6b276586`
  - `av` `130ef9bf2d1cb49746751edfaab459a7809e327d0987443fd7bd6291f8767b98`
  - `margincode-daemon` `0f1bb18cc8028422e6c226287f3e7fa15e14c888961ba7c3100edf2e5f331fa5`
  - `margincode-bench` `49cea865757b1edaa952822669cf95b080eca2924ee8f598a9cbfd0a16750390`
- **Daemon and stack**: Part 1's recipe on the network `m2-net`: harness `m2-harness`
  (`rust:1.97.1`), Postgres `postgres:16` (`m2-pg`), Qdrant v1.13.6 (`m2-qdrant`), TEI cpu-1.7.2
  serving `BAAI/bge-small-en-v1.5` (`m2-tei`); the daemon listens on `127.0.0.1:7450` (pid
  16682). The database is not fresh: it holds the stub check's and the three pilots' stores. Each
  pass seeds a fresh repo of its own (a new run id), and the worklist reads only its repo.
- **Options** ("the stack's options"): `--av-url http://127.0.0.1:7450 --work-root
  /root/bench-work --import-dir /root/bench-work/src --import-mount /root/bench-work/src`; repo
  `bench/tallyho/make-repo.sh` into `~/bench-repos/tallyho`; noise seed 1.
- **Agent**: Claude Code 2.1.325 (node 20, jq 1.7, git 2.47.3, Python 3.13.5 in the harness),
  an Anthropic API key passed into the container by name. Dreamer and verifier are two agents
  (`dreamer`, `verifier`) on the same model and the same script, `/root/dreamer.sh` =
  `/root/verifier.sh` (sha256 `e2452e7f04cce6ce1e548caf388fec010961b9a4d11d3d6f6497daca6539eea9`):

  ```sh
  # Plan 16 M2 stage A agent: Claude Code on the model in $M2_MODEL, in the dream's directory.
  exec claude -p "Read DREAM.md and do what it says." \
    --model "$M2_MODEL" --output-format json --max-budget-usd "$M2_RUN_CAP" \
    --permission-mode acceptEdits --allowedTools "Read,Write,Bash" \
    > "claude-result-${AV_DREAM_ROUND:-0}.json"
  ```

  with `M2_MODEL=claude-sonnet-5-5` and `M2_RUN_CAP=1` ($1 per agent call, about 5 times the
  dearest pilot 3 run, so a pass that disputes more than the pilots did is not cut off).
- **Model and prices**: `claude-sonnet-5-5`, $2 input / $10 output per million tokens, cache
  read $0.10 (list). Cost is Claude Code's own `total_cost_usd`, summed over every
  `claude-result-*.json` of a pass. Budget: $20 for all M2 stage A work, $1.65 of it spent on the
  smoke call and the three pilots (`README.md`).
- **The command**, run six times, one at a time:

  ```bash
  margincode-bench dream-freeze bench/tasks-relay-wrong --only relay-quote-split \
    --wrong-reverted --noise 50 --until-empty --limit 50 \
    --dream-cmd 'sh /root/dreamer.sh' --verify-cmd 'sh /root/verifier.sh' \
    --out /root/m2/stage-a/dreamed-$i.json <the stack's options>
  ```

  **Deviation: `--limit 50`** where the stage A paragraph says "at the default limit" (20). The
  reason, and the go/no-go it passed, are in `README.md` ("Pilot 3"): a reviewer that leaves most
  items undecided cannot have every record handed at 20; no stage A outcome had been seen. 50
  items a run may itself change how the reviewer judges. `--max-runs` stays at its default, 20.
- **Rules fixed before pass 1** (`README.md`): an uncovered pass is one of the six and does not
  meet the bars; an aborted pass is run again from the same start; before each pass the money
  left must be at least 1.5 times the dearest pass so far.

### Part 2 (M1, paid), filled in 2026-11-16 before its first paid session

This subsection is written in two commits, both before the first paid session (the smokes): the
first names the builds, the stack, the commands, the rules and the free checks' pass conditions,
before any free check ran; the second adds the free checks' results. Nothing above this
subsection is changed. The scripts are in `m1-part2/`.

- **Model and stack: M0's, as "Model and stack" above asks.** M0 ran on Claude Haiku 5.5
  (`../2026-11-14-m0/AMENDMENT-1-haiku.md`, which says M1 Part 2 copies it), so Part 2 does:
  `claude-haiku-5-5` on Anthropic's first-party API, `ANTHROPIC_BASE_URL` unset, the key passed
  into the container by name at `docker exec` and given to Claude Code as `ANTHROPIC_API_KEY`
  (never written down), `ANTHROPIC_DEFAULT_HAIKU_MODEL`, `ANTHROPIC_SMALL_FAST_MODEL` and
  `CLAUDE_CODE_SUBAGENT_MODEL` set to `claude-haiku-5-5`; Claude Code 2.1.318; prices
  `--price-per-mtok 0.10,0.125,0.01,0.50` (Haiku 5.5's list prices for prompts up to 100K
  tokens); `--agent-user benchagent`, `--jit-hooks`, no MCP, no `--keep-work`, no noise, run seed
  1, `--session-cap-usd 0.08`. The tasks are `bench/tasks-m0` on `notes-silent.toml`, with
  `--wrong-reverted` in both arms: the pool's eight `no_commit` notes (each task's own wrong note
  among them) are the records of an earlier task whose merge was reverted; the five true notes
  stay the records of an open earlier task.
- **The run tree.** `/m1/tree-h`: a `git archive` of `main` at `7dd5208` (tree `012def99…`, as
  M0's) made by M0's `haiku-tree.sh`, so it differs from the archive only in the `model` line of
  the 13 task files under `bench/tasks-m0/` and `bench/tasks-relay/`. Its task, pool and tallyho
  files are byte for byte M0's `/m0/tree-h` (`diff -r` shows only files M0 added later under
  `bench/results/` and `__pycache__`).
- **Build P0, `av` with Phase 0:** commit `90ba62e` (tree `391e83cd…`), M0's build B: its
  binaries, copied from `/m0/B/bin` to `/m1/P0/bin`:
  - `av` `021ad167c462d568d5c0d0c4e0fe8842ceb80fddb1f543dc56d179f3166ac956`
  - `scholion` `f52b8b0182e11a94cd4f62b80b1e6ac5e30120d69264cf544757f7cf0f68f90b`
  - `scholion-daemon` `461f0e6632b91af3a20b20e9a023256c0152ee8205089207d49f9056b9bf2d47`
- **Build P1, `av` with Phases 0 and 1:** commit `84bb472` (tree
  `61195c99e62937375026e98122c5d290362f0645`), whose parent is `90ba62e`: the two builds differ by
  Phase 1 (standing) and nothing else. From a `git archive` in `/m1/srcP1`, `cargo build
  --release --locked -j 12 -p scholion-cli -p scholion-daemon -p scholion-bench` in the harness
  container from an empty `CARGO_TARGET_DIR=/target/m1p1`, copied to `/m1/P1/bin`:
  - `av` `eeb89199ad606604e4fa862a0d5b0de6f6cd7e03e3c85b46ccff108cb4213d27`
  - `scholion` `c065b0206f5321d58b15f0c89bcf4fd3d48c73bcaf43a3b05b9f2010359efe2f`
  - `scholion-daemon` `021c9633835d3d4b67794fb3fc247a943cb10fd3bdd85f32c4267319d694219e`

  Main's Phase 2 (`3f9e159`, verdicts) and everything after it are left out: they would change
  the arm beyond Phase 1.
- **One harness for both arms: `main` at `7dd5208`.** Neither build's harness has
  `--wrong-reverted` (it came in `850359b`, after the rename). The harness of `7dd5208` seeds the
  reverted task through routes both daemons have (`submit`, `validate`, `review`, `merge`,
  `revert`). Between `90ba62e` and `7dd5208` the harness's runner changes the seeding,
  `--dreamed` and names, not what a session is given or how it is run or verified. Built from
  `/m1/srcH` (a `git archive` of `7dd5208`), `cargo build --release --locked -j 8 -p
  margincode-bench`, `CARGO_TARGET_DIR=/target/m1h`, copied to `/m1/H/bin` (mode 700, root
  only): `margincode-bench` `bb261833202ebcee98f8ae3e734738b597be9fb18e545358f1c4d1b9f18ad188`.
- **Container and services**: `lp-harness` (`rust:1.97.1`), M0's container, in a new root `/m1`,
  on the network `lp-bench` with Postgres 16 (`lp-pg`), Qdrant 1.13.6 (`lp-qdrant`) and TEI
  cpu-1.7.2 (`lp-tei`) serving `BAAI/bge-small-en-v1.5`. M0's daemons were stopped first.
- **Daemons**: build P0's on `127.0.0.1:7481` (new database `m1p0`, store `/m1/P0/store`,
  workspace root `/m1/bench-work/ws-p0`); build P1's on `127.0.0.1:7482` (new database `m1p1`,
  store `/m1/P1/store`, workspace root `/m1/bench-work/ws-p1`). Both with
  `AV_QDRANT_URL=http://lp-qdrant:6334` and `AV_EMBED_URL=http://lp-tei:80`.
- **Options**: `HOME=/m1`; `--work-root /m1/bench-work --import-dir /m1/bench-work/src
  --import-mount /m1/bench-work/src`; `--agent-user benchagent --av-ws-root` with the command's
  daemon's workspace root; `--av-bin` the build's `av`. Task repository
  `bench/tallyho/make-repo.sh` into `/m1/bench-repos/tallyho` (`main` tree `c0ea5267…`, the same
  tree as M0's); venv `/m1/bench-work/venv`, Python 3.13.5 with M0's packages (pytest 9.2.0,
  pluggy 1.7.0, iniconfig 2.3.1, packaging 26.3, Pygments 2.21.0). `m1-part2/m1cmd.sh` runs one
  command with these options; `m1-part2/m1chain.sh` runs commands 1 and 2.

**Commands** (`m1cmd.sh P0|P1 <args>`, which adds the options above and `--confirm`; results to
`/m1/results/<out>`, copied to `m1-part2/<out>` afterwards as the harness wrote them):

```bash
# 0. smokes, one per build: does an isolated session run at all? (not a result)
m1cmd.sh P0 bench/tasks-relay/order-ids.toml --arms av --jit-hooks --reps 1 --wrong-reverted \
  --session-cap-usd 0.08 --budget-usd 0.10 --out /m1/results/smoke-p0
m1cmd.sh P1 bench/tasks-relay/order-ids.toml --arms av --jit-hooks --reps 1 --wrong-reverted \
  --session-cap-usd 0.08 --budget-usd 0.10 --out /m1/results/smoke-p1
# 1. av with Phase 0, then 2. av with Phases 0 and 1, back to back (m1chain.sh)
m1cmd.sh P0 bench/tasks-m0 --arms av --jit-hooks --reps 10 --wrong-reverted \
  --session-cap-usd 0.08 --budget-usd 0.70 --out /m1/results/m1-p0
m1cmd.sh P1 bench/tasks-m0 --arms av --jit-hooks --reps 10 --wrong-reverted \
  --session-cap-usd 0.08 --budget-usd 0.70 --out /m1/results/m1-p1
```

`relay-order-ids`'s pool has no `no_commit` note, so `--wrong-reverted` changes nothing in the
smokes; it is passed so the smoke command is the measured one. **The smokes pass** under M0's
conditions and its amendment's: every row `status=ok` and valid; the model answered and made at
least one edit; verify ran; `hook-count` not empty; `meta.json` names `benchagent`, Claude Code
2.1.318 and `models: ["claude-haiku-5-5"]`; no single request with a prompt above 100K tokens;
no session above $0.04. If one fails, the cause is fixed and it runs again in a new directory
(`smoke-p0-2`, …); if the prompt or cost condition fails, the run stops and the owner is asked.

**The store export**, after command 2, from each daemon's database, M0's two queries
(`../2026-11-14-m0/PREREG.md`), written to `m1-p0/` and `m1-p1/`: `superseded.jsonl`, and
`records.jsonl` with `r.author_agent NOT IN ('agent-a', 'agent-r', 'bench-merger')` (`agent-r`
is the reverted task's author and `bench-merger` its merge and revert, `runner.rs` of the
harness).

**Decision rule** (the plan's, as M0 wrote it out; `m1-part2/analyze.py`). Primary: accepted /
valid sessions, `av-p01` (build P1) against `av-p0` (build P0), pooled over the three tasks, each
with its 95 % Wilson interval. No claim unless both arms have 30 valid sessions, 10 per task.
**Phase 1 helps** if `av-p01`'s lower end is above `av-p0`'s upper end; **Phase 1 hurts** if
`av-p01`'s upper end is below `av-p0`'s lower end; otherwise **not shown at this size**, and the
words "equal" and "no effect" are not used. Per task, delivery of the wrong note (in the
session-start hook at any tier, whole, labelled `reverted`, whole in a pre-read answer), the
wrong edit, superseded records and cost are described beside it, not used for the decision.
`analyze.py` reads every row of both arms into the decision with no filter between the tables
and the rule. Tested before the run on M0's `av-before` and `av-phase0` rows relabelled as
`m1-p0` and `m1-p1`: it reproduces M0's tables (11/30 and 30/30, 18 wrong edits, 15 supersedes
of the task's wrong note and 1 of a true note), and with an arm cut to 20 rows it prints "No
claim".

**A prediction, for the record.** In M0, `av-phase0` (build P0, without `--wrong-reverted`) was
accepted 30 of 30 on these tasks. If build P0 stays near that with the wrong notes seeded as
reverted work, `av-p01` cannot be shown to help at this size (30 of 30 is the most it can get,
and its lower end, 0.886, is not above a P0 upper end of 1.000); only "hurts" or "not shown" can
come out.

**Leak check, before the result is read.** `m1-part2/leak.py` (M0's, with this run's
directories) and part 1 of `m1-part2/content.py` (`--hidden-only`, which reads no row) are run
on every transcript and committed before `rows.jsonl` is read for accepted counts. The primary is
computed on all valid sessions; exposed sessions are listed and the tables given again without
them.

**Stopping rules, validity and spend.** `--session-cap-usd 0.08`; `max_turns` 30; the budgets
above (an arm that runs out is partial, never extrapolated, and the primary is then not
assessed). A session the harness marks `invalid` stays and is not retried; a command that stops
with an error (no row written) is fixed and rerun (a rerun resumes). No row is edited, dropped
or run again by hand. The key's month cap is the owner's ($100 a calendar month, tracked outside
this repository): a command starts only if this month's spend plus its budget stays under it.
Commands 1 and 2 run back to back, with nothing between them.

**Confounds named in advance.** The two arms differ by Phase 1 and by their daemon's fresh
store, and run one after the other, not interleaved. The harness is newer than both builds (one
harness for both, so it is not a difference between them). The merge and revert of the earlier
task are in the repository's history in both arms, and the revert's own note (`merge … reverted
by bench-merger`) is a record in both; only build P1 ranks and labels by them. One model, one
wording of each note, ten repetitions per task.

**Free checks** (no model), with their pass conditions, fixed before any of them ran:

1. Task set: `bench/tasks-relay/reference/check.py`, `bench/tasks-relay-silent/check_set.py`
   and `bench/tasks-m0/check_set.py` with `/m1/bench-repos/tallyho` and the venv: all ok.
2. Specs: `margincode-bench check bench/tasks-m0 bench/tasks-relay` in the run tree: all ok.
3. Replay, one per build, fresh state each: `margincode-bench first-packs bench/tasks-m0
   --wrong-reverted --jit-hooks --av-bin /m1/<b>/bin/av --out /m1/results/first-packs-<b>
   <options>`, against that build's daemon. It passes if both exit 0 (every task's reverted task
   merged and reverted, and the agent's task started on the base tree) and, for each of the
   three tasks: on build P0, the task's wrong note is in the session-start pack with its `rec:`
   handle and not labelled with an origin, and Phase 0's line is under "Decisions & dead ends";
   on build P1, Phase 0's line is there, and every line that shows one of the eight seeded wrong
   notes is labelled `reverted`. Where build P1 puts each task's wrong note (`summary`, `handle`
   or absent in the hook pack; whole in a pre-read answer or not) is reported, not a condition:
   moving it down is what Phase 1 does. If a condition fails, no paid session starts and the
   replay is reported as it is.
4. One `av` per command: as `benchagent` with each command's `PATH`, `av` is that build's (sha256)
   and `av record --help` names `--supersedes`; the other build's `av` cannot be run.
5. Isolation and fresh state: `/m1` 711; the run tree, both sources, the harness's directory, the
   repositories, both stores, `/m1/results` and `/m1/checks` 700; `/m1/bench-work`, its `runs`,
   `ws-p0` and `ws-p1` 711; `/m0`, `/b2`, `/bench`, `/root` and `/src` 700; `/tmp` emptied before
   each command; new databases, stores and workspace roots for both daemons.

