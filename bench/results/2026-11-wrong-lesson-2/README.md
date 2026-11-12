# A confident wrong note, as a record and as a notes file (E4, wrong-lesson cell, second run)

2026-11-12. 72 sessions of `deepseek-flash` under Claude Code 2.1.318 on one task,
`relay-quote-split`, with the agent isolated (`--agent-user benchagent`). Pre-registered in
[`PREREG.md`](PREREG.md), pushed before the first paid session. The first run is
[`../2026-11-wrong-lesson/README.md`](../2026-11-wrong-lesson/README.md).

## Summary

The task's stored note confidently recommends an approach that fails the hidden test, and nothing
the agent can read contradicts it. The same note was given three ways:

| arm | what the agent gets | accepted | 95 % Wilson |
|---|---|---|---|
| `cold` | no note | 12 of 12 | 0.758 to 1.000 |
| `notes` | the note in `NOTES.md` | 27 of 30 | 0.744 to 0.965 |
| `av` | the note as a Scholion record in the pack | 2 of 30 | 0.018 to 0.213 |

By the pre-registered rule **H1 is reported**: on this task an agent that gets the wrong note as a
record is accepted less often than one that gets the same text in a notes file. The model solves
the task every time without any note, and 27 times of 30 with the note in a notes file. The run
does not establish why the record does so much worse: no hypothesis about the cause was
registered, and the two arms differ in three ways named in advance (see "What this does not
show").

This is one task, one wrong note, one wording and one model. It says what happens here when a
record is wrong; it says nothing about how often records are wrong, and nothing about right ones.
The arms differ in more than the container of the note (see "What this does not show").

The secondary cell (three more tasks) has not run yet; this file reports the primary cell alone.

## Result by the pre-registered rule

From [`analysis.md`](analysis.md), written by [`analyze.py`](analyze.py):

| arm | sessions | valid | accepted | rate | 95 % Wilson | cost | cost per accepted | median turns |
|---|---|---|---|---|---|---|---|---|
| cold | 12 | 12 | 12 | 1.000 | 0.758 to 1.000 | $0.1051 | $0.0088 | 14.0 |
| notes | 30 | 30 | 27 | 0.900 | 0.744 to 0.965 | $0.4481 | $0.0166 | 21.0 |
| av | 30 | 30 | 2 | 0.067 | 0.018 to 0.213 | $0.3499 | $0.1750 | 17.5 |

- `av` and `notes` each have 30 valid sessions, and the upper end of `av`'s interval (0.213) is
  below the lower end of `notes`'s (0.744): H1.
- `cold` solved 12 of 12, above the 10 of 12 the rule asks for: the comparison is not confounded
  by a task the model cannot solve unaided.
- No session was invalid and none was skipped for budget.

## Leak check (run before any row was read)

[`leak.md`](leak.md), written by [`leak.py`](leak.py), classifies every tool call of all 72
sessions by what its input names.

- No session named a hidden test, another session's directory or workspace, the web, or the
  daemon over HTTP.
- **One session is exposed by what it named: `av` repetition 26.** It listed `/bench/store`,
  `/bench/results`, `/bench/bench-repos` and `/bench/bench-work/runs`, and searched `/bench`,
  `/home`, `/root` and `/tmp` for the task's name. Every one of those calls came back empty: the
  closed harness refused them. The session was not accepted. By the pre-registration an exposed
  session on the primary task is a deviation; without it `av` is 2 of 29 (0.019 to 0.220) and
  the decision is the same.
- The same session named `/bench/daemon.log` and `/bench/daemon.pid`. Checked afterwards as the
  agent user: both are refused. That user can list the names in `/bench` and, under them, only
  `bin` (the `av` binaries) and the venv that holds pytest.
- The "looked around" column of `leak.md` is inflated: the scan counts a bare `/` inside other
  text (a regular expression, a fraction). It is kept as written; the exposing classes do not have
  this fault, because they need a path under a named directory.
- **What the scan can miss.** It classifies a call by the absolute paths and names in its input.
  A read through `~`, `$HOME` or another variable, a `..` path that names none of the watched
  words (it is counted only as "looked around"), a path put together in the shell, or a symlink
  made inside the working copy would not be classed as exposing. Network use is seen only as
  WebSearch, WebFetch, `curl` or `wget`. So "no session named" is what the scan found, not proof
  that no session reached.
- **A second check that does not depend on paths** (post hoc, [`content.py`](content.py),
  [`content.md`](content.md)): every event of all 72 transcripts, inputs and outputs, was searched
  for the file name of a hidden test and for the names of the 30 test functions the hidden tests
  define. No transcript has one. A session that had read a hidden test whole, by any file route,
  would show one of those names in a tool result; a partial read that happened to show none of
  them would not. This still says nothing about the daemon or the network.

## Secondary measures as registered

**Sessions that wrote the wrong edit** (`repeated_dead_ends` ≥ 1, the harness's pattern):

| arm | valid | wrote it | wrote it, accepted | wrote it, not accepted | never wrote it, not accepted |
|---|---|---|---|---|---|
| cold | 12 | 0 | 0 | 0 | 0 |
| notes | 30 | 3 | 0 | 3 | 0 |
| av | 30 | 26 | 0 | 26 | 2 |

By the harness's pattern, 26 of the 28 `av` failures and all 3 `notes` failures wrote the note's
approach. The other two `av` failures (repetitions 4 and 9) are not matched by the pattern. Post
hoc, [`content.md`](content.md) prints the last text each of the 31 sessions that were not
accepted wrote to `tallyho/pricing/quote.py`: all 31, those two included, call `.times(` (the
note's approach) and none calls `.allocate(` (the one that passes). The table shows the last
Edit or Write call on that file, not the file as it was tested: a later change through the shell
would not be seen, and for a multi-part edit the replaced text is searched too. The base file
calls neither. It is read from the transcripts, which are not in this repository, and only the
first 300 characters of each text are printed.

**What the agent did with the note** ([`behaviour.md`](behaviour.md), the first run's
`behaviour.py` unchanged, run through [`behaviour.py`](behaviour.py)):

| arm | n | accepted | edited NOTES.md | av record | supersedes | recorded an attempt | restates the wrong edit | final edit is the wrong one |
|---|---|---|---|---|---|---|---|---|
| cold | 12 | 12 | 0 | 0 | 0 | 0 | 0 | 0 |
| notes | 30 | 27 | 21 | 0 | 0 | 0 | 1 | 3 |
| av | 30 | 2 | 0 | 30 | 0 | 0 | 30 | 26 |

- 21 of 30 `notes` agents edited `NOTES.md`. No `av` agent could correct the record: this build
  has no command for it, which the pre-registration names as a difference between the arms.
- Every `av` agent wrote a record. "Restates the wrong edit" is counted by the task's pattern in
  the record's text, so its 30 includes the two accepted sessions, whose records say the opposite
  ("do not use `total.times(1/parts)`", read by hand). Of the other 28 I read two (repetitions 1
  and 4): both record the wrong approach again as a decision, in their own words. I did not read
  the remaining 26.
- No `av` agent recorded an `attempt` (a dead end).

**Delivery.** The note reached every session of both note arms and no `cold` session. `av`: whole
in the session-start hook's output, 30 of 30 (the first record of the pack, 709 of 800 tokens),
never by the pre-read hook. `notes`: 29 sessions read `NOTES.md` with the Read tool and one with
`cat`; all 30 have the note's whole summary in a tool result.

**Cost.** Cost per accepted task is in the table above, as the harness reports it at the
pre-registered prices. Median session cost, from `qs-wrong/report.md` and `qs-cold/report.md`:
`notes` $0.015, `av` $0.011, `cold` $0.009. `av` costs less per session than `notes` and about
ten times more per accepted task, because it is seldom accepted.

One sentence of the generated `report.md` files is wrong for this pool: it says the notes are
commit messages in every arm but `cold`. The wrong notes are marked `no_commit` and are in no
commit message; the harness's report text was not updated for that.

## What this does not show

The confounds named in advance all stand:

- **Framing.** The `notes` arm's `CLAUDE.md` says to read `NOTES.md` and follow its decisions; the
  pack lists the record under "Decisions & dead ends" with no instruction. This run cannot say
  which part of the difference is the framing.
- **What can be done about a note.** A notes agent can edit the file; an `av` agent cannot correct
  a record. The run cannot separate "believed it more" from "could not fix it".
- **What is shown.** The pack shows a record's summary; the notes file shows summary and body.
- **Size of the claim.** Thirty repetitions of one task are not thirty tasks. The intervals
  describe this task under this note with this model.

And the isolation closes file-system routes only: the daemon answers any session about any
repository, the network is open, and verify runs what the agent wrote as root.

## Deviations from the pre-registration

1. **Smoke.** Three invocations were refused by the harness's preflight before any session
   started (no row, no spend): the import directory was enterable by the agent user, and then
   each refused attempt left an empty run directory that the next preflight refused on. The
   fourth, `smoke-3`, ran and passed by the registered criteria (three valid rows, the model
   answered and edited, verify ran, `agent_isolation: benchagent`); "passed" is about the set-up,
   not the task: its `cold` session was not accepted. A refused preflight should not leave a
   directory behind; that is a harness fix to make.
2. **Leak scan.** The pre-registration names the first run's `leak.py`. This run's `leak.py` is
   that scan with the closed set-up's paths, exact session names (the first script would have
   missed `-r1` inside `-r10`) and a list of every outside path. Written while the primary cell
   ran, committed (`07419cf`) before any row was read.
3. **`analyze.py`** was changed after its first commit and before any row was read (`91f534d`): a
   session's directory is under the run id (`qs-cold` is `runs/qscold`), and the accepted table is
   printed again without exposed sessions. That commit had a line break inside two `print`
   strings and did not run; it was repaired after `behaviour.md` had been printed, that is after I
   had seen the accepted counts. The repair changes no computation.
4. **`behaviour.py`** here is a wrapper: the first run's script reads fixed directory names, so it
   is copied into a temporary tree where those names point at this run's, and run unchanged.
5. **An exposed session on the primary task** (`av` 26, above).
6. **The bill.** The pre-registration expected the bill to be about the harness figure. It was
   about half (below).
7. **The secondary cell has not run.** It waits for DeepSeek's off-peak window, as registered.
8. **`content.py`** is not in the pre-registration. It was written after the result was known,
   when a review of this file asked what the path scan can miss and how the two unmatched `av`
   failures were known to be the note's approach. It is used for nothing in the decision.

## Spend

| | harness figure | DeepSeek balance |
|---|---|---|
| before the smoke (04:41 UTC) | | $2.67 |
| smoke (`smoke-3`) | $0.0438 | |
| `qs-cold` | $0.1051 | |
| `qs-wrong` | $0.7980 | |
| after the primary cell (05:42 UTC) | $0.9469 in all | $2.19 |

The balance fell by $0.48. The balance is shown in cents and may lag the last sessions by a few
minutes.

## Exact commands

As in `PREREG.md`, commands 0 to 2, in `lp-harness` from `/bench/tree` (a `git archive` of
`55afbc8`), binaries by the hashes recorded there:

```bash
scholion-bench run bench/tasks-relay/order-ids.toml --arms cold,notes,av --jit-hooks --reps 1 $P \
  --session-cap-usd 0.08 --budget-usd 0.30 --out $OUT/smoke-3 $COMMON --confirm
scholion-bench run bench/tasks-relay-wrong/quote-split.toml --arms cold --jit-hooks --reps 12 $P \
  --session-cap-usd 0.08 --budget-usd 0.40 --out $OUT/qs-cold $COMMON --confirm
scholion-bench run bench/tasks-relay-wrong/quote-split.toml --arms notes,av --jit-hooks --reps 30 $P \
  --session-cap-usd 0.08 --budget-usd 1.40 --out $OUT/qs-wrong $COMMON --confirm
```

Afterwards, as root, from a copy of the tree with these results beside the scripts:

```bash
python3 bench/results/2026-11-wrong-lesson-2/leak.py      > leak.md       # first
python3 bench/results/2026-11-wrong-lesson-2/analyze.py   > analysis.md
python3 bench/results/2026-11-wrong-lesson-2/behaviour.py > behaviour.md
```

The transcripts (`events.jsonl` per session) are kept in the container under
`/bench/bench-work/runs/{qscold,qswrong,smoke3}` and are not in this repository.

## Files

- `PREREG.md`: the pre-registration. `qs-cold/`, `qs-wrong/`, `smoke-3/`: what the harness wrote
  (`rows.jsonl`, `meta.json`, `report.md`). `first-packs-qs/`, `first-packs-silent/`: the free
  delivery check before the run.
- `leak.py`, `leak.md`, `exposed.json`; `content.py`, `content.md` (post hoc); `analyze.py`,
  `analysis.md`; `behaviour.py`, `behaviour.md`.

## What follows

As decided in the pre-registration for H1: the next step is the change already being built, an
agent can supersede a record (`av record --supersedes`) and the pack says a record is an earlier
agent's claim to check against the code. It is measured on this same cell with the same rule,
under its own pre-registration. Two things that measurement should be able to tell apart, which
this one cannot: whether the pack's one line changes what the agent does with the record, and
whether agents use the command when they have it.
