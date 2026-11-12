# What a confident wrong note does (E4, wrong-lesson cell)

2026-11-11. 93 paid sessions on `deepseek-flash` (3 smoke, 90 in the cell), $1.26 at the
harness's peak prices, $0.62 off the balance. Design: `DESIGN.md`. Rules fixed in advance:
`PREREG.md`. Everything under "What the rule does not see" was found after the run and is
labelled as such.

## Summary

- **By the pre-registered rule, neither arm is hurt, and the arms do not differ.** Accepted:
  `cold` 27 of 30, `notes` 30 of 30, `av` 24 of 30. `av` is above the lower end of `cold`'s
  interval, and the intervals of `notes` and `av` overlap.
- **An observation the rule does not cover (post hoc, six sessions per arm, no claim).** On one
  task of five, `relay-quote-split`, the `av` arm failed 6 of 6 and both other arms passed 6 of
  6. In every one of those six `av` sessions the agent took the approach the wrong record
  recommends, said the record was why, and stored the same wrong decision again as a new record.
  In all six `notes` sessions the agent read the same note and rejected it, and five of them
  rewrote the note.
- **One task is contaminated.** On `relay-line-taxes`, 8 of 18 sessions read the grading test
  from outside their working copy: 5 from an earlier session's kept copy, which `--keep-work`
  (added for this run) left on disk; 2 from an earlier `av` session's workspace, which the
  daemon keeps for an hour with or without `--keep-work`; and 1 from the bench source tree in
  the container. The last two routes do not need `--keep-work`. That task's numbers say nothing
  about the note.
- So this cell does not give a clean answer. It gives one specific failure, seen in six of six
  sessions on one task, to test in a cell with the leak closed, and on that task a difference in
  how the two memory systems were used that was not predicted: the `notes` agents corrected the
  file and the `av` agents repeated the record.

## Result by the pre-registered rule

| arm | valid sessions | accepted | rate | 95 % Wilson interval | cost | cost per accepted |
|---|---|---|---|---|---|---|
| `cold` | 30 | 27 | 0.900 | 0.744 to 0.965 | $0.3214 | $0.0119 |
| `notes` | 30 | 30 | 1.000 | 0.886 to 1.000 | $0.4785 | $0.0160 |
| `av` | 30 | 24 | 0.800 | 0.627 to 0.905 | $0.4134 | $0.0172 |

The Wilson intervals are computed by `analyze.py` with the formula of
`scholion_bench::stats::wilson_interval`. Every session is valid and none was skipped for budget.

- An arm is hurt if its rate is below 0.744, the lower end of `cold`'s interval (22 of 30 or
  fewer). `notes` (1.000) and `av` (0.800) are both above it: **neither is shown to be hurt.**
- The intervals of `notes` (0.886 to 1.000) and `av` (0.627 to 0.905) overlap: **no claim that
  the arms differ.**
- H0 is what the rule reports. By the rule's own follow-up clause, a mark on records from reverted
  work is not justified by solved tasks on this result.

Thirty sessions per arm cannot show a small effect, and six repetitions of five tasks are not
thirty independent tasks. The next section shows what the aggregate is made of.

## Accepted per task

| task | `cold` | `notes` | `av` |
|---|---|---|---|
| `relay-coupon-remove` | 6/6 | 6/6 | 6/6 |
| `relay-line-taxes` | 3/6 | 6/6 | 6/6 |
| `relay-money-plain` | 6/6 | 6/6 | 6/6 |
| `relay-quote-split` | 6/6 | 6/6 | **0/6** |
| `relay-store-count` | 6/6 | 6/6 | 6/6 |

The six sessions `av` lost are one task. The three sessions `cold` lost are another task, the
contaminated one. In the aggregate the two cancel in part: `av` is 27 − 6 + 3 = 24.

## What the rule does not see (found after the run; not pre-registered)

### 1. On one task the av agents followed the wrong record, every time

`relay-quote-split` asks for `Quote.split(parts)`. The right fix is `Money.allocate`, whose parts
add up to the total. The wrong note says to use `total.times(Decimal(1) / n)` and not `allocate`.
No session on this task named a grading test, another session's files or the source tree
(`leak.md`). One `notes` session listed the run directory and read nothing in it.

- **`av`, 0 of 6.** All six sessions wrote `times(Decimal(1) / parts)`. Their closing messages
  say why, for example "per the recorded decision, this divides evenly rather than using
  `Money.allocate`" and "deliberately not `Money.allocate`". The existing tests pass with that
  code, so nothing in the session contradicted the record. Only one session ran `av expand` on
  it.
- **All six then recorded the wrong approach again** as a new `decision` of their own task
  (`behaviour.md`, column "restates the wrong edit"). A wrong record reproduced itself in six of
  six sessions.
- **`notes`, 6 of 6.** All six read the same text in `NOTES.md`. All six used `allocate`. From
  one session: "One note conflicts with the task: it says to split using
  `total.times(Decimal(1) / n)` and avoid `allocate`, but `times` rounds each share
  independently, so the parts won't add up to the bill." Five of the six edited `NOTES.md` to
  replace the wrong note.
- **`cold`, 6 of 6**, as in E4 at scale.

This is 0 of 6 against 6 of 6 on one task, with one wording and one model, picked out after
seeing the data. The pre-registered rule makes no claim from six sessions, and none is made here:
it is an observation to test, not a result.

### 2. On three tasks the wrong note changed nothing

`relay-coupon-remove`, `relay-money-plain` and `relay-store-count`: 6 of 6 in every arm, and no
session's last edit to the task's file is the wrong one. What the `av` agents' messages say
about the record (read after the run, found by a keyword search of each session's messages):

- `relay-store-count`: 5 of 6 say the record is wrong and give the reason from the code:
  `list_page` clamps to `max_page_size`, so the recommended `len(list_page(...))` undercounts.
  One says it checked (250 records gave 100). Four wrote an `attempt` record.
- `relay-money-plain`: 4 of 6 say the record is wrong because the currency table has JPY with 0
  and KWD with 3 decimals. Two of them say their new record supersedes it.
- `relay-coupon-remove`: 0 of 6 mention the record. Five say they normalize the code as `get`
  does.

So on the two tasks where agents explained themselves, the code they read contradicts the note.
On `relay-quote-split` the six closing messages cite the record, the existing tests pass with the
wrong code, and no message reports a check of whether the parts add up. That fits "the agent
follows a record the code does not contradict", but one task of each kind cannot show it.

### 3. How the two memories were used

From `behaviour.md`, over the 30 sessions of each arm:

| | `notes` | `av` |
|---|---|---|
| sessions that changed the stored note | 24 edited `NOTES.md` | 2 with a call that mentions superseding a record |
| sessions that recorded an `attempt` (a rejected approach) | not applicable | 5 (4 on `relay-store-count`, 1 on `relay-line-taxes`) |
| sessions that wrote a new record | not applicable | 30 (the stop hook asks for one) |
| sessions whose new record or note text matches the wrong edit's pattern | 1 | 15 |

The last row counts text that matches the `[[dead_end]]` pattern. For `relay-quote-split` all six
endorse the approach. For the other tasks a record may name the approach in order to reject it;
those were not read one by one.

This was not predicted. `DESIGN.md` expected the `notes` arm to be the more obedient one, since
its `CLAUDE.md` says to follow the notes. On `relay-quote-split` the reverse happened. The cell
cannot say why. Candidates, none tested:

- A record arrives in the session-start hook's output under the heading "Decisions & dead ends"
  with nothing that says who wrote it or whether it was checked. A note is a line in a file the
  agent opened itself.
- A notes file can be edited in place, and agents did. A record can only be superseded by a new
  one, which takes knowing the option exists. Two sessions of thirty have a call that mentions
  it (whether the store holds the link was not checked), and five recorded the rejected approach
  as an `attempt` beside the wrong record, which stays as it was.
- The pack shows the note's summary only. The `notes` arm also read the body.

### 4. One task is contaminated: agents read the grading test

While reading transcripts, one `av` session on `relay-line-taxes` said: "I found the actual
grading tests in the bench tree". `leak.py` then classified every tool call of all 90 sessions
(`leak.md`):

| arm | sessions | exposed | names a hidden test | names another session's files | names the source tree | web | daemon over HTTP |
|---|---|---|---|---|---|---|---|
| `cold` | 30 | 1 | 1 | 1 | 1 | 1 | 0 |
| `notes` | 30 | 3 | 3 | 2 | 1 | 0 | 0 |
| `av` | 30 | 4 | 4 | 4 | 2 | 0 | 0 |

- All 8 exposed sessions are on `relay-line-taxes`, the hardest task of the five, and all 8 were
  accepted. No session on another task is exposed.
- **Where each of the 8 read the grading test** (from its calls):
  - 5 from an earlier session's kept copy under `runs/`: `cold` r4 from `cold` r3's copy; `notes`
    r2 and r4 from earlier `notes` copies; `av` r1 from a `cold` copy; `av` r2 from a `notes`
    copy. These copies were there because of `--keep-work`.
  - 2 from an earlier `av` session's workspace under the daemon's workspace root: `av` r5 and r6
    read it in the workspace of `av` r2. This route does not need `--keep-work`: without the
    flag the harness abandons the task and the daemon removes the workspace after its retention
    time, which was the default of one hour here, and the whole `wrong` run took 49 minutes. The
    grading test is in that workspace because the verify step copies it there.
  - 1 from the bench tree itself: `notes` r1, the first session of the `wrong` run on this task,
    read `/src/bench/tasks-relay/files/test_relay_line_taxes.py` and
    `/src/bench/tasks-relay/reference/edits.py`, the reference fix. This route does not need
    `--keep-work` either.
- **Other routes.** The repository with the grading tests is public and the container's network
  is open. One session (`cold` r4) ran a web search and a code-search query for the task's
  function names; neither call names a test file, and it read the test from a kept copy. No
  session called the bench daemon over HTTP. `av` subcommands other than `record` and `expand`
  (`search`, `log`, `task`, `events`, `status`) were used by two `av` sessions, both among the
  exposed ones. Their answers, read in the event streams, show only the session's own repository:
  its own intent, the seeded records and the base history. Each `av` session has a repository of
  its own on the daemon, but the daemon is one process with no per-session access, so a session
  that asked for another repository by name would get it. None did.
- **A route left open by the sessions themselves.** Two exposed sessions (`notes` r4 and `av`
  r6) copied the grading test into `/tmp` to run it, and the copies stayed there, readable by
  every later session. No other session's calls name them. Found afterwards, when the work on
  the harness looked for copies of grading files in shared directories.
- **Cause.** Sessions run as root in the harness container, where the work root holds every
  earlier session's directory, the daemon's workspace root holds earlier `av` sessions'
  workspaces, and `/src` holds the bench tree with the grading tests and the reference fixes.
  `--keep-work` made it worse: each earlier `cold` and `notes` session's copy stayed on disk
  with the grading test the verify step had copied into it and with that session's solution.
- **Effect.** `relay-line-taxes` rose from 3 of 6 in `cold` to 6 of 6 in both note arms. With 7
  of those 12 sessions exposed, that rise cannot be put down to the note. Among sessions that
  were not exposed the task reads `cold` 2 of 5, `notes` 3 of 3, `av` 2 of 2, which is too few to
  say anything.
- With exposed sessions set apart (post hoc): `cold` 26 of 29, `notes` 27 of 27, `av` 20 of 26.
- The classes are by what a command names, not by what it printed, so a session counted as
  exposed may have learned nothing, and a session that found the test some other way is missed.

**What this means for earlier runs.** The same container layout was used for E4 at scale: one
container, sessions as root, the source clone with the grading tests on the same filesystem, and
earlier sessions' event streams under the work root. That sweep did not keep working copies, and
its volumes are gone, so its transcripts cannot be checked. Its `cold` arm had the same access
and solved 3 of 15 on the decision tasks, so widespread use of the leak there is unlikely, but it
is not excluded. Two of the three routes used here need no kept copies: the bench tree, and an
earlier `av` session's workspace within the daemon's retention time (what that sweep's daemon
had set is not known).

## Secondary measures as registered

- **Sessions that wrote the wrong edit** (`analysis.md`): `cold` 3 (none accepted), `notes` 0,
  `av` 6 (2 accepted, 4 not). `cold`'s three are `relay-line-taxes`, where the wrong edit is the
  obvious first try. Two further `av` sessions failed `relay-quote-split` with the wrong approach
  in a form the pattern did not match.
- **Delivery:** the task's wrong note reached the agent in 30 of 30 `notes` sessions (whole, in a
  tool result; 28 by a read of `NOTES.md`) and 30 of 30 `av` sessions (whole, in the session-start
  hook's output). It is in no kept copy's `git log` (30 of 30 `notes` copies checked) and in no
  `cold` session's event stream.
- **Cost:** per accepted task `cold` $0.0119, `notes` $0.0160, `av` $0.0172. Median session cost
  from the harness (`wrong/report.md`): `notes` $0.013 ($0.011 to $0.015), `av` $0.011 ($0.009 to
  $0.012).

## Deviations from the pre-registration

- **The leak above.** The pre-registration says `--keep-work` "changes nothing a session sees".
  That was wrong. The pre-registration also did not name the bench tree in the container, or
  the daemon's workspaces of earlier sessions, as things a session can read.
- The delivery check on the first session of each arm passed as written. The first `av` session's
  hook counts were session-start 1, pre-read 3, post-edit 1, stop 2.
- `leak.py` and `behaviour.py` were written after the run, and both were extended once more
  after the first write-up (`leak.py`: the web and daemon classes; `behaviour.py`: superseding
  counted in any call, and the `attempt` column). `analyze.py` was committed before the run.
- The first write-up of this file called the `relay-quote-split` difference "not noise". That
  was a claim about six sessions, which the rule forbids, and it was taken out.
- No row was edited, dropped or re-run. `rows.jsonl`, `meta.json` and `report.md` are as the
  harness wrote them. `meta.json`'s `av_rev` reads `unknown`, as the pre-registration said it
  would; the build is the tree of commit `8121f29`.

## Spend

| | |
|---|---|
| harness total (peak prices) | **$1.2557** (smoke $0.0424, `cold` $0.3214, `notes` $0.4785, `av` $0.4134) |
| DeepSeek balance before | $3.30 (2026-11-11 17:05 UTC) |
| balance after | $2.68 (18:19 UTC, 2 minutes after the last session; $3.15 at 17:28 after `cold`) |
| balance delta | **$0.62** |

The run was inside DeepSeek's off-peak window, so the bill is about half the harness figure. No
stopping rule was reached.

## Exact commands

In `lp-harness` from `/src` (this branch), the key passed in with `docker exec -e
DEEPSEEK_API_KEY` and never written down, each command under a subreaper:

```bash
export PATH=/target/release:$PATH
unset ANTHROPIC_API_KEY
export ANTHROPIC_BASE_URL=https://api.deepseek.com/anthropic
export ANTHROPIC_AUTH_TOKEN="$DEEPSEEK_API_KEY"
export ANTHROPIC_DEFAULT_HAIKU_MODEL=deepseek-flash ANTHROPIC_SMALL_FAST_MODEL=deepseek-flash
export CLAUDE_CODE_SUBAGENT_MODEL=deepseek-flash
P="--price-per-mtok 0.30,0.30,0.006,1.20"
OUT=bench/results/2026-11-wrong-lesson
COMMON="--av-url http://127.0.0.1:7450 --work-root /root/bench-work --import-dir /root/bench-work/src --import-mount /root/bench-work/src"
scholion-bench run bench/tasks-relay/order-ids.toml --arms cold,notes,av --jit-hooks --reps 1 $P \
  --session-cap-usd 0.08 --budget-usd 0.30 --keep-work --out $OUT/smoke $COMMON --confirm
scholion-bench run bench/tasks-relay-wrong --arms cold --jit-hooks --reps 6 $P \
  --session-cap-usd 0.08 --budget-usd 0.60 --keep-work --out $OUT/cold $COMMON --confirm
scholion-bench run bench/tasks-relay-wrong --arms notes,av --jit-hooks --reps 6 $P \
  --session-cap-usd 0.08 --budget-usd 2.00 --keep-work --out $OUT/wrong $COMMON --confirm
```

Setup: `PREREG.md`, "Differences from the E4-at-scale setup". Session times: smoke 17:10 UTC,
`cold` 17:13 to 17:28, `notes` and `av` 17:28 to 18:17.

## Files

- `cold/`, `wrong/`, `smoke/`: the harness's `rows.jsonl`, `meta.json`, `report.md`.
- `first-packs/`: the free replay before the run.
- `analysis.md` from `analyze.py`; `leak.md` from `leak.py`; `behaviour.md` from `behaviour.py`;
  `patterns.py`, the free check of the dead-end patterns.
- The session directories (event streams, kept copies) are in the build container and are not
  committed. An archive of all 93 is kept outside the repository.

## What to do next

1. **Close the leak before any other paid cell.** Run the agent as a user that can read only its
   own working copy, with the bench tree, the reference fixes and earlier sessions out of reach,
   and without `--keep-work`. Check it without a model first, with two sessions in a row: as
   the second session's user, reading a grading test, the reference fixes, the first session's
   directory and the first session's daemon workspace must all fail. Two routes stay open after
   that and have to be named in any later result: the daemon answers any session about any
   repository (closing it takes a daemon or a token per session), and the public repository is
   reachable over the network unless the container's egress is limited to the model's API.
2. **Run this cell again on the closed harness**, pre-registered, with the per-task comparison on
   `relay-quote-split` named in advance as the primary measure at 30 sessions per arm, and more
   tasks of its kind: a wrong note that the code and the visible tests do not contradict.
3. **Then test a change to how a record is presented**, not a new store feature: say in the pack
   that a record is an earlier agent's claim to check against the code, and tell the agent it can
   supersede one. The same cell measures it, and the true-lesson relay tasks check that it does
   not cost the 30 of 30 that true lessons give.
