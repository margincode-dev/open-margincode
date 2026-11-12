# Pre-registration: getting a task's own lesson to its agent

Written 2026-11-10, after the diagnosis in `README.md` and before any change to
`scholion-pack`. Nothing here is paid: every measure is `scholion-bench first-packs` on the bench
stack of `README.md` (no agent, no model calls).

## Question

At 500 noise notes the hooks show a relay task its own lesson in full in 1 task of 10. Can the
packer deliver it without being told which record is the lesson, and without giving up the code
the pack names?

## The change

Two changes, each in its own commit with a replay after it. They are stated here as rules; the
constants are chosen on the design set only.

- **A, pre-read hook (`records_at`).** A file's records are ordered by how well they match the
  goal of the task that is reading the file, and only then by kind and age as today. Matching is
  by words (always available) and by embedding similarity when the semantic layer answers within
  the existing 150 ms; past that, today's order stands.
- **B, the pack.**
  - *Candidates:* a pack asks for more record hits than code hits, and the task's own intent no
    longer takes one of them. Code hits stay as they are.
  - *Ranking:* the few records that match the goal best lead the "Decisions & dead ends"
    section, ahead of records that are only anchored on a symbol the goal names. Records mined
    from git history are not promoted this way.

Neither rule knows about noise, templates, kinds of the lesson or the relay pool.

## Sets

- **Design set** (looked at freely): the 10 tasks of `bench/tasks-relay` at N = 0, 50 and 500,
  noise seed 1.
- **Held-out set:** N = 200 with seed 1, and N = 500 with seeds 2 and 3. Their baselines
  (`before/held-out/`) were run before the change. I do not open them until both changes are
  committed, and after that the packer is not changed again in this pull request. Whatever they
  show is reported.

## Measures

- **Primary:** tasks whose lesson is **shown in full by either hook** at N = 500, seed 1: the
  hook pack at the `summary` or `full` tier, or a pre-read answer for one of the task's
  `relevant_files`. Baseline 1 of 10.
- **Guards:**
  1. N = 0 and N = 50, seed 1: shown in full stays at 9 of 10 or better, and shown at all stays
     10 of 10.
  2. Relevant files named in the hook pack, summed over the 10 tasks, is not lower than the
     baseline at any level. This is what would show records being bought with code.
  3. No hook pack is marked degraded.
  4. `cargo test --workspace` passes, with the retrieval gate's floors unchanged.
  5. `scholion-bench retrieval` on `bench/retrieval/ci` against this stack (semantic layer on),
     before and after: no metric at 300, 450 or 600 tokens falls by more than 0.03. That fixture
     has no records, so this only checks that code ranking did not move.

## Decision rule

- **Fixed, with a semantic layer:** the primary measure is 8 of 10 or better, every guard holds,
  and each held-out N = 500 seed is 8 of 10 or better.
- **Improved, not fixed:** the primary measure rises but one of those conditions fails. The pull
  request then says which, and the change is still offered if every guard holds.
- **No change merged:** a guard fails and cannot be met without a rule that knows about the
  benchmark.

## Limits known in advance

- **No paid run.** At N = 500, 4 of 10 tasks were shown nothing and every session was still
  accepted, so acceptance cannot move, and `av`'s cost per accepted task rose only 14.0 % over
  the whole range. A re-run of the `av` cells would most likely show nothing; the remaining
  balance is kept.
- **One repository, one embedder, template noise.** A pass here says the ordering rule works on
  this fixture. It does not say how it behaves on real records.
- **Without a semantic layer** (the default install), a record becomes a pack candidate only
  through an anchor, and that is not changed here. What word matching alone does is measured in
  the unit tests and stated in the pull request.
- **Determinism.** A pack breaks ties on record ids, which are new on every setup.
  `before/n500-again` repeats N = 500 to show how much one pass varies.
