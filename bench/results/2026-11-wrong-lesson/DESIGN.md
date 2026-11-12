# Design: does a confident wrong note cost solved tasks?

Written 2026-11-11, before the harness for it was run and before any session. The
pre-registration (`PREREG.md`, written after the free replay and before the first paid session)
fixes the measures and the decision rule; this file says why the cell looks the way it does.

## Why this cell

Every comparison so far ends at a ceiling. In E4 at scale (`../2026-11-e4-scale/README.md`) the
`notes` and `av` arms both solved 30 of 30 at every noise level, so memory could only be shown to
save money, not to change an outcome.

There is one place in a memory system where an outcome can move the other way. A note recorded
with confidence and never checked is trusted by every later session. Anthropic's write-up on
agent memory (<https://www.anthropic.com/research/components-of-agent-memory>) describes exactly
that failure, a wrong landmark note that sent a run in circles until an offline pass corrected
it. Scholion has no defence against it today:

- a record is superseded only when an agent writes a newer one that says so;
- a record is marked changed only when its anchored code moves, and the pre-read hook does not
  print that mark;
- records of a task that was abandoned or reverted are served like any other.

So before building a correction feature, measure whether there is anything to correct: **does a
wrong note lower the rate of solved tasks, and does it do so equally for a flat notes file and
for anchored records?**

## The tasks

The relay set has ten tasks: five whose lesson is a dead end ("tried X, it breaks Y") and five
whose lesson is a decision. In the cold arm of E4 at scale (`../2026-11-e4-scale/cold/report.md`)
the five dead-end tasks were solved 15 of 15 without any lesson, and the five decision tasks 3 of
15.

- On the **dead-end tasks** the model gets it right unaided. A wrong note can only hurt, and any
  fall below the cold rate is the note's doing. These are the five tasks of this cell.
- On the **decision tasks** the model fails unaided, so a wrong note has nowhere to push the rate
  down to. They are left out.

## The wrong notes

For each of the five tasks, the pool's lesson is replaced by a note that recommends the approach
the lesson warns about, stated as a `decision`, with a body that claims it was checked. Each one
endorses the `dead` edit of `bench/tasks-relay/reference/edits.py`, which `reference/check.py`
proves fails the task's hidden test. The texts are in
`bench/tasks-relay/pool/notes-wrong.toml`; a test pins that nothing else in the pool differs.

Choices, and what each one rules out:

- **Same anchor and same place in the pool** as the lesson it replaces: delivery is the same as
  in E4, so a difference from E4 is the text's doing.
- **Byte-identical text in both arms**: `notes` reads it in `NOTES.md`, `av` gets it as a record.
- **Not in a commit message** (`no_commit = true`). In E4 every lesson is also a commit, and in
  the first relay run 18 of 30 baseline sessions read `git log`. A wrong note there could not be
  corrected by any memory feature, and would hide what the memory channel does.
- **No correct lesson beside it.** That is a second question (which of two notes wins) with its
  own confound, since a rule that hides the older one wins by construction. First find out
  whether a wrong note alone hurts.

## Arms and size

`cold`, `notes`, `av`: 5 tasks × 6 repetitions = 30 sessions per arm, noise 0, the model and
harness settings of the E4 sweep. `cold` is rerun on this build so that all three arms share a
binary. At the sweep's cost per cell ($0.38 to $0.43 at noise 0) the three cells come to about
$1.20.

## What can come out

1. **Nothing moves** (both arms stay at or near the cold rate). The model weighs a note against
   the code and the tests in front of it and follows the code. Then this task set cannot show an
   outcome gain from correcting notes, and the honest claim for a correction feature is cost
   only. That is worth knowing before building one.
2. **Both arms fall together.** A wrong note hurts, and anchoring it to code neither helps nor
   harms. This is the baseline a correction feature is measured against: the same cell with the
   feature on.
3. **The arms differ.** `notes` tells the agent to read the file and follow it (`CLAUDE.md`
   says so); the av pack presents records without that instruction. A difference may come from
   that framing and not from anchoring. The pre-registration names this confound and does not
   claim a winner unless the intervals are disjoint.

## Checks that make a null readable

A null result means something only if the wrong note reached the agent and the agent could have
followed it.

- **Free replay first.** `scholion-bench first-packs bench/tasks-relay-wrong` reports whether the
  wrong note is in the session-start pack or a pre-read answer. Fewer than 4 tasks of 5 shown in
  full, and the av cell is not run as designed.
- **Followed or not, per session.** Each spec's `[[dead_end]]` pattern matches the wrong edit in
  the agent's diff. The report counts sessions that wrote it, including those that replaced it
  later in the session: a note that costs a detour but not the task still costs money.

## What this does not test

- One repository, one model (`deepseek-flash`), five tasks, one wording per wrong note.
- A note that is wrong from the start. A note that was right and went out of date is a different
  case, and Scholion has a partial answer to it (the changed mark).
- Any correction feature. This cell is the baseline for one.
