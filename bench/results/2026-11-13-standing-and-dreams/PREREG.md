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
