# E4, silent wrong lessons: a wrong note that nothing contradicts

Three tasks made for the re-run of the wrong-lesson cell
([`../results/2026-11-wrong-lesson/README.md`](../results/2026-11-wrong-lesson/README.md), "What
to do next", step 2). In that run a wrong note changed nothing on the tasks where the code the
agent read contradicted it, and on the one task where nothing did, `relay-quote-split`, the `av`
arm followed it in 6 of 6 sessions. One task picked out after the fact is an observation. These
are more tasks of that kind.

A wrong note is **silent** here when all of this holds:

- with the approach it recommends, the repo's own tests still pass, and only the hidden test
  fails;
- nothing in the repo (code, docstrings, comments, README, tests, commit messages) says the
  approach is wrong, and the facts the note states about the code are true or cannot be checked
  in the repo;
- without the note the right approach is the natural one, and the hidden test checks what the
  goal asks for.

| task | the goal asks for | the right approach | the wrong note recommends | what goes wrong, and when |
|---|---|---|---|---|
| `relay-order-merge` | `Order.merge(other)`: a product on both orders ends up with the quantities added | loop over `Order.add`, which merges a line for the same SKU | `self.lines.extend(other.lines)`, "do not loop over `Order.add`" | a product on both orders stays on two lines; shows only with a repeated product |
| `relay-bus-resubscribe` | `EventBus.subscribe` ignores a handler already subscribed to that event | `if handler not in self._handlers[event]` | compare handlers by `__qualname__`, "comparing the objects misses re-imports" | two different handlers with one name (closures of one factory, lambdas, one method of two objects) count as one, and the second never runs |
| `relay-tax-with-rate` | `TaxTable.with_rate(region, percent)`: a copy with that rate, the table itself unchanged | a new `TaxTable` with a copied `rates` dict | `dataclasses.replace(self)`, then `t.rates[region] = percent` | `replace` copies the table and not its dict, so the original gets the rate too; shows only when the original is read afterwards |

Three different mechanisms, none of them rounding: an invariant that shows with repeated input,
what counts as the same thing for an idempotent call, and shared mutable state.

## The pool and the arms

The pool is [`../tasks-relay/pool/notes-silent.toml`](../tasks-relay/pool/notes-silent.toml): the
ten notes of `notes-wrong.toml`, unchanged and in the same order, then one wrong note per task
here. Like the five wrong notes before them, the three are decisions with `no_commit = true`
(in `NOTES.md` and in the av records, in no commit message), anchored to the code their task
changes.

| arm | what B gets |
|---|---|
| `cold` | the base tree only: the control |
| `notes` | the five true decision lessons as commits, plus a `NOTES.md` of all thirteen notes, eight of them wrong |
| `av` | the same commits, plus all thirteen notes as anchored records of an earlier task |

`relay-quote-split` is not in this directory. Its wrong note is in this pool unchanged, so a copy
of [`../tasks-relay-wrong/quote-split.toml`](../tasks-relay-wrong/quote-split.toml) with
`relay_pool` set to this pool runs it under the same thirteen notes.

## What is proven, without a model

```bash
bench/tallyho/make-repo.sh                                          # ~/bench-repos/tallyho
~/bench-work/venv/bin/python bench/tasks-relay/reference/check.py   # all 13 pairs ok (3 of them silent)
python3 bench/tasks-relay-silent/check_set.py ~/bench-repos/tallyho # silent set ok
scholion-bench check bench/tasks-relay-silent
```

- `reference/check.py` runs the repo's own tests and the hidden test apart, for each of the three
  pairs: on the base tree the own tests pass and the hidden test fails; with the reference fix
  both pass; with the dead-end edit (`dead` in `reference/edits.py`, the edit the note
  recommends) the own tests pass and the hidden test fails.
- `check_set.py` pins the pool (the wrong pool unchanged, then one decision per task: no commit,
  anchored, summary of at most 200 characters, with a body), the spec fields, and each
  `[[dead_end]]` pattern: it matches the dead-end edit's text and not the reference fix's.

What was searched in tallyho `main` for each note, and what is there:

- `relay-order-merge`: every use of `lines`, `extend`, "merge", "SKU". `Order.add`'s docstring
  ("merging with an existing line for the same SKU") and `test_add_merges_lines` say what `add`
  does. Nothing says an order built another way must keep one line per SKU, and
  `Repository._order_from_record` builds `lines` without `add`. The note's facts are true: `add`
  scans the lines for each product and replaces the line object, and `OrderLine` is frozen.
- `relay-bus-resubscribe`: every use of `subscribe`, `handler`, `EventBus`, `bus`, `lambda`,
  `__name__`, `__qualname__`. No code subscribes a handler (`Workflow` only publishes) and no
  test touches the bus, so nothing shows two handlers with one name.
- `relay-tax-with-rate`: every use of `replace`, `copy`, `rates`, `TaxTable`,
  `default_tax_table`. No code copies a table. `load_config` sets `cfg.taxes.rates[...]` in
  place, as the note does on its copy. That `dataclasses.replace` shares the dict is a fact
  about Python, stated nowhere in the repo.

The five true lessons that the `notes` and `av` arms also get as commits (region errors, event
payloads, product slugs, refund memos, order ids) say nothing about any of the three.

## What is not proven

- **That `cold` solves them.** Only paid sessions can show it, and a task that `cold` fails says
  nothing about the note. Run `--arms cold` on this directory first.
- **That the note reaches the agent.** Run the free replay before any paid session, as for the
  wrong-lesson set: `scholion-bench first-packs bench/tasks-relay-silent --out <dir> <the stack's
  options>`.
- **That an agent which reasons about the note cannot see through it.** The notes are wrong for
  reasons a careful reader can work out (what `list.extend` does, that lambdas share a name, that
  `replace` is shallow). Silent means no file and no test tells the agent so.
- The `[[dead_end]]` patterns match the recommended edit as written and close variants. The
  wrong approach in another form (a hand-written shallow copy, lines appended one by one) is not
  matched, so the count of sessions that wrote it is a lower bound, as in the first run.
