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
effective; true lessons `disputed` or `outdated`). Test:
`a_frozen_dream_is_replayed_into_a_session_of_the_dreamed_arm`
(`crates/margincode-bench/tests/runner.rs`). The commands:

```bash
OUT=bench/results/2026-11-13-standing-and-dreams
# stage A: six passes from the same start (wrong-lesson pool plus 50 noise notes)
for i in 1 2 3 4 5 6; do
  margincode-bench dream-freeze bench/tasks-relay-wrong --wrong-reverted --noise 50 \
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
