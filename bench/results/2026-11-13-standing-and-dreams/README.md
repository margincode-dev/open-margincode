# M1 Part 1, the free replay: results, 2026-11-13

No agent ran, no model was called, nothing was spent. The registration is `PREREG.md` in this
directory; its "Build" section (Part 1) was committed in `597be88` before the first `first-packs`
command. Binaries built from `850359b` (tree `e7fcaa3`).

## Commands as run

Inside the harness container `m1-harness` (`rust:1.97.1`, this checkout at `/src`,
`PATH=/target/release:$PATH`), against the release daemon at `127.0.0.1:7450` described under
"Build":

```bash
OUT=bench/results/2026-11-13-standing-and-dreams
COMMON="--av-url http://127.0.0.1:7450 --work-root /root/bench-work --import-dir /root/bench-work/src --import-mount /root/bench-work/src"
margincode-bench check bench/tasks-relay-wrong bench/tasks-relay        # all 15: ok
margincode-bench first-packs bench/tasks-relay-wrong --wrong-reverted --out $OUT/m1-reverted $COMMON
margincode-bench first-packs bench/tasks-relay-wrong                  --out $OUT/m1-open     $COMMON
for n in 0 50 500; do
  margincode-bench first-packs bench/tasks-relay --noise $n --out $OUT/m1-true-$n $COMMON
done
```

All five exited 0, one after the other on one daemon and one database, in the order above
(noise seed 1). Each wrote `first-packs.jsonl`, `first-packs.md` and `packs/`.

## B1: the wrong record is not at `summary` or `full` in the hook pack, at least 4 of 5

| task | `m1-reverted` hook pack | `m1-open` hook pack | `m1-reverted` pre-read has it | `m1-open` pre-read has it |
|---|---|---|---|---|
| relay-coupon-remove | handle | summary | yes | no |
| relay-line-taxes | handle | summary | yes | no |
| relay-money-plain | handle | summary | yes | no |
| relay-quote-split | handle | summary | no | no |
| relay-store-count | handle | summary | yes (`store/base.py`) | no |

**B1: 5 of 5, pass.** In `m1-reverted` the wrong record is at the `handle` tier in every hook
pack, listed as `decision, reverted: …` and cut to 70 characters. In `m1-open`, the same build
without the flag, it is at `summary` in 5 of 5, so B1 passing is the flag's doing, not the
build's: without it 0 of 5 tasks meet the bar.

Not a bar, and reported because it bears on Part 2: with the flag, the pre-read answer for a
relevant file carries the wrong record in 4 of 5 tasks (a pre-read answer carries the whole
summary), against 0 of 5 without it. "Lesson shown in full" in `first-packs.md` is therefore 4
of 5 in `m1-reverted` and 5 of 5 in `m1-open`. The demotion moves the wrong note from the hook
pack into the pre-read answer more than it takes it away from the agent. (Why the pre-read
answer is empty of it in `m1-open` is inferred, not checked: the hook pack already sent it in
full under the same session, and the pre-read answer leaves out what the session was given.)

## B2: on the true-lesson set, every lesson shown in full, 10 of 10 at each N

| N | shown in full | hook pack: summary / handle / absent | in full only through pre-read |
|---|---|---|---|
| 0 | **10 of 10** | 10 / 0 / 0 | none |
| 50 | **10 of 10** | 7 / 2 / 1 | coupon-remove, money-plain (handle); order-ids (absent) |
| 500 | **10 of 10** | 8 / 0 / 2 | order-ids, refund-event (absent) |

**B2: pass at N = 0, 50 and 500.** Relevant files named in the hook pack: 14 of 14 at each N; no
pack marked degraded.

## Decision

B1 and B2 both pass, so the registration's decision rule allows Part 2. Part 2 was not run.

## Deviations and notes

- **The `check` command** is not among the registration's commands; it was run on both task
  sets before the first replay, as `bench/README.md` asks, and changed nothing.
- **Docker engine restart before the build.** During the first build attempt (it had not got past
  updating the crates.io index), every container on the machine exited at once with code 255
  (16:27 UTC). The `m1-` containers were started again and the build was rerun with `-j 6`;
  nothing of the measurement ran before that. The cause was not determined.
- **The tallyho repo** was first built from the bind-mounted checkout, where Docker Desktop shows
  every file as mode 755; it was removed and rebuilt from `git archive HEAD bench/tallyho`, so the
  files keep their committed modes. Only the second one was used.
- Run ids are `fp<pid>` per command (the harness's own); the workspaces were removed after each
  task (no `--keep-work`).

## M2 stage A: a mechanics check with no model (free), 2026-11-15

Not one of stage A's six passes, and no bar is read from it. Before any paid dream run, one
`dream-freeze --until-empty` pass was made on stage A's own pool with a stub agent that calls no
model, to see what the harness does at full size: how many runs a pass takes and whether every
pool note is handed to one. Build: `56968ed` (PR #7), release binaries in `rust:1.97.1`, the daemon
and stack of Part 1's recipe on a fresh database (containers `m2-*`), the same options.

```bash
margincode-bench dream-freeze bench/tasks-relay-wrong --only relay-quote-split \
  --wrong-reverted --noise 50 --until-empty \
  --dream-cmd 'sh /root/stub-dreamer.sh' --verify-cmd 'sh /root/stub-dreamer.sh' \
  --out /root/dry/dreamed-dry3.json <the stack's options>
```

The stub (`sh` and `jq`): in review mode it confirms every item, quoting the first line of the
item's anchored code with at least 12 non-space characters; in verify mode it agrees with every
verdict, quoting the record. So it says nothing about judgement, only about the loop.

- **61 records handed** (the open task's 5 true lessons and 50 noise notes; the reverted task's
  5 wrong notes and the note its revert wrote, "merge … reverted by bench-merger …"), **5 review runs** at the default limit and **3 verify runs**; 60 verdicts, all 60 effective.
- Every wrong note (5) and every true lesson (5) was handed to a run; the command exited 0.
- **1 of 61 left undecided**: a record whose anchored code has no line the stub could quote. It
  was handed again in run 5, which then found nothing new, and the loop stopped as designed.
- Two earlier stub attempts are not counted: in the first, the stub quoted only the record, and
  the daemon refused every `confirmed` verdict ("a confirmed verdict needs a quote of the code it
  was checked against"); the second failed on a jq escape. Both ended at the stub, not the harness.

What it predicts for the paid passes: about 5 review and 3 verify runs a pass, about 48 dream runs
for stage A, as the registration's harness paragraph estimated.

## M2 stage A: the pilot (paid), declared before it runs, 2026-11-15

Not one of stage A's six passes, and no bar is read from it. It is run on **another pool** than
stage A's, the true-lesson relay set (`bench/tasks-relay`, `--only relay-quote-split --noise
50`, no `--wrong-reverted`), so that no stage A outcome is seen before the passes, and with
`--until-empty --max-runs 1`: one review run and one verify run. It measures what one real-model
run costs and whether the agent can do the job (no permission denials, no errors, verdicts the
daemon accepts). After it, only mechanics may change (tools, permissions, caps); `DREAM.md` and
the prompt are not changed.

- **Agent.** Claude Code 2.1.325 with an Anthropic API key (passed into the container by name,
  never written to a file or a command), the same model for dreamer and verifier, as two agents
  (`dreamer`, `verifier`). The pilot runs `claude-sonnet-5-5`.
- **Model rule, fixed now.** Projected cost of the six passes = 6 × (5 × the pilot's review run +
  3 × its verify run), from the stub check's run counts. If that is at most $14, the passes run
  on Sonnet 5.5. Otherwise, if the same projection for Haiku 5.5 (Sonnet's cost × 0.05, its price
  ratio) is at most $14, on Haiku 5.5. Otherwise stage A is not run and the owner is asked.
- **Budget.** $20 at list prices for all of it, counting the smoke call ($0.0009) and the pilot.
  Each run's cost is Claude Code's own `total_cost_usd`, one file per round
  (`claude-result-$AV_DREAM_ROUND.json`), summed over every file. Before each pass the money left
  must be at least 1.5 × the measured cost of a pass; if it is not, the passes stop there.
- **An aborted pass** (a run hits its cap or fails, and `dream-freeze` stops) is reported as a
  deviation and run again from the same start; it is not counted as one of the six.
