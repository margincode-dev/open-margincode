# E2, 2026-10-30: merge fuzzing and pack determinism

Plan 6 E2, the two free parts that need no public repos: merge correctness under random
concurrent edits, and whether the same state gives the same pack. No agent, no money.

## Merge correctness, fuzzed

`crates/scholion-vcs/tests/merge_fuzz.rs` (proptest). Each case builds a git repo of one to three
files, starts two to five tasks (most from the same base, some after earlier merges), gives each
random operations (replace a line, delete a file, add its own file, add a file another task may
also add), merges them in order, and sometimes reverts an earlier merge. After every merge and
revert it checks:

- trunk equals a line model of the expected result, file for file (no committed line lost, none
  invented);
- a clean result is one new commit whose first parent is the previous trunk; a conflict leaves
  trunk where it was;
- jj reports a conflict exactly when the model does, on the same paths;
- `git merge-tree` of the same three commits agrees: same conflict paths, or the same tree object.

Edits by different tasks never touch adjacent lines. git conflicts on adjacent changes and jj may
not, and that difference is not a correctness bug.

| run | cases | clean results | conflicts | reverts | failures |
|---|---|---|---|---|---|
| CI (fixed seed, debug) | 24 | 79 | 11 | 14 | 0 |
| seeds 1–3, 300 each (before reverts) | 900 | 2,730 | 435 | – | 0 |
| seeds 11–12, 400 each | 800 | 2,974 | 401 | 547 | 0 |

The test can fail. A model that keeps the trunk side when the task side changed was caught and
shrunk to two tasks, one of them empty. Swapping a revert's sides was also caught. A first version
of the generator produced no conflicts at all in 24 cases, because tasks mostly started from the
latest trunk. The test now asserts that a run sees clean results, conflicts and reverts.

Long run: `PROPTEST_CASES=2000 PROPTEST_RNG_SEED=<n> cargo test -p scholion-vcs --release --test merge_fuzz`.

Decision rule (Plan 6): any lost-work or false-clean merge blocks a release. None was found.

## Pack determinism

`crates/scholion-pack/tests/determinism.rs`. The fixture is built to tie: twelve near-identical
handlers and two dozen records with equal scores, under a 2,000-token budget that cuts some of
them. Each pack gets a fresh database, so write order, symbol ids and record ids are the only
things that can differ.

The README claimed "the same state gives the same pack". Three things broke that:

1. **Lexical search cut ties by symbol id.** `search_code_lexical` ordered by score, then
   `symbol_id`, then applied `LIMIT`. Symbol ids follow the order files were indexed, so
   handlers 00–07 got the text-match boost in one run and 04–11 in the other. A fresh import and
   an incremental index of the same tree could pick different code. Ties now go by path and line.
2. **The budget cut ties by item id.** For records the id is a random UUID, so equal-scored
   records were ordered at random. Recency counts in whole seconds, and an agent writes its
   records at submit within one second, so this tie is the usual case for one task's records.
   Equal scores now put the newer record first, and code by path and line. The cold-start
   hit expansion uses the same order.
3. **Not bugs, but they bit the test:** the real tokenizer counts a random hex id as 3 to 6
   tokens, so ids decide what fits in a tight budget. Ages also round to whole seconds, which lets
   the clock flip a near tie. The test therefore pins record ids and ages from their content. Both
   are part of the state, not noise to hide.

Each fix has a test that fails without it: write order with ids blanked, and a burst of records
that must list newest first. Neither golden file changed, and the E1 CI gate passes unchanged.

E1 on the 42 public tasks, `main` against this branch on the same imports (`--reuse-run`), both
without the semantic layer: 46 of 336 pack rows changed, and the means did not move at three
decimals except `symbols_sig` at 4k for `pack` (0.732 → 0.735). The fixes change which of
equally scored items wins, not how well the packer retrieves.

## Not done yet in E2

- Scale and latency on public repos of about 1k, 10k and 50k files, and the 100-agent load
  generator.
- Token estimate accuracy per model family (the `calibrate` path exists).
