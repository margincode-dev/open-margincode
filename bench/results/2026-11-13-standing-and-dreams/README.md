# M1 Part 1, the free replay: results, 2026-11-13

(M1 Part 2, the paid run, is at the end of this file: "M1 Part 2, paid: results, 2026-11-16".)

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
  the reviewer and agreed by the verifier, in all six passes, each time on the same two lines of
  `allocate` (`base, rest = divmod(self.minor, parts)` and the remainder handed out a cent at a time).
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
- **Quotes: met.** Every verdict and every review the six passes posted passed the daemon's
  quote check when posted (no `refused.json` in any run). Re-checked afterwards, offline: each
  of the 100 stored `code` and `record` quotes of the six `dreamed-N.json` is found verbatim in
  that file on the tallyho repo's `main` (the tree every pass's quotes were checked at) or in that
  record's text; 0 failed (`m2-stage-a/recheck.py`, run in the harness; one `event` reference, in pass
  5, is checked by the daemon only). The daemon's own re-check, when `run --dreamed` writes the
  frozen verdicts into a freshly seeded store, first runs in stage B.

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
  The 4 to 6 records never handed were noise notes each time (listed by `m2-stage-a/recheck.py`; the
  revert's own note was handed in every pass), so the bars are not affected (the
  coverage rule is about the pool's notes), and the undecided share is over handed records only.
- Verify runs judged every proposed verdict in one run each; 2 of 47 verdicts were rejected
  (pass 4: a `confirmed` on a noise note; pass 5: the dispute above).

**Deviations from the registration**, all decided before pass 1 and none after a stage A
outcome was seen: `--limit 50` instead of the default 20 (Build, and "Pilot 3" above); three
pilots, declared here and not in the PREREG; the agents' tools widened to `Read,Write,Bash`
after pilot 1's denials; a database that was not fresh (it held the stub check's and the
pilots' stores; each pass seeded its own repo); the runs from `/root/m2bin` and a `bench`
snapshot of `0a3a141`.

**Cost.** Stage A's six passes **$4.41** (Claude Code's `total_cost_usd`, list prices); with the
smoke call and the three pilots ($1.65), **$6.07 of the $20** budget. The plan's estimate for all
of M2 was about $2.

## 2026-11-15: the review worklist after stage A

Stage A ran before the daemon set declined records aside (binaries built at `56968ed`). Since
then a record that a finished review run was handed and left without a verdict waits behind the
other records until it scores higher than it did in that run (#13 made the waiting permanent;
the change after it, branch `dream-declined-lapse`, makes it lapse when the score rises; see
`CHANGELOG.md`). The first review run on a freshly seeded store is ranked exactly as before, since
no finished run exists yet, so stage B's one frozen pass is not affected. Stage B's Build must
name the daemon commit it runs.

What the rule implies, not a measured result (nothing was re-run): an `--until-empty` pass like
stage A's would be handed every record of the pool, at any `--limit` (given enough
`--max-runs`), instead of 55 to 57 of 61, since a run that judges none of what it was handed no
longer gets the same records again while others wait.

## M1 Part 2, paid: results, 2026-11-16

**Not shown at this size.** On Claude Haiku 5.5, with each task's wrong note seeded as the record
of a task whose merge was reverted, `av` with Phases 0 and 1 (build P1, `84bb472`) was accepted
**30 of 30** (Wilson 0.886 to 1.000), and `av` with Phase 0 alone (build P0, `90ba62e`) was also
**30 of 30** (0.886 to 1.000). The intervals overlap, so by the registered rule nothing is
claimed. Phase 1 is not shown to help or to hurt on these tasks. 60 sessions cost $0.3605, and
with the smokes $0.3766.

This is the outcome the prediction committed with Build expected. Build P0 already solved every
task, so a gain from Phase 1 could not show. At 30 sessions an arm can only be shown to hurt.

The registration is `PREREG.md`, "Part 2". Its Build subsection ("Part 2 (M1, paid)") was
committed in two parts before the first paid session: `d980fd9` with the builds, the stack, the
commands, the rules and the free checks' conditions, and `e0ba268` with the free checks'
results. The smokes are in `bbdfd07`. The leak and hidden-test checks and the store export are in
`77a5b90`, committed before any accepted count was read. The files are in `m1-part2/`. No claim
is made beyond these three tasks, this model and tallyho.

### Results (`m1-part2/checks/analyze.txt`, `analyze.py` as committed)

| arm | build | valid | accepted | 95 % Wilson | cost | median session |
|---|---|---|---|---|---|---|
| `av-p0` (Phase 0) | P0, `90ba62e` | 30 | 30 | 0.886 to 1.000 | $0.1788 | $0.0060 |
| `av-p01` (Phases 0 and 1) | P1, `84bb472` | 30 | 30 | 0.886 to 1.000 | $0.1817 | $0.0059 |

Each task, in each arm, was accepted 10 of 10 (`relay-quote-split`, `relay-bus-resubscribe`,
`relay-tax-with-rate`). Every row is `status=ok`, and no session was skipped for budget.

Described beside the rule, not used for it:

- **Delivery of the task's wrong note.** In P1, the session-start hook showed it in 30 of 30
  sessions and labelled it `reverted` in all 30. It was whole (`summary` tier) in only 10, all
  of them `relay-tax-with-rate`. On `relay-quote-split` and `relay-bus-resubscribe` it was cut to
  the `handle` tier, as in the replay. A pre-read answer carried it whole in 18 P1 sessions. In
  P0 the hook showed it whole in 29 of 30, with no origin, and a pre-read answer carried it whole
  in 1. The one P0 session without it, `relay-tax-with-rate` rep 3, got a session-start pack with
  the map and code but no records at all. Its pre-read answer for `tallyho/pricing/tax.py` then
  carried the note whole, and the session was accepted. Phase 0's line was in the hook of the 29
  P0 sessions whose pack had records, and of all 30 P1 sessions.
- **Superseded records, read from the store** (`m1-p0/superseded.jsonl`, `m1-p1/superseded.jsonl`).
  In P0, 11 sessions wrote a record that supersedes their task's wrong note: 8 on
  `relay-tax-with-rate` and 3 on `relay-quote-split`. One of those quote-split sessions also
  superseded another task's wrong note (line-taxes'), and no session superseded a true note. In
  P1 no session superseded anything (0 of 30). All 60 sessions were accepted, whether or not they
  corrected the note. With the note demoted and labelled `reverted`, the P1 agents left it alone
  rather than correcting it.
- **The wrong edit** (`repeated_dead_ends` ≥ 1). 1 session matched it, P1 `relay-bus-resubscribe`
  rep 7, and it was still accepted, since its final file passed the hidden test. P0 had 0.
- **Last edits** (`checks/content.txt`). No session failed, so the table is empty.
- **Cost.** P1 cost 1.6 % more than P0 ($0.1817 against $0.1788), and $0.0061 against $0.0060 per
  accepted task. The largest single request's prompt was 32,011 tokens, under the 100K price
  threshold. The harness figure at list prices matches Claude Code's own `total_cost_usd` to
  the cent in every directory.

Set beside M0 as a description, not a comparison (another run, without `--wrong-reverted`): M0's
`av-phase0` (the same binaries as P0) was 30 of 30 on these tasks, and P0 here is 30 of 30 too.
Seeding the wrong notes as reverted work did not cost build P0 anything.

**The leak check** (`m1-part2/checks/leak.txt`, `exposed.json`) found 0 exposed sessions of 60
and no flagged call. Outside a session's own directories, the agents named `/` (counted as
"looked around": 16 P0 sessions, 11 P1), their own build's `av` directory (listed, not exposing),
`/dev/null`, a `/tmp` file in one P1 session, and pieces of code that the path pattern picks up
(`/from`, `/n`, `/3`, `/d`, `/import`). The hidden-test scan
(`checks/content-hidden.txt`) found 0 hits in 60 sessions. The tables without exposed sessions
are therefore the same.

### What ran

| command | sessions | rows | harness figure | time (UTC) |
|---|---|---|---|---|
| 0. smoke, build P0 (`smoke-p0/`) | 1 | 1 `ok` | $0.0075 | 01:49 to 01:50 |
| 0. smoke, build P1 (`smoke-p1/`) | 1 | 1 `ok` | $0.0086 | 01:50 to 01:51 |
| 1. `av-p0`, build P0 (`m1-p0/`) | 30 | 30 `ok` | $0.1788 | 01:51 to 02:10 |
| 2. `av-p01`, build P1 (`m1-p1/`) | 30 | 30 `ok` | $0.1817 | 02:10 to 02:28 |
| **total** | **62** | | **$0.3766** | |

Commands 1 and 2 ran back to back from `m1chain.sh`, with nothing between them. **The smokes
pass** (`checks/smoke-check.txt`, `smoke_check.py`). Every row is `ok` and valid, and the model
answered and made 4 and 5 edits. Verify ran, and `hook-count` shows session-start, pre-read,
post-edit and stop. `meta.json` names `benchagent`, Claude Code 2.1.318 and
`models: ["claude-haiku-5-5"]`, and every assistant message is from `claude-haiku-5-5`. The
largest prompt was 33,887 tokens, and the dearer smoke cost $0.0086, under $0.04. The store
export (`export.sh`) ran once after command 2: P0 has 12 superseding records and 45 records by the
sessions' agents, and P1 has 0 and 37.

### Deviations

- **One harness for both arms.** Neither build's own harness has `--wrong-reverted`, which came
  after the rename, so both arms ran `main`'s harness (`7dd5208`) against their own daemon and
  `av`. This was declared in Build before any check. The replay showed that the harness's seeding
  works on both daemons.
- **The first P1 build was not P1.** The first `cargo build` of `84bb472` reused a cache copied
  from M0's build A target directory. The archive's source files carry the commit's older
  timestamps, so cargo kept build A's `scholion-*` binaries (`av` had build A's size). This was
  seen from the timestamps before any check ran. The binaries were thrown away and rebuilt from
  an empty target directory, and the hashes under Build are that clean build. Nothing ran on the
  first one.
- **Free check 1 ran before its conditions were committed.** These are M0's task-set checks, with
  M0's conditions. `tasks-m0/check_set.py` was run on `/m1/srcH`, the archive before the model
  lines changed. On the run tree its byte-for-byte checks against `tasks-relay-silent` and
  `tasks-relay-wrong` fail by design (`checks/check-m0.txt` holds the passing run).
- **The preflight refused both smokes once** (`checks/smoke-p0.refused.log`, `smoke-p1.refused.log`),
  before any session. The replay had left its agent-task workspaces under `ws-p0` and `ws-p1`
  open. They were closed (mode 700), and the same commands ran again. This is the same thing that
  happened in M0.
- **The databases are not empty of everything but the run.** They hold the replays' repositories
  (`fp<pid>-…`). Every session imports and seeds a repository of its own, as in M0.
- **The branch** started from `306bf0b` (`bench-m0-run`, PR #15) while #15 was open. #15 was
  merged during the run without changing that commit, so the branch is off `main`'s history.

### Files

- `PREREG.md`, "Part 2 (M1, paid)": the Build. `m1-part2/`: `analyze.py`, `leak.py`, `content.py`,
  `replay_check.py`, `smoke_check.py`, `export.sh`, `m1cmd.sh` and `m1chain.sh`. The first three
  and the two run scripts were committed with the Build.
- `m1-part2/first-packs-p0/`, `first-packs-p1/`: the free replay. `smoke-p0/`, `smoke-p1/`,
  `m1-p0/` and `m1-p1/` are what the harness wrote, with the store exports in the last two.
- `m1-part2/checks/`: the free checks, the harness logs, `leak.txt`, `content-hidden.txt`,
  `content.txt` and `analyze.txt`.
- Transcripts are not committed. They stay in `lp-harness` under `/m1/bench-work/runs`, closed to
  the agent user.

## M2 stage B: raw against dreamed memory, results, 2026-11-18

Run as `AMENDMENT-1-stage-b.md` registers it (committed and merged in #52 before the first
session): one build of `main` at `aa2547b`, one daemon with `AV_PLAIN_RECORDS=on` (both arms'
`meta.json`: `plain_records: true`), the M0 tasks, Claude Haiku 5.5, 30 sessions per arm, back to
back (raw 04:58 to 05:14 UTC, dreamed 05:14 to 05:29). Results in `m2-stage-b/` (`m2-raw/`,
`m2-dreamed/`, `smoke/`, `dreamed.json`, `checks/`).

**Primary (pooled, valid sessions; Wilson 95 %, `margincode_bench::stats`):**

| arm | valid | accepted | rate | 95 % Wilson | cost |
|---|---|---|---|---|---|
| raw | 30 | 14 | 0.467 | 0.302 to 0.639 | $0.1615 |
| dreamed | 30 | 21 | 0.700 | 0.521 to 0.833 | $0.1500 |

**Decision by the claim rule: not shown at this size.** The intervals overlap (dreamed's lower
end 0.521 is below raw's upper end 0.639).

**Per task** (described, below the claim size): quote-split raw 0/10, dreamed 1/10;
bus-resubscribe raw 4/10, dreamed 10/10; tax-with-rate raw 10/10, dreamed 10/10.

**Beside the primary.**

- Dream cost: $0.5677 for both passes on the three tasks (6 Sonnet 5.5 calls, Claude Code's
  `total_cost_usd`).
- Verdicts each task's own wrong note ended with: bus-resubscribe `disputed` (effective);
  quote-split **`confirmed`** (effective, verified, as in all six stage A passes); tax-with-rate
  undecided (handed, not judged). Over all three stores 17 wrong-note verdicts were `disputed`,
  3 `confirmed` (quote-split's note, once per store); one true lesson ended `disputed`
  (`customer-region`, in the bus-resubscribe store); none `outdated`. Undecided: 5, 6 and 7 of
  13 handed.
- **The confirmed wrong note.** quote-split's note ended `confirmed`, and quote-split is the task
  where both arms failed (raw 0/10, dreamed 1/10). With the note confirmed the dream changed
  nothing for it, so the task gives the dreamed arm no room to move. Not claimed: 10 sessions a
  task is below the claim size.
- Sessions that tried `--supersedes` on another task's record: 0 of 60.
- Smoke (order-ids, raw, not a result): passed; $0.0070, largest request 32 K tokens.

**Spend:** stage B $0.886 (freeze $0.568, smoke $0.007, raw $0.162, dreamed $0.150), under
the $5 ceiling. The largest session cost $0.0078.

**Deviations and stops** (none changes the question, the arms or the claim rule):

- The first freeze attempt was refused by the API (a bad key, $0). After the key was replaced and
  the host restarted, the same command ran again on the same binaries, daemon setting and
  database. Its Build records both attempts.
- The smoke preflight refused twice before any session (directories left open to the agent user
  by `first-packs` and the freeze, and the agent user's home). Fixed by closing them;
  `m2bchain.sh` was changed after the amendment's commit to also close the kept workspaces
  between the arms, so the dreamed arm's preflight would not refuse. `docs/backlog.md` has it.

## M2 stage B, amendment 2: raw against dreamed on the revised `DREAM.md`, results, 2026-11-19

Run as `AMENDMENT-2-stage-b-revised.md` registers it. Both of its commits were pushed on the
branch `m2-stage-b-revised` before the first session; they were not merged first, as stage B's
amendment was. One build of `main` at `21f30dc` (the revised `DREAM.md`), one daemon with
`AV_PLAIN_RECORDS=on` (both arms' `meta.json`: `plain_records: true`), one new freeze, the M0
tasks, Claude Haiku 5.5, 90 sessions per arm. The smoke ran 05:31 to 05:32 UTC, raw 05:32 to 06:20,
dreamed 06:21 to 07:05. The build lock was free before each; the gap between the arms was 40 s.
Results are in `m2-stage-b-revised/` (`m2-raw-r2/`, `m2-dreamed-r2/`, `smoke-r2/`, `dreamed.json`,
`first-packs-*`, `checks/`). The sessions do not pool with stage B's 30 a side.

**Primary (pooled, valid sessions; Wilson 95 %, as `margincode_bench::stats` computes it):**

| arm | valid | accepted | rate | 95 % Wilson | cost |
|---|---|---|---|---|---|
| raw | 90 | 47 | 0.522 | 0.420 to 0.622 | $0.4846 |
| dreamed | 90 | 90 | 1.000 | 0.959 to 1.000 | $0.4481 |

**Decision by the claim rule: dreaming helps.** The intervals are disjoint: dreamed's lower end,
0.959, is above raw's upper end, 0.622. Both arms have 90 valid sessions, 30 per task.

**What the claim covers, and what it does not.** It covers this one frozen dream, on these three
tasks, on Haiku 5.5. The dreamer's text (`DREAM.md`, #63) was revised after stage B's results
and fitted through three in-sample pilots on these same three stores. So this shows that a dream
from text tuned on these tasks helps on these tasks. It does not show that the revised text
helps on other tasks. Ninety sessions over three tasks are not ninety tasks (M0's
pre-registration). The freeze was made once and used as it came out.

**Per task** (described; 30 sessions a task):

| task | raw | dreamed |
|---|---|---|
| quote-split | 7/30 | 30/30 |
| bus-resubscribe | 10/30 | 30/30 |
| tax-with-rate | 30/30 | 30/30 |

tax-with-rate was 30 of 30 in both arms, as it was 10 of 10 in both in stage B. The difference is
on the two tasks where raw had room to move. Each of those tasks' own wrong notes was `disputed`
in the frozen dream, where stage B's freeze had confirmed quote-split's note.

**Beside the primary.**

- **Dream cost:** $0.6079 for both passes on the three tasks (6 Sonnet 5.5 calls, Claude Code's
  `total_cost_usd`; none hit the $0.25 cap).
- **Verdicts the freeze ended with** (all verified, `effective`): each task's own wrong note
  ended `disputed` in its own store. Over all three stores 19 of the 24 wrong notes were
  `disputed` (6, 7 and 6 of 8), none `confirmed`. No verdict was `outdated` or `duplicate`.
- **True lessons:** none got a verdict (judged 0 of 5 in each store). So "no true lesson
  `disputed` or `outdated`" holds only because none was judged, as in pilot 3.
- **Undecided:** 7, 6 and 7 of 13 handed.
- **Sessions that tried `--supersedes` on another task's record:** 0 of 180 (no session's
  commands, in its `events.jsonl`, contained `--supersedes` at all; 155 of 180 ran `av`).
- **Repeated dead ends** (the harness's count): raw 42, dreamed 0.
- **Stage B beside this run** (not pooled; the old `DREAM.md`, 30 sessions a side): raw 14 of 30
  (0.302 to 0.639), dreamed 21 of 30 (0.521 to 0.833); per task quote-split 0/10 and 1/10,
  bus-resubscribe 4/10 and 10/10, tax-with-rate 10/10 and 10/10.
- **Median session wall time:** raw 20.7 s, dreamed 18.6 s.
- **Largest session and largest request:** the largest session cost $0.0081. The largest single
  request was about 31 K tokens of prompt, against a 100 K limit.
- **Smoke** (order-ids, raw, not a result): passed under amendment 1's conditions. It cost
  $0.0076, its largest request was 32 K tokens, `hook-count` was not empty, and verify ran.

**Spend:** run 2 cost **$1.548** in all, under the $3.00 ceiling:

| step | cost |
|---|---|
| freeze | $0.608 |
| smoke | $0.008 |
| raw | $0.485 |
| dreamed | $0.448 |

Raw ran with `--budget-usd 2.10`. Dreamed ran with $1.89, the smaller of $2.10 and $3.00 less what
had been spent. Neither budget bound.

**Deviations** (none changes the question, the arms or the claim rule):

- **The daemon was started twice before any command.** The first start had a umask of 077, which
  stage B's daemon did not have, so it was stopped before any request reached it and started
  again with the default umask. The second start served everything above. The amendment's second
  commit records this.
- **The dreamed arm's budget was $1.89, not $2.10,** by the amendment's money rule. It did not
  bind.
- **Run 1's daemon (a separate store and port) was also up during the run.** It was idle; no
  command of this run used it.

## M2 stage B, amendment 3: raw against dreamed on five held-out tasks, results, 2026-11-19

Run as `AMENDMENT-3-held-out.md` registers it, on the branch `heldout-amendment-3`. Its three
commits before the run were pushed before the first paid command and the first session
(GitHub's branch activity): the build and the checks at 13:01:51 UTC, before the freeze started at
13:02:08; the frozen dream and free check 5 at 13:10:17, before the smoke started at 13:10:29.
They were not merged first. Amendment 2's binaries (`main` at `21f30dc`), a run tree of
`21ac445`, one new daemon with `AV_PLAIN_RECORDS=on`, one new freeze, the five tasks of
`bench/tasks-heldout`, Claude Haiku 5.5, 100 sessions per arm. The smoke ran 13:10 to 13:11 UTC,
raw 13:11 to 14:07, dreamed 14:07 to 15:06, the M0 row 15:06 to 15:39, cold 15:40 to 15:53
(`m2-heldout/checks/chain.log`). Results are in `m2-heldout/`. The sessions do not pool with stage
B's or amendment 2's.

**Primary (pooled over the five held-out tasks, valid sessions; Wilson 95 % by
`margincode_bench::stats::wilson_interval`, `m2-heldout/wilson.rs`):**

| arm | valid | accepted | rate | 95 % Wilson | cost |
|---|---|---|---|---|---|
| raw | 100 | 91 | 0.910 | 0.838 to 0.952 | $0.6128 |
| dreamed | 100 | 100 | 1.000 | 0.963 to 1.000 | $0.6180 |

**Decision by the claim rule: dreaming helps.** The intervals are disjoint: dreamed's lower end,
0.963, is above raw's upper end, 0.952. Both arms have 100 valid sessions, 20 per task, over the
registered floor of 50 with 10 a task. The margin is thin: 99 of 100 would have given 0.946 to
0.998, and the intervals would have overlapped.

**Per task** (described, not claimed; 20 sessions a task a side, cold 10):

| task | where the evidence is | raw | dreamed | cold (no notes) |
|---|---|---|---|---|
| order-can-cancel | another file | 20/20 | 20/20 | 10/10 |
| delete-order | another file | 20/20 | 20/20 | 10/10 |
| order-comments | another file | 20/20 | 20/20 | 10/10 |
| ledger-revenue | the anchored file, two lines | 20/20 | 20/20 | 10/10 |
| coupon-days-left | the anchored class, one line | 11/20 | 20/20 | 10/10 |

**What this shows, and what it does not.**

- **The whole difference is one task, coupon-days-left** (raw 11/20, Wilson 0.342 to 0.742;
  dreamed 20/20, 0.839 to 1.000). The amendment named it as the task closest to the notes the text
  was fitted on (a fact contradicted by one line of the anchored code, as coupon-remove and
  money-plain) and kept it as the in-route contrast. Without it, raw and dreamed are 80 of 80 each
  (Wilson 0.954 to 1.000 both): no difference. Without delete-order as well (the other task the
  amendment called close to a fitted note, store-count), 60 of 60 each.
- **On the other four tasks raw had no room.** Raw was 20 of 20 on each, so the sessions did not
  test whether a dream helps against a wrong note whose evidence is in another file. The three
  cross-file tasks, which were what this set added, are 20/20 in both arms. Every raw session's
  transcript holds the start of its task's wrong note (20 of 20 on each task,
  `m2-heldout/checks/delivery.txt`), so the note reached the agent (in the hook pack, at least as
  its handle line, or in a pre-read answer) and Haiku did not follow it on those four.
- **On coupon-days-left the note misled raw.** Cold, with no notes, was 10 of 10 on it. All 9
  failed raw sessions ended with the note's formula, `(expires - today).days` floored at 0, in
  their `days_left`; one of them (rep 18) first wrote the correct `+ 1` and then replaced it with
  the note's version, and one (rep 16) also wrote `today >= self.expires` into `usable`.
  In the dreamed arm the note carried its `(disputed: A coupon is still usable on its expires day
  …)` mark in all 20 transcripts.
- **What the freeze did is the only held-out evidence on the cross-file notes, and it is beside
  the claim, not under it.** One freeze, ten Sonnet 5.5 calls: every wrong note `disputed` in every
  store (25 of 25, so 15 of 15 on the three cross-file notes), each with a reason that names the
  contradicting code in its own file; every true lesson `confirmed` (25 of 25, none `disputed` or
  `outdated`); nothing left undecided. Where amendment 2's freeze judged no true lesson and left 6
  or 7 of 13 records undecided per store, this one judged all ten. That is one draw of the text on
  these notes.
- **The designer confound bears on all of it, and most on the freeze counts.** The person who wrote
  these five notes had read the revised `DREAM.md` and amendment 2's verdicts, and chose notes a
  dream can dispute from the code. So a clean freeze shows the text handles notes written to be
  disputable by someone who knew it, in a repository its author had seen. It does not show how it
  handles notes written by someone who had not read it.
- It covers one frozen dream, one session model (Haiku 5.5), one repository (`tallyho`) and five
  tasks. A hundred sessions over five tasks are not a hundred tasks.

**The M0 comparison row** (described beside, never pooled, no claim; `bench/tasks-m0`, 10 a task
a side, amendment 2's frozen dream replayed, `meta.json` `dreamed sha256:fe405a5b…`):

| task | raw | dreamed |
|---|---|---|
| bus-resubscribe | 5/10 | 10/10 |
| quote-split | 1/10 | 10/10 |
| tax-with-rate | 10/10 | 10/10 |
| pooled | 16/30 (0.361 to 0.698) | 30/30 (0.886 to 1.000) |

The in-sample gain is there on the same day, stack and build (amendment 2: raw 47/90, dreamed
90/90). So the held-out arms' small gap is not a changed day: the held-out tasks left raw almost
no room.

**Beside the primary.**

- **Dream cost:** $1.2517 for the two passes on the five stores (10 Sonnet 5.5 calls, Claude Code's
  `total_cost_usd`; none hit the $0.25 cap).
- **Verdicts each task's own wrong note ended with:** `disputed` (effective, verified) in its own
  store, 5 of 5; split by where the evidence is, 3 of 3 in another file and 2 of 2 in the anchored
  file. No wrong note ended `confirmed`.
- **True lessons:** 25 of 25 judged, all `confirmed`; none `disputed` or `outdated`.
- **Undecided:** 0 of 10 handed, in each store.
- **A task where raw had no room:** order-can-cancel, delete-order, order-comments and
  ledger-revenue (20 of 20 each). They stay in the primary, as registered.
- **Repeated dead ends** (the harness's count): raw 6, dreamed 2, M0 row raw 14 and dreamed 0, cold
  0. The coupon pattern matches only `max((self.expires - today).days, 0)`, so 3 of raw's 9
  coupon failures (reps 4, 10 and 18, which ended with `max(0, …)`) are not in the 6. Dreamed's 2 are order-comments sessions (reps 7 and 13) whose code comments say they did not
  use `"order-comments"`; both passed.
- **Sessions that tried `--supersedes`:** 0 of 200 held-out sessions (0 of 260 with the M0 row; no
  Bash command contained it). 39 raw and 38 dreamed sessions ran `av`.
- **Median session wall time:** raw 21.7 s, dreamed 20.5 s, cold 15.4 s.
- **Largest session and largest request:** the largest session cost $0.0098; the largest single
  request was about 37 K tokens of prompt, against a 100 K limit.
- **One raw and dreamed pair compared** (coupon-days-left, rep 2; the session's start pack holds no
  records, so the pair is the pre-read answer on `tallyho/pricing/coupons.py`): both give the wrong
  note in full with its anchor; the dreamed one adds `(disputed: A coupon is still usable on its
  expires day: it is rejected only when today > expires., vd:…)`. Raw rep 2 failed with the note's
  formula; dreamed rep 2 passed. In the first packs (`first-packs-ho-raw/`,
  `first-packs-ho-dreamed/`) the dreamed hook packs differ from raw only by the `(disputed)` marks
  and the re-ranking they bring.
- **One dreamed session did not see its note:** dreamed order-comments rep 20's transcript holds
  no part of the wrong note (19 of 20 on that task); it passed.
- **Smoke** (order-ids, raw, not a result): passed under amendment 1's conditions. It cost $0.0063,
  its largest request was 31 K tokens, `hook-count` was not empty, and verify ran.
- **Recounted** a second time on the host from the committed `rows.jsonl` with `grep`
  (`m2-heldout/checks/recount-host.txt`), apart from `analyze.py`: the same counts per arm and per
  task. The copied files' sha256 match the container's.

**Spend:** run 3 cost **$3.0215** in all, under the $5.00 ceiling (`m2-heldout/checks/spend-*.txt`):

| step | cost |
|---|---|
| freeze | $1.2517 |
| smoke | $0.0063 |
| raw | $0.6128 |
| dreamed | $0.6180 |
| M0 row raw | $0.1575 |
| M0 row dreamed | $0.1427 |
| cold | $0.2325 |

Every command ran with its written budget (smoke $0.10, each held-out arm $2.30, each M0-row arm
$0.70, cold $0.40): $5.00 less the spend before each was larger every time. No budget bound.

**Deviations** (dated 2026-11-19; none changes the question, the arms or the claim rule; the
first two are also in the amendment's second and third commits):

- **Free checks before the amendment's first commit.** The checks were first run on a preliminary
  tree, before the amendment was written up, and again on the run tree; the preliminary
  `first-packs` left repositories `bfp164-…` in the run's database. No model ran.
- **Free check 5's first call was refused** for a relative path, after it had seeded one store
  ($0, no verdict). It was repeated with the absolute path.
- **The host's build lock was not checked before each command**, as amendment 2 did. The other
  containers on the host (the `scholion-foundry-*` stack and `mc-dogfood`) were up throughout. The
  gaps between the arms were 13 to 30 s; between the freeze and the smoke, 3 min (the third commit
  and its push).
- **The dead-end pattern undercounts coupon-days-left** (above). The harness's count is reported
  as it came out; the primary does not use it.
