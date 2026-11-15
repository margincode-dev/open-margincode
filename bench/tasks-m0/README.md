# M0: the tasks that measure correction (Plan 16, Phase 0)

The three tasks M0 runs on, and that the measurements after it (M1 Part 2, M2 stage B) copy. Each
one has a confident wrong note that nothing in the repository contradicts. The pre-registration
is [`../results/2026-11-14-m0/PREREG.md`](../results/2026-11-14-m0/PREREG.md).

| task | spec | where it comes from |
|---|---|---|
| `relay-quote-split` | [`quote-split.toml`](quote-split.toml) | [`../tasks-relay-wrong/quote-split.toml`](../tasks-relay-wrong/quote-split.toml), with `relay_pool` set to `notes-silent.toml` |
| `relay-bus-resubscribe` | [`bus-resubscribe.toml`](bus-resubscribe.toml) | [`../tasks-relay-silent`](../tasks-relay-silent), byte for byte |
| `relay-tax-with-rate` | [`tax-with-rate.toml`](tax-with-rate.toml) | [`../tasks-relay-silent`](../tasks-relay-silent), byte for byte |

No new task was written for M0. Two of the three silent tasks from the second wrong-lesson run
already have the property M0 needs, and none of them has had a paid session yet (that run's
secondary cell never ran). The third, `relay-order-merge`, is left out: its goal states the
behaviour the hidden test checks ("a product that is on both ends up with the two quantities
added together"), so the task text contradicts its note (`../results/2026-11-wrong-lesson-2/PREREG.md`,
"Design").

All three run under one pool, [`../tasks-relay/pool/notes-silent.toml`](../tasks-relay/pool/notes-silent.toml).
It holds thirteen notes, eight of them wrong. `relay-quote-split`'s note is the same text as in
`notes-wrong.toml`. The pool has three more notes than in the stage-2 run of that task, so its
numbers here are not stage 2's. What the pack shows for it is checked again by the free replay
(the PREREG, "Free checks at the start of a run").

## What is proven, without a model

```bash
bench/tallyho/make-repo.sh                                          # ~/bench-repos/tallyho
~/bench-work/venv/bin/python bench/tasks-relay/reference/check.py   # all 13 pairs ok (3 of them silent)
python3 bench/tasks-relay-silent/check_set.py ~/bench-repos/tallyho # the silent pool and the two specs
python3 bench/tasks-m0/check_set.py ~/bench-repos/tallyho ~/bench-work/venv/bin/python   # M0 set ok
margincode-bench check bench/tasks-m0
```

- `reference/check.py`: for each pair, the hidden test passes on the reference fix and fails on
  the edit the wrong note recommends (`dead` in `reference/edits.py`).
- `check_set.py` here: the two silent specs are the silent set's files unchanged. `quote-split.toml`
  differs from the wrong-lesson spec only in `relay_pool`. In the pool, each task's own note is a
  `no_commit` decision anchored in the file the task changes. It also runs `check.py`'s silent
  proof on all three pairs, `relay-quote-split` included, which `check.py` does not run for it: on
  the base tree the repo's own tests pass and the hidden test fails; with the fix both pass; with
  the wrong note's edit the own tests still pass and only the hidden test fails.
- What was searched in tallyho for the two silent notes, and found, is in
  [`../tasks-relay-silent/README.md`](../tasks-relay-silent/README.md). For `relay-quote-split` no
  search was written down. What is there: `Money.allocate`'s docstring says its parts "sum exactly
  to this one", and `test_allocate_sums_exactly` tests that. The note's claims about the code are
  true: `allocate` hands out parts that differ by a cent. Nothing says a split must add up to the
  total, and the goal does not say it either. In the first wrong-lesson run the six `av` sessions
  cited the record and none reported a check against the code
  ([`../results/2026-11-wrong-lesson/README.md`](../results/2026-11-wrong-lesson/README.md)). This
  is the weakest of the three on silence: a reader who connects the docstring to the goal can see
  through the note.

What is not proven is listed in `../tasks-relay-silent/README.md`, "What is not proven", and
applies to all three tasks. In short: that `cold` solves them (only paid sessions can show it),
that the note reaches the agent (the free replay), and that a careful reader cannot see through
the note.
