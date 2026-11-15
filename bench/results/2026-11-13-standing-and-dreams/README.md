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

### Pilots 1 and 2, as run (true-lesson pool, `--max-runs 1`, Sonnet 5.5, $2 cap per run)

| | agent's tools | review run | verify run | verdicts | undecided / handed |
|---|---|---|---|---|---|
| pilot 1 | `Read,Write,Bash(av:*),Bash(jq:*)` | $0.171, 11 turns, 4 denials | $0.095, 11 turns, 5 denials | 5 | 15 / 20 |
| pilot 2 | `Read,Write,Bash` | $0.128, 11 turns, 0 denials | $0.056, 6 turns, 0 denials | 2 | 18 / 20 |

- **Pilot 1 is a mechanics failure.** Claude Code's permission check denied every command that
  used `$AV_REPO` (`av show PATH --repo "$AV_REPO"`, as `DREAM.md` writes them), so both agents
  judged from the bundle's excerpts alone and said so. Fix, a permission (allowed by the rule
  above): `--allowedTools "Read,Write,Bash"` inside the throwaway harness container.
- **Pilot 2: no denials, no errors, every verdict accepted** (no `refused.json`). The reviewer
  judged 2 of 20 (both `confirmed`, both relay lessons) and left 18 out, saying the attempt,
  convention and profiling notes cannot be confirmed or contradicted from the anchored code. That
  is `DREAM.md`'s own rule ("leave an item out when you cannot tell"), and it is not changed.
- Spent: $0.0009 (smoke call) + $0.266 (pilot 1) + $0.184 (pilot 2) = **$0.45** of $20.

**What pilot 2 means for coverage.** The daemon's worklist (`dream::worklist`) is the top
`limit` records by a fixed score (+3 from reverted work, +2 changed anchor, +2 clustered, shown
in packs, +1 never had a verdict), ties by record id. A judged record leaves the candidates
while its verdict is proposed; one left undecided keeps its score and its slot. So every record
is handed only once the review runs have judged `records − limit` in all, before a run that
judges nothing new. On stage A's pool at the default limit (20) that is 41. The stub check's
first worklist held all 5 wrong notes but only 3 of the 5 true lessons, and the other two sit in
the score-1.0 tier, 49 records in random id order. A reviewer that judges about 2 in 20 will
not reach 41, and a pass would end "not covered" by the harness's rule. At the daemon's largest
limit, 50, it needs 11.

### Pilot 3, declared before it runs: one whole pass at `--limit 50`, off the stage A pool

```bash
margincode-bench dream-freeze bench/tasks-relay --only relay-quote-split --noise 50 \
  --until-empty --limit 50 --max-runs 6 \
  --dream-cmd 'sh /root/dreamer.sh' --verify-cmd 'sh /root/verifier.sh' \
  --out /root/m2/pilot3.json <the stack's options>
```

The true-lesson pool again (60 records, so 10 must be judged), no bar read. It measures how
many records a reviewer judges per run at 50 and what a whole pass costs.

- **Go / no-go, fixed now.** If pilot 3 hands every pool lesson (the harness exits 0), stage A
  runs at `--limit 50`; that is a deviation from "at the default limit", recorded in Build and
  below, and 50 items a run may itself change how the reviewer judges. If it does not, stage A
  does not start, and the owner chooses between a harness that dreams per area (so each worklist
  fits the limit; a rebuild and a new Build) and an amended coverage rule.
- **Model rule, applied to the measured pass.** Projected cost = 6 × pilot 3's whole pass cost
  (the declared 5 + 3 runs assumed a reviewer that judges everything); Sonnet 5.5 if that is at
  most $14, as declared.
- **An uncovered pass of stage A counts as one of the six** (it is reported, and it does not
  meet the bars). It is not an aborted pass, and it is not run again: that would be choosing
  passes. An aborted pass (a run fails or hits its cap, and `dream-freeze` stops) is still run
  again from the same start, as declared.
- **Isolation.** From here the runs use binaries copied to `/root/m2bin` (the same sha256s as
  under Build) and a `git archive 0a3a141 bench` snapshot in `/root/m2src`, so a rebuild or a
  checkout change elsewhere on the machine cannot reach them; the daemon's pid is checked before
  each pass.

### Pilot 3, as run: covered, so stage A goes at `--limit 50`

| run | mode | items | verdicts | cost | turns |
|---|---|---|---|---|---|
| 1 | review | 50 | 5 | $0.177 | 12 |
| 2 | review | 50 | 2 | $0.152 | 10 |
| 3 | review | 50 | 1 | $0.169 | 13 |
| 4 | review | 50 | 5 | $0.196 | 12 |
| 5 | review | 47 | 0 | $0.148 | 10 |
| 6 | review | 47 | 0 | $0.179 | 13 |
| 7 | verify | 13 | (2 rounds; 1 quote refused, fixed in round 2) | $0.182 | 15 |

- All 60 records handed (run 5 was handed the last 2; run 6 found nothing new and the loop
  stopped, which was also `--max-runs 6`); the command exited 0. 13 verdicts, 12 effective; 5
  of the pool's 10 lessons judged, none `disputed` or `outdated` (no bar is read here).
  Undecided 47 of 60. No permission denials, no errors.
- **Whole pass $1.203.** Projected six passes: 6 × $1.203 = **$7.22**, at most $14, so stage A
  runs on **Sonnet 5.5**. Spent so far $1.65 of $20.
- Go: stage A runs at `--limit 50` (deviation from "at the default limit"; see Build). Stage A's
  pool has 61 records, so its passes need 11 judged before a run that judges nothing new.

## M2 stage A: results, 2026-11-15

Six passes of the registered command (PREREG, Build "M2 stage A", `83706ff`), one at a time, all
on Sonnet 5.5 at `--limit 50`. None aborted, none was run again, and each exited 0 (covered:
every wrong note and every true lesson was handed to a run). Files: `m2-stage-a/dreamed-N.json`
(what each pass ended with) and `m2-stage-a/pass-N.log` (its output and summed cost).

| pass | review / verify runs | verdicts (effective) | wrong notes `disputed` and effective | true lessons judged | true lessons `disputed`/`outdated` | undecided / handed | cost |
|---|---|---|---|---|---|---|---|
| 1 | 4 / 1 | 15 (15) | **4 of 5** | 0 | 0 | 40 / 55 | $0.725 |
| 2 | 4 / 1 | 7 (7) | **4 of 5** | 0 | 0 | 48 / 55 | $0.754 |
| 3 | 4 / 1 | 5 (5) | **4 of 5** | 0 | 0 | 50 / 55 | $0.567 |
| 4 | 5 / 1 | 6 (5) | **4 of 5** | 0 | 0 | 50 / 56 | $0.874 |
| 5 | 4 / 1 | 7 (6) | **4 of 5** | 1 (disputed, rejected by the verifier) | 0 | 49 / 56 | $0.675 |
| 6 | 5 / 1 | 7 (7) | **4 of 5** | 0 | 0 | 50 / 57 | $0.817 |

**Bars (registered, unchanged):**

- **Wrong notes: met.** At least 4 of 5 `disputed` and effective in 6 of 6 passes (the bar asks
  for 5 of 6). The same four every time (store-count, line-taxes, coupon-remove, money-plain).
  The fifth, **quote-split's** ("Split a Money amount with total.times(Decimal(1) / n) …
  Money.allocate(n) hands out uneven parts … do not use it for splits"), was **`confirmed`**, by
  the reviewer and agreed by the verifier, in all six passes, each time on `allocate`'s own
  lines (`base, rest = divmod(self.minor, parts)` and the remainder handed out a cent at a time).
  Its factual half is true read literally (the parts can differ by a cent); its advice is the
  dead end. It is stage B's target task's own wrong note, so a dreamed store would carry it
  *confirmed*.
- **True lessons: met, but tested in one pass only.** No true lesson ended `disputed` or
  `outdated` in any pass. In five of the six passes no true lesson was judged at all: all five
  were handed and left undecided, so the bar held without being put to the test. In pass 5 the
  reviewer judged one, customer-region's ("Errors about customer data must not echo the
  rejected value …"), and **disputed** it, quoting `check_region`'s `raise ValidationError(f"invalid
  region {region!r}", …)`: the code on `main` breaks the rule, because that is the bug the
  lesson's task fixes. The verifier did not agree, so the verdict was `rejected` and never took
  effect. The bar holds because of the second agent, on the one occasion it was tested.
- **Quotes: met as far as stage A can show.** Every verdict and every review the six passes
  posted passed the daemon's quote check (no `refused.json` in any run). The second check, when
  `run --dreamed` writes the frozen verdicts into a freshly seeded store, first runs in stage B.

**Decision.** Stage A meets its bars as registered, so the registration allows stage B. Read with
the two notes above: one wrong note is confirmed every time, and the true-lesson bar was
exercised once.

**Reported, not bars:**

- **Undecided share 287 / 334 = 86 %** over the six passes (records a review run was handed that
  no run gave a verdict). The plan's Phase 5 (transcripts) is gated on this going above 30 %; it
  is far above.
- **Not every record was handed.** 55 to 57 of 61 a pass. The harness stops a pass when a run
  is handed nothing new, but the reviewer is not deterministic on records it is handed again:
  in pass 1, run 4 was handed the same 50 records as run 3 (which judged none) and judged 10.
  The 4 to 6 records never handed were noise notes each time, so the bars are not affected (the
  coverage rule is about the pool's notes), and the undecided share is over handed records only.
- Verify runs judged every proposed verdict in one run each; 2 of 47 verdicts were rejected
  (pass 4: a `confirmed` on a noise note; pass 5: the dispute above).

**Cost.** Stage A's six passes **$4.41** (Claude Code's `total_cost_usd`, list prices); with the
smoke call and the three pilots ($1.65), **$6.07 of the $20** budget. The plan's estimate for all
of M2 was about $2.
