# M2 stage B, amendment 3: raw against dreamed on held-out tasks

**Committed and pushed before the run's first paid command.** Written as a draft on 2026-11-19
and approved by the owner the same day ("Run as recommended"): the draft's open choices are
settled as it recommended them, and are listed below as fixed decisions ("Decisions"). The draft's
task directory `bench/tasks-heldout-DRAFT/` became `bench/tasks-heldout/` (every path in the specs
is relative, so nothing else changed). `PREREG.md`, `AMENDMENT-1-stage-b.md` and
`AMENDMENT-2-stage-b-revised.md` are not edited. Where this file does not name a change, the run is
stage B as amendment 2 has it: one frozen dream replayed into every session of the dreamed arm,
the two arms, the primary metric (the accepted rate, by the hidden pytest), the models, the daemon
setting, the free checks, the smoke, the claim rule and the money rule. Its sessions do not pool
with stage B's or amendment 2's.

## What had been seen when this was written

Written on 2026-11-19, before any session or freeze on the held-out tasks, and **after** these
results:

- **Stage B** (`README.md`, "M2 stage B: raw against dreamed memory, results, 2026-11-18"): raw
  **14 of 30** (Wilson 0.302 to 0.639), dreamed **21 of 30** (0.521 to 0.833), not shown at this
  size. Old `DREAM.md`.
- **Amendment 2** (`README.md`, last section): raw **47 of 90** (0.420 to 0.622), dreamed **90 of
  90** (0.959 to 1.000), disjoint, so by the claim rule dreaming helps. Per task quote-split 7/30
  and 30/30, bus-resubscribe 10/30 and 30/30, tax-with-rate 30/30 and 30/30. **In-sample:** the
  revised `DREAM.md` (#63) was fitted on these three tasks' stores.
- **The three pilots behind #63** (amendment 2, "What had been seen"): three in-sample
  `dream-freeze` runs on the M0 stores with an `av` built from successive drafts of the text,
  $0.6519, $0.6504 and $0.6030 ($1.905 in all). Each draft was changed after the pilot before it
  had been seen. Pilot 3, the merged text, met its four declared conditions. In each of its stores
  no true lesson got a verdict (judged 0 of 5).
- **The #70 observation** (`../2026-11-19-dream-promote-also-read/README.md`): two review runs
  on a bus-resubscribe store, each with an appended section the freeze does not have. Both
  disputed the same 5 of 8 wrong notes and left out bus-resubscribe's and quote-split's. Pilot 3
  and amendment 2's freeze had bus-resubscribe's note `disputed` in 3 of 3 stores. One text gives
  different dreams; a frozen dream is one draw.
- **What the frozen dream quoted** (amendment 2's `m2-stage-b-revised/dreamed.json`, read for
  this draft). Each of its 19 `disputed` verdicts quotes one line of the file its record is
  anchored to. Bus-resubscribe's note was disputed with the reason "subscribe does not dedupe; it
  appends every handler unconditionally". The note described how the method should dedupe; the
  task asks for that change, and the reason only says that the code does not dedupe yet. No dispute
  quoted a line from another file, and none quoted a test.
- **The person who designed these tasks had read the revised `DREAM.md` and those verdicts
  before writing the notes.** The tasks may therefore be fitted to the text without meaning to
  be: the notes were chosen so that, in principle, a dream can dispute them from the code, and
  that judgement was made by someone who knows how the text reads. This is a confound of this
  set, and it is named again under "What a result would show".
- **Run on the held-out tasks: no model, no dream, no session.** The only thing run was a local
  Python check of the reference edits (below), on a copy of `bench/tallyho/src` with a stand-in
  for the few pytest features the tests use (pytest is not installed where this was written). It
  showed, for each of the five pairs: on the base tree the repo's own tests (21) pass and the
  hidden test fails; with the reference fix both pass; with the edit the wrong note recommends
  the own tests pass and only the hidden test fails. `check_set.py` (below) was also run and
  passed. `margincode-bench check` was not run, nor `reference/check.py` with real pytest.
- **After the owner's approval, before this commit** (no model, no dream, $0): on a preliminary
  tree in `m2b-harness` (amendment 2's Haiku tree with `bench/tasks-heldout` copied in),
  `check_set.py` passed; `reference/check.py` with real pytest (the bench venv, Python 3.13.5,
  pytest 9.2.0) gave all 5 pairs ok, 5 of them silent; `margincode-bench check
  bench/tasks-heldout` gave 5 of 5 `ok`; and a raw `first-packs` on this run's daemon showed each
  task's wrong note in its hook pack (5 of 5), no `--supersedes` line and no origin label. Every
  anchor of the pool (`Ledger.balances` included) is a symbol `margincode-index` stores under that
  exact qualified name (checked on amendment 2's indexed `tallyho`). Nothing in the tasks, the
  pool, the hidden tests or the reference edits was changed after these checks; the only edits
  since the draft are the word "DRAFT" in two comments. The free checks are run again on the run
  tree itself and recorded in the second commit (below).

## Why

Amendment 2 shows that a dream made from a text fitted to three tasks helps on those three. It
does not show that the text helps on tasks it was not fitted on. Only three silent tasks exist:
`bench/tasks-m0` holds quote-split, bus-resubscribe and tax-with-rate. The fourth silent task,
`relay-order-merge` (`bench/tasks-relay-silent/order-merge.toml`), was left out by the owner on
2026-11-14 (`../2026-11-14-m0/PREREG.md`, "Tasks"; `bench/tasks-m0/README.md`). Its goal states
the behaviour its hidden test checks ("a product that is on both ends up with the two quantities
added together"), so the task text contradicts its note. Its note was also in every fitted store,
so it is not held out either. This amendment adds five new tasks under a new pool that the text
has never been run on.

## The fitted kinds, and how the held-out notes differ

The text was fitted on stores that held all eight wrong notes of `notes-silent.toml`, not only
the three tasks' own. Pilot 3's conditions counted 21 of 24 wrong-note verdicts. So the held-out
notes are compared with all eight:

| fitted wrong note | what the note gets wrong | where the evidence is (amendment 2's freeze) |
|---|---|---|
| tax-with-rate | **a wrong claim about an API's behaviour**: `dataclasses.replace(self)` "builds a new table, so the original keeps its rates" (it shares the dict) | the anchored file (the `rates` field line), plus a fact about Python |
| bus-resubscribe | **a rule with a wrong reason**: dedupe by `__qualname__` because "re-imports create new objects", a cause the repository cannot show | the anchored file; disputed as a present-tense fact ("subscribe does not dedupe") |
| quote-split | **a note the code already contradicts**: true fact (`allocate` hands out uneven parts) under wrong advice (do not use it), where the rejected helper's docstring and a test say it is the tool for the job | the anchored file (`allocate`'s docstring) |
| store-count | a method's result (`list_page` returns everything for a large limit) | the anchored file (`list_page`'s docstring) |
| line-taxes | arithmetic (per-line taxes add up to the order's) | the anchored file |
| coupon-remove | normalisation (codes keyed as typed) | the anchored file |
| money-plain | data (every currency has two decimals) | the anchored file (`MINOR_UNITS`) |
| order-merge | true facts under wrong advice (`extend` instead of `add`) | not disputed in any of amendment 2's three stores |

The three in bold are the three kinds the pilots tuned the text on.

| held-out task | what its wrong note gets wrong | evidence a dream can quote | in the anchored file? | `DREAM.md` bullet that would decide it | nearest fitted note |
|---|---|---|---|---|---|
| `relay-order-can-cancel` | **a rule of one operation carried over to another**: "Workflow.cancel takes only draft orders, the same rule Order.add and Order.remove follow" (true of add and remove, false of cancel) | `tallyho/workflow.py`: `OrderStatus.PLACED: {OrderStatus.PAID, OrderStatus.CANCELLED},` | no (anchored in `models.py`) | "A fact is contradicted by code" (words about what a named function does now) | order-merge (a true fact used to draw a false conclusion), but here the conclusion is itself a false fact about other code |
| `relay-delete-order` | **a false error contract under a reason that holds**: "RecordStore.delete raises NotFound for an id that is not there", with the reason that a `get()` first reads the file a second time (true) | `tallyho/store/base.py`: `Remove the record; missing records are ignored.`; also `tallyho/store/memory.py` and `jsonfile.py`: `pop(record_id, None)` | no (anchored in `repository.py`) | "A fact is contradicted by code", against "a reason that holds keeps the advice": the text has not been run on that pairing | **store-count**: the evidence is again a `RecordStore` docstring in `base.py`. The closest to a fitted note |
| `relay-order-comments` | **a false claim that every implementation of an interface behaves alike**, under a naming rule: "Every RecordStore backend takes the collection name as an opaque key" (`JsonFileStore` refuses non-identifiers) | `tallyho/store/jsonfile.py`: `if not collection.isidentifier():` | no (anchored in `repository.py`) | "A record that states both a fact and a rule is judged on each: a contradicted fact is enough" | store-count ("one count() in RecordStore serves all backends") |
| `relay-ledger-revenue` | **a sign convention**: "balances() gives each account as the shop reports it: cash, revenue and tax_payable all come out as positive amounts" | `tallyho/ledger.py`: `out[e.account] += e.debit.minor - e.credit.minor` with `("revenue", z, quote.discounted),` (two lines, one file) | yes, but it takes two lines read together | "Advice holds or fails with its fact" | money-plain, line-taxes (a fact in the anchored file) |
| `relay-coupon-days-left` | **an inclusive/exclusive boundary**: "Coupon.expires is the first day a coupon no longer works" | `tallyho/pricing/coupons.py`: `if self.expires is not None and today > self.expires:` | yes, one line of the anchored class | "A fact is contradicted by code" | coupon-remove (same file, another class), money-plain: **the fitted route**, included as the contrast |

What this set does and does not vary:

- **New:** five kinds of mistake that none of the eight notes has. Three tasks whose evidence is
  in another file than the record's anchor, which the frozen dream never had to reach (the bundle
  hands the anchored code; another file needs `av show` or `av search`). One note whose reason
  holds while its fact is false. One fact that takes two lines to see.
- **Not new:** every one of the five is reached in the end through "a fact is contradicted by
  code". None is a pure rule disputed by a test or a docstring (a clause of the revised text no
  fitted store exercised). None needs a fact about Python, and none gives a reason the repository
  cannot check. A note the dream cannot reach from the code would not test the dream, so this is
  on purpose. It also limits the set: it asks whether the text finds contradicting code in new
  places and forms, not whether it handles a new kind of judgement.
- **No wrong note describes the method its task adds as if it existed.** Each one gives advice
  ("Write Order.can_cancel as …") or a fact about code that is on `main`. Bus-resubscribe's
  dispute above came from a present-tense description of code still to be written.

Considered and left out:
- `Money.percent` with a wrong half-up rounding rule that `Money.of`'s docstring and
  `test_times_rounds_half_even` contradict. It is a rule a test disputes, but it is in `money.py`
  and about rounding, too close to quote-split.
- `Order.set_quantity` with a rule letting placed orders change, which `test_only_drafts_change`
  contradicts. It is a rule a test disputes, but its note contradicts can-cancel's note in the
  same store.
- Paging in `iter_records`: too close to store-count.
- `untaxed_lines`: in `quote.py` and `tax.py` (line-taxes and tax-with-rate), and a circular import
  in `models.py`.

## The held-out tasks

Directory `bench/tasks-heldout/`:
five specs (`*.toml`), the pool `pool/notes-heldout.toml`, the hidden tests `files/`, the
reference edits `reference/edits.py`, `reference/check.py` (a byte-for-byte copy of
`bench/tasks-relay/reference/check.py`, which finds `edits.py` and `../files` beside it) and
`check_set.py`. The pool is in a subdirectory because `spec_files` reads every `*.toml` of the
task directory as a spec. Every spec has the M0 specs' fields
(`~/bench-repos/tallyho`, `base_rev = "main"`, the venv's pytest as `verify_cmd`, `max_turns =
30`, `session_timeout_secs = 900`) and `model = "claude-haiku-5-5"`. M0's `haiku-tree.sh` changes
exactly 13 task files under `tasks-m0` and `tasks-relay` and refuses any other diff, so the new
specs carry the run's model themselves.

**The test of a goal.** For each task: would someone who reads only the goal and the note, without
the code, see that the note is wrong? If so, the task has `relay-order-merge`'s fault. For all
five the answer is no. The goal says what the method must do. The note claims something about the
code that is consistent with the goal, so it takes reading the code to find out that it is false.

### 1. `relay-order-can-cancel`: a rule carried over from another operation

- **Changes** `tallyho/models.py`. **Goal:** "The order page hides its Cancel button once an
  order can no longer be cancelled. Add `Order.can_cancel()`: True while the order can still be
  cancelled, False once it cannot."
- **Wrong note** (`decision`, `no_commit`, anchored `tallyho/models.py` `Order`): "Workflow.cancel
  takes only draft orders, the same rule Order.add and Order.remove follow. Write Order.can_cancel
  as self.status is OrderStatus.DRAFT, beside them." Body: "So the order page can ask the order
  itself, without a Workflow."
- **Reference fix:** `return self.status in (OrderStatus.DRAFT, OrderStatus.PLACED)`. **Dead
  end:** `return self.status is OrderStatus.DRAFT`. Pattern
  `return\s+self\.status\s+(is|==)\s+OrderStatus\.DRAFT\b`.
- **Hidden test** (`test_relay_order_can_cancel.py`, the `order` fixture of `conftest.py`):
  - for every `OrderStatus`, `order.can_cancel()` equals whether `Workflow(Ledger()).cancel(order)`
    goes through without `ValidationError`;
  - after `place`, `can_cancel()` is true; after `cancel`, false;
  - after `place` and `pay`, false.
  
  The dead end fails on `PLACED`.
- **Silent:** no test touches cancellation, and nothing in `models.py` says when an order can be
  cancelled. What the note says of `add` and `remove` is true. The only rule for cancelling is
  `_ALLOWED` in `workflow.py`, which a session working in `models.py` need not open.
- **A dream can dispute it from the code alone:** the note states what `Workflow.cancel` accepts,
  and `workflow.py` line `OrderStatus.PLACED: {OrderStatus.PAID, OrderStatus.CANCELLED},` shows
  that a placed order is accepted too.
- **Room for raw:** a session that greps for `cancel` finds `_ALLOWED` and may see through the
  note. Unknown until it runs.

### 2. `relay-delete-order`: a false error contract under a reason that holds

- **Changes** `tallyho/repository.py`. **Goal:** "Add `Repository.delete_order(oid)`: it removes
  the stored order with that id and returns True, or returns False when there is no such order."
- **Wrong note** (anchored `tallyho/repository.py` `Repository`): "RecordStore.delete raises
  NotFound for an id that is not there, as get does. Write delete_order as try: store.delete(...)
  except NotFound: return False, then return True; no get() first." Body: "A get() before the
  delete reads the collection file a second time on JsonFileStore."
- **Reference fix:** `get` first (`NotFound` → False), then `delete`, True. **Dead end:** `delete`
  inside `try/except NotFound`, which never raises, so it always returns True. Pattern
  `delete\(\s*ORDERS[^)]*\)\s*except\s+NotFound`.
- **Hidden test**, on both backends (`MemoryStore` and `JsonFileStore(tmp_path)`), with two
  stored orders:
  - `delete_order("o1") is True`, then `order("o1")` raises `NotFound` and `orders()` lists only o2;
  - `delete_order("o9") is False`, and both orders remain;
  - deleting o2 twice gives True, then False.
- **Silent:** the store tests never delete a missing id, and nothing in the repository calls
  `delete` on a missing record. The note's reason is true: with `JsonFileStore`, `get` and
  `delete` each load the file.
- **A dream can dispute it from the code alone:** `store/base.py`: `"""Remove the record;
  missing records are ignored."""`; `memory.py` and `jsonfile.py` delete with `pop(record_id,
  None)`. The revised text says both "a reason that holds keeps the advice" and "advice holds or
  fails with its fact". This note tests which one the dream follows.
- **Nearest fitted:** store-count (a `RecordStore` docstring in `base.py`). Kept because the
  mistake (an error contract) and the reason that holds are new, and the evidence is not in the
  anchored file.

### 3. `relay-order-comments`: one backend is not like the others

- **Changes** `tallyho/repository.py`. **Goal:** "Staff want to keep short comments on an order.
  Add `Repository.add_order_comment(oid, text)` and `Repository.order_comments(oid)`, which returns
  that order's comments as a list of strings, oldest first, and an empty list for an order without
  any. Keep them in the shop's store, so they survive a restart."
- **Wrong note** (anchored `tallyho/repository.py` `Repository`): "Collections that hang off an
  order are named \"order-<what>\": comments go in \"order-comments\". Every RecordStore backend
  takes the collection name as an opaque key, so any string works." Body: "Keeps them apart from
  \"orders\" when the store is listed or opened on disk."
- **Reference fix:** one record per order, `{"id": oid, "comments": [...]}` in the collection
  `"order_comments"`. **Dead end:** the same code with `"order-comments"`. Pattern
  `["']order-comments["']`.
- **Hidden test:** three stored orders, on both backends:
  - o1 gets two comments and o2 one; `order_comments` gives them per order, oldest first;
  - o3 gives `[]`;
  - a comment written through one `Repository(JsonFileStore(tmp_path))` is read back through a
    new one.
  
  The dead end passes on `MemoryStore` and fails on `JsonFileStore`
  (`ValidationError: collection: bad collection name 'order-comments'`).
- **Silent:** the repo's own tests never give a store a new collection name. `MemoryStore`, the
  CLI's `--dry-run` store, takes any name. The only refusal is in `jsonfile.py`.
- **A dream can dispute it from the code alone:** `store/jsonfile.py`: `if not
  collection.isidentifier():` raises `ValidationError(f"bad collection name …")`. The note states
  a fact (every backend takes any name) beside a rule (the name), and the text says a contradicted
  fact is enough.

### 4. `relay-ledger-revenue`: a sign convention

- **Changes** `tallyho/ledger.py`. **Goal:** "Add `Ledger.revenue()`: the revenue the ledger has
  booked so far, as Money (zero when nothing has been sold)."
- **Wrong note** (anchored `tallyho/ledger.py` `Ledger.balances`): "Ledger.balances() gives each
  account as the shop reports it: cash, revenue and tax_payable all come out as positive amounts.
  Write Ledger.revenue() as self.balances()[\"revenue\"], no sign change." Body: "The trial
  balance check sums the same balances, so revenue() reads the number it checks."
- **Reference fix:** `-self.balances().get("revenue", Money.zero(self.currency))`. **Dead end:**
  the same without the minus. Pattern `return\s+self\.balances\(\)`.
- **Hidden test:**
  - one sale of the `order` fixture gives `revenue() == quote.discounted == Money.of("45.00")`;
  - two sales give 90.00;
  - an empty ledger gives `Money.zero()`.
  
  The dead end gives -45.00.
- **Silent:** no test reads a balance, and `trial_balance_ok` only sums them. A session must
  combine `balances()` (debits minus credits) with `record_sale` (revenue is credited) to see the
  sign. The note's facts about cash and about the trial balance are true.
- **A dream can dispute it from the code alone:** `out[e.account] += e.debit.minor -
  e.credit.minor` and `("revenue", z, quote.discounted),`, both in `ledger.py`.
- **Room for raw:** the anchored method shows the formula. A session that reads it with care sees
  through the note, so this task may have little room, as tax-with-rate had none.

### 5. `relay-coupon-days-left`: an inclusive/exclusive boundary

- **Changes** `tallyho/pricing/coupons.py`. **Goal:** "Customers ask how long a coupon stays
  valid. Add `Coupon.days_left(today)`: on how many days, counting from today, the coupon can
  still be used; 0 once it cannot be used any more, and None for a coupon that never expires."
- **Wrong note** (anchored `tallyho/pricing/coupons.py` `Coupon`): "Coupon.expires is the first
  day a coupon no longer works: one that expires on 10 March is good to 9 March. Days left =
  (expires - today).days, floored at 0; the expires day does not count." Body: "Customers read it
  off the coupon page, so make it a method of Coupon."
- **Reference fix:** `max((self.expires - today).days + 1, 0)`. **Dead end:** without `+ 1`.
  Pattern `\(\s*self\.expires\s*-\s*today\s*\)\.days\s*,\s*0\s*\)`.
- **Hidden test** (expiry 2026-03-10):
  - 1 on 03-10, 3 on 03-08, 0 on 03-11 and on 04-01;
  - `None` without an expiry;
  - for every day from 03-07 to 03-13, `days_left(day) > 0` equals `usable(Money.of("50.00"),
    "standard", day)`.
- **Silent:** no test touches expiry. **But** the contradicting line sits in the same class, a
  few lines above where the method goes.
- **A dream can dispute it from the code alone:** `if self.expires is not None and today >
  self.expires:` (a coupon is still usable on its expiry day).
- **Why it is in the set:** it is the fitted route: a fact contradicted by one line of the
  anchored code, as coupon-remove and money-plain. If the dream disputes this note and not the
  cross-file ones, that says where the text stops, not that it fails everywhere. The owner kept
  it in the set (decision 1); its per-task numbers are reported so that a reader can drop it.

### The true lessons beside them

Five true lessons, decisions, not `no_commit`, so each is also an empty commit, as the M0 pool's
true lessons are. Each states something about code on `main` that can be checked, so a dream can
judge them, where amendment 2's freeze judged none of its five. None says anything about the five
tasks or their evidence lines:

| pair | anchor | what it says, and why it is true |
|---|---|---|
| `pad-width` | `tallyho/text.py` `pad` | `pad` returns exactly `width` characters, cutting to `width - 1` plus "…" (the code; `test_pad`) |
| `csv-by-hand` | `tallyho/report/csv_export.py` `rows_to_csv` | written without the `csv` module so the bytes are stable; lines end in a bare newline; a cell is quoted only when it holds a comma (the module docstring, `_cell`) |
| `config-upper` | `tallyho/config.py` `load_config` | `[tax]` regions are upper-cased because `configparser` lower-cases option names (a fact about Python, as tax-with-rate's, but true) |
| `error-field` | `tallyho/errors.py` `ValidationError` | raise with `field=`; `str()` reads "field: message"; every check in `validate.py` does (the code) |
| `discount-order` | `tallyho/pricing/discounts.py` `apply_discounts` | percentages added and taken off together, at most 100 %, then fixed amounts; never below zero (the docstring, the code) |

A true lesson that states a rule the code breaks (like `customer-region`, which stage B's freeze
disputed and the revision was written against) is left out as a near-copy of fitted material.

### `tallyho` is reused

The held-out tasks reuse the `tallyho` repository unchanged (`bench/tallyho`). Only the pool and
the tasks are new.

**Whether reuse leaks.** What was fitted is the text of `DREAM.md`. The model keeps no memory from
one call to the next, so a dream on a `tallyho` store learns nothing from the pilots. The text
names nothing of `tallyho` (it was searched for `allocate`, `qualname`, `coupon`, `Money` and
`tallyho`: none). Reuse leaks only through what the person who revised the text saw:

- the dreamer reading `tallyho` code;
- the lines it quoted (listed above);
- the wording of the 13 notes.

So:

- **Lines.** None of the held-out evidence lines is a line any fitted verdict quoted. Two
  evidence files are files a fitted verdict quoted from: `store/base.py` (delete-order) and
  `pricing/coupons.py` (coupon-days-left).
- **Files.** In a repository of 1,275 lines of Python (tests included), almost every file had a fitted record anchored to it.
  The held-out anchors (`models.py`, `repository.py`, `ledger.py`, `coupons.py`) all had one (a
  wrong note on `Order` and on `CouponBook`, true lessons on `Repository` and `Ledger.record_sale`).
  None of those records is in the held-out pool.
- **Wording.** The new notes follow the old ones' form (a decision, "Write X as …", a body
  with a reason), because the harness and the pool format are the same. That is a leak of form,
  not of content.

A new repository would remove the first two leaks. It would also change two things at once: the
tasks and the code the sessions and the dream read. It needs a new `make-repo`, new
dependencies, and raw rates nobody has seen. The recommendation is to reuse `tallyho` here and
to take a new repository as the next step if this run shows a gain.

## Pre-registration

### Question

**On five tasks whose wrong notes are in no store the revised `DREAM.md` was fitted on, is an
agent accepted more often when its store has had one verified dream pass, made with the frozen
text, than with the raw records?**

### Arms

As amendment 2: `av` raw against `av --dreamed` (one frozen dream, replayed into every session of
the dreamed arm), `AV_PLAIN_RECORDS=on`, no `--wrong-reverted`, `--jit-hooks`, Claude Haiku 5.5
sessions at M0 amendment 1's prices with `--session-cap-usd 0.08`, run seed 1. The dreamer and
the verifier are Sonnet 5.5 with `M2_RUN_CAP=0.25`, `--limit 50`, no `--until-empty`.

### Tasks and pool

`bench/tasks-heldout` (the five tasks above) under `pool/notes-heldout.toml` (ten notes: five
wrong, five true). Each task's store holds all ten, as each M0 store held all 13 of its pool.

### Sizes

`--reps 20`: 5 tasks × 20 = **100 sessions per arm**, 200 in all.

Wilson 95 % intervals (`margincode_bench::stats::wilson_interval`):

| sessions | accepted | rate | interval |
|---|---|---|---|
| 100 | 50 | 0.500 | 0.404 to 0.596 |
| 100 | 52 | 0.520 | 0.423 to 0.615 |
| 100 | 70 | 0.700 | 0.604 to 0.781 |
| 150 | 75 | 0.500 | 0.421 to 0.579 |
| 150 | 99 | 0.660 | 0.581 to 0.731 |

- At 100 a side, a raw rate of 0.50 needs dreamed at 70 of 100 or more for disjoint intervals: a
  gap of about 0.20. At 150 a side (30 a task) the gap needed is about 0.16; at 90 a side
  (amendment 2's size), about 0.21.
- Amendment 2's in-sample gap was 0.48. **An illustration, not a prediction:** suppose raw is
  misled on four tasks at 0.35 each and right on the fifth, so raw is about 0.48. If the dream
  corrects three of the four fully, dreamed is about 0.87 and the intervals are disjoint at
  100. If it corrects only one, dreamed is about 0.61, and that is not shown even at 150. The
  pooled rule can show a gain over the set; it cannot show a gain on one task.
- No claim unless each arm has at least 50 valid sessions with at least 10 on each task.
  Sessions are not rerun.

### Claim rule (unchanged)

M0's, as stage B and amendment 2: a difference is claimed only if the Wilson intervals are
disjoint. **Dreaming helps** if dreamed's lower end is above raw's upper end; **hurts** if
dreamed's upper end is below raw's lower end; otherwise **not shown at this size**. Pooled over
the five tasks, valid sessions only. The M0 comparison row below is never pooled with it.

### `DREAM.md` is frozen

- `integrations/dream/DREAM.md` stays at the text merged in #63: last changed in `dcba022` (merge
  `96286c3`), sha256 `f66dc2ec655cf8747df585d35c4dc8d0e7205f14ebd3220da50fc220aa6973aa`.
  **Nothing touches it between now and the run.** A change to it before the run voids this
  amendment.
- **The binaries are amendment 2's**, built from `21f30dc`, which embed that text: `av`
  `ff48898161a2d38b88f52535170e545e796b53784ec4254ff56e72e89adc27b9`, `margincode-daemon`
  `a5702a789ea90ae797cd76963d5d031ae9eb931f012a60b1a4dbe711f2302b1c`, `margincode-bench`
  `a1f7f1e207ef04036e1d45dbab157f5c74c9d5136e46e420357a1376614ec160`, `margincode`
  `c24399064efa56c1974204a7646fb626fca3af8736cb62597d6238368c44ba9a`. They are reused if `git
  diff 21f30dc <the run commit> -- crates integrations Cargo.toml Cargo.lock
  rust-toolchain.toml` is empty. At `d897611` it is: only `HANDOFF.md`, `docs/backlog.md` and
  `bench/results/` differ; the run commit adds only `bench/tasks-heldout/` and files under
  `bench/results/` to that, and the Build records the diff at the run commit. Otherwise they
  would have been rebuilt as amendment 2 built them, with new sha256s in the Build.
- **A freeze condition:** every review and verify run of the freeze has `instructions_sha`
  `f66dc2ec…` (the sha256 of the text `av dream` hands the agent,
  `crates/margincode-cli/src/dream.rs`; no `--promote`, no `--also-read`).
- **Before the freeze:** no dream of any kind on a held-out store. That includes a $0 seeded run
  a model then reads. No task, note or hidden test changes after the freeze. The owner fixed the
  set at all five tasks (decision 1), so a task that fails a free check is not dropped: the run
  stops before any spend and the cause is reported.

### Free checks

Before the freeze (the first paid command), none of which needs a dream:

1. `python3 bench/tasks-heldout/check_set.py ~/bench-repos/tallyho`: ok.
2. `~/bench-work/venv/bin/python bench/tasks-heldout/reference/check.py`: all 5 pairs ok, 5 of
   them silent (the real-pytest form of the local check above).
3. `margincode-bench check bench/tasks-heldout`: 5 of 5 `ok`.
4. `first-packs` for the held-out tasks, raw: no line containing `--supersedes`, no origin label,
   each task's wrong note in its hook pack (5 of 5).

A check that fails for a typo or a path is fixed and all the checks are run again; any other
failure stops the run before any spend. No paid command runs until 1 to 4 pass on all five
tasks. Before the freeze, two more free checks of the stack (no held-out dream): `first-packs
bench/tasks-m0 --dreamed` with amendment 2's `dreamed.json` replays on this run's new database
(the M0 row's dreamed arm), and `run bench/tasks-heldout --arms cold` without `--confirm` prints
its plan (the cold arm's flags).

After the freeze, before the smoke:

5. `first-packs` with the new frozen dream: against raw, the packs differ only where the frozen
   verdicts say. If the M0 row runs, the same for `bench/tasks-m0` with amendment 2's
   `dreamed.json`.

If check 5 fails, no session runs and the cause is reported.

**One freeze.** It is made once and used as it comes out, whatever its verdicts. It is used only
if, for each task, every record of its store (10) was handed to the review run, `dream-freeze`
exits 0, and every run's `instructions_sha` is the frozen text's. Otherwise no session runs.

**The smoke**, as amendment 2: `relay-order-ids`, raw, one session, under amendment 1's
conditions.

### Reported beside the primary

As amendment 2, plus:

- **Per task, the verdict its own wrong note ended with** in its own store (k of 5 `disputed`,
  any `confirmed`). This is the held-out measure of the text itself, free of session noise. It is
  split by where the evidence is: in another file (can-cancel, delete-order, comments: 3) or in
  the anchored file (revenue, days-left: 2).
- **True lessons:** how many were judged at all, and any `disputed` or `outdated`.
- The undecided share; per-task accepted counts; dead ends repeated; sessions that tried
  `--supersedes`.
- **A task where raw has no room** (20 of 20, as tax-with-rate was) is reported as such. It stays
  in the primary.
- A wrong note that ended `confirmed`, with each arm's count on its task. delete-order is where
  the text's two bullets pull apart.

### The M0 comparison row (decision 6: run)

`bench/tasks-m0`, raw and dreamed, `--reps 10` (30 a side). The dreamed arm reuses amendment 2's
frozen `m2-stage-b-revised/dreamed.json` (sha256 `fe405a5b…`, same binaries, same pool), so no
second freeze. It runs after the held-out arms in the same session of work. **Described beside,
never pooled, no claim.**

It answers one question: if the held-out arms show no gain, is the in-sample gain still there on
the same day, stack and build? If it is, the difference is the tasks.

### What a result would show, and what not

- **Helps:** one frozen dream, from the text as fitted, raised the accepted rate on five tasks it
  was not fitted on, on Haiku 5.5, in one repository the text's author had seen. It would still
  not show it for another repository or another model. Nor would it show it for notes written by
  someone who had not read the text (the designer confound above).
- **Not shown:** the per-task verdicts and the split by evidence place say where it stopped.
- **Hurts:** reported as it stands, with the confirmed or wrongly disputed records named.

## Money

**Recorded costs:**

- Amendment 2's freeze: **$0.6079** for three stores, 6 Sonnet 5.5 calls (mean $0.101, largest
  $0.139).
- A Haiku 5.5 session: about **$0.005** (raw $0.4846 for 90, $0.0054 each; dreamed $0.4481 for
  90, $0.0050).
- Smoke: $0.0076.

**Cost estimate (an estimate):**

| step | basis | estimate |
|---|---|---|
| freeze | 5 stores × 2 calls × about $0.10 (10 records a store against 13; a call's cost is mostly its fixed overhead, so a guess) | about $1.0 ($0.8 to $1.3) |
| smoke | amendment 2 | $0.01 |
| held-out arms | 200 × $0.0054 (new tasks may run longer: up to 1.5×, a guess) | about $1.1 ($1.1 to $1.6) |
| M0 row | 60 × $0.0054 | about $0.3 |
| cold arm | 50 × $0.0054 (up to 1.5×) | about $0.3 ($0.3 to $0.4) |
| **total** | | **about $2.4** ($2.2 to $3.2) without the cold arm; **about $2.7** ($2.5 to $3.6) with it |

**The freeze's worst case** is 5 tasks × (one review and one verify run) × 2 calls (one fix
round) × $0.25 = **$5.00**.

**Ceiling: $5.00** for all of this run's paid work, the money rule of amendment 2 unchanged:

- Spend is counted as it is spent: the freeze's `total_cost_usd` over every
  `claude-result-*.json`, plus the harness's session costs (`rows.jsonl`), summed and logged
  after each command.
- Before the freeze, nothing has been spent. The freeze runs with its fixed per-call cap, and its
  worst case equals the ceiling.
- Before the smoke and before each arm: at $5.00 spent or more, nothing more runs. Otherwise the
  command runs with **the smaller of its written budget and $5.00 less what has been spent**, so
  no command's budget can take the total over $5.00 (the harness stops launching once spent plus
  one session's $0.08 cap would pass its budget). A command whose budget so computed is below
  one session's cap ($0.08) is not run, and the README says so.
  Written budgets: smoke $0.10; each held-out arm $2.30 (amendment 2's $2.10 for 90, scaled to
  100); each M0-row arm $0.70 (stage B's for 30); the cold arm $0.40.
- An arm that stops part-way for money is reported as it stands and not rerun. The order of the
  commands below puts the primary first, then the M0 row, then the cold arm, so money cuts the
  cold arm first and the M0 row next.
- At the estimates, no budget binds. Arithmetic on the table, sessions at $0.0054 (and, in
  brackets, at 1.5× that): the cold arm is cut only if the freeze costs more than about $3.3
  ($2.5); the M0 row only above about $3.6 ($2.9); the dreamed held-out arm only above about
  $3.9 ($3.4). The freeze's worst case, $5.00, would leave nothing for any session.

## Decisions (fixed by the owner, 2026-11-19)

The draft left eight choices open; the owner settled all of them as it recommended ("Run as
recommended"):

1. **Tasks:** all 5, coupon-days-left (the fitted route) included as the in-route contrast.
2. **Sessions per task:** `--reps 20`, 100 sessions per arm.
3. **Session model:** Claude Haiku 5.5 (`claude-haiku-5-5`), as the fitted run.
4. **Dreamer and verifier model:** Claude Sonnet 5.5 (`claude-sonnet-5-5`).
5. **Ceiling:** $5.00 of money spent for all of this run's paid work.
6. **The M0 comparison row:** run, `--reps 10` (30 a side) on `bench/tasks-m0`, reusing amendment
   2's `dreamed.json`; reported beside, never pooled, no claim.
7. **The repository:** `tallyho`, reused unchanged.
8. **A `cold` arm** on the held-out tasks: run, `--reps 10` (50 sessions), last, described only.
   The harness's `cold` arm (`crates/margincode-bench/src/runner.rs`, `Arm::Cold`) gives the agent
   the scrubbed base tree with neither the relay commits nor av, so it needs no flag beyond
   `--arms cold`; M0 ran it the same way under `benchagent` (`../2026-11-14-m0/haiku-chain.sh`).

## Build (committed before the first paid command)

As amendment 2, in commits pushed before the first paid command and the first session: this one,
before the freeze, names the build, the stack, the commands and the rules; the next adds the free
checks' results on the run tree; the one after it, before the smoke, adds the frozen
`dreamed-ho.json`'s sha256 and free check 5. The branch is `heldout-amendment-3`, off `main` at
`d897611`; it is pushed, not merged, before the run.

- **Binaries:** amendment 2's, in `/m2b/main-21f30dc/bin` and `/m2b/main-21f30dc/H/bin` (sha256
  as listed under "`DREAM.md` is frozen", checked again in the container before this commit). Not
  rebuilt: `git diff 21f30dc d897611 -- crates integrations Cargo.toml Cargo.lock
  rust-toolchain.toml` is empty, and this branch changes none of those paths.
- **Run tree:** `/m2b/r3/tree`, a `git archive` of this commit (no line-ending conversion), its
  tree id checked against the commit's; `/m2b/r3/tree-h`, made from it by M0's `haiku-tree.sh`
  (exactly the 13 `model` lines of `bench/tasks-m0` and `bench/tasks-relay` differ; the held-out
  specs already name `claude-haiku-5-5`). Both 700.
- **Task repository and venv:** amendment 2's (`/m2b/bench-repos/tallyho`, `/m2b/bench-work/venv`,
  Python 3.13.5, pytest 9.2.0). `git diff 21f30dc d897611 -- bench/tallyho bench/tasks-m0
  bench/tasks-relay` is empty.
- **Container and services:** amendment 2's `m2b-harness` (Claude Code 2.1.318,
  `DISABLE_AUTOUPDATER=1`, the agent user `benchagent`) with `lp-pg`, `lp-qdrant` and `lp-tei`,
  started again after a stop.
- **Daemon:** `127.0.0.1:7493`, new database `m2h`, store `/m2b/r3/store`, workspace root
  `/m2b/r3/bench-work/ws`, `AV_PLAIN_RECORDS=on`, started by amendment 2's
  `m2-stage-b-revised/daemon.sh 7493 m2h /m2b/r3` (unchanged) with umask 022. `GET /v1/version`
  answers `"plain_records":true`; the database holds migrations to 0032. One daemon, not
  restarted, serves the free checks, the freeze, the smoke and every arm. No other daemon of the
  bench runs (runs 1 and 2's, ports 7491 and 7492, are not started).
- **Options:** `HOME=/m2b`; `--av-url http://127.0.0.1:7493 --work-root /m2b/r3/bench-work
  --import-dir /m2b/r3/bench-work/src --import-mount /m2b/r3/bench-work/src --av-bin
  /m2b/main-21f30dc/bin/av`; for sessions also `--agent-user benchagent --av-ws-root
  /m2b/r3/bench-work/ws`. `m2-heldout/m2hcmd.sh` runs one command with these options (amendment
  2's `m2rcmd.sh` with the new paths); `m2-heldout/m2hchain.sh` runs the smoke or one arm and
  logs it to `checks/chain.log`.
- **Output names:** `smoke-r3`, `m2-raw-ho`, `m2-dreamed-ho`, `m2-raw-m0c`, `m2-dreamed-m0c`,
  `m2-cold-ho`. A session's repository is named after its `--out` (`bm2rawho-…`); none of these
  names begins a repository of any database on `lp-pg` (checked before this commit: `bench`,
  `bench2`, `bench3`, `m0a`, `m0b`, `m0ha`, `m0hb`, `m1p0`, `m1p1`, `m2b`, `m2p`, `m2r`).
- **Isolation:** as amendment 2's: `/m2b/r3` 711; its store, results, checks and `bench-work/src`
  700; `bench-work` and `ws` 711; the first-packs and freeze directories and `dreamed-ho.json`
  closed (700) before the smoke; each command's run and workspace directories closed when it ends;
  `/tmp` emptied before each command.
- **Dreamer and verifier:** stage B's `dreamer.sh` and `verifier.sh` (both sha256
  `e2452e7f04cce6ce1e548caf388fec010961b9a4d11d3d6f6497daca6539eea9`), unchanged, with
  `M2_MODEL=claude-sonnet-5-5` and `M2_RUN_CAP=0.25`.
- **The M0 row's dream:** amendment 2's `dreamed.json`, copied to `/m2b/r3/results/dreamed-a2.json`;
  its sha256 must be `fe405a5b1131f4ab295b63e02564cfc8473500c7cea127bf606241ae42dbec7b`, and the
  dreamed M0 arm's `meta.json` must name it.
- **The sessions:** as amendment 2's (Claude Haiku 5.5, first-party API, the key passed in by name
  and never written down, `--price-per-mtok 0.10,0.125,0.01,0.50`, `--jit-hooks` in the av arms,
  no MCP, no `--keep-work`, no noise, run seed 1, `--session-cap-usd 0.08`, no `--wrong-reverted`).

**Commands** (`m2hcmd.sh <subcommand> <args>`; results to `/m2b/r3/results/<out>`, copied to
`m2-heldout/<out>` as the harness wrote them; free checks 1 to 3 run from `/m2b/r3/tree-h`):

```bash
python3 bench/tasks-heldout/check_set.py /m2b/bench-repos/tallyho                  # free check 1
/m2b/bench-work/venv/bin/python bench/tasks-heldout/reference/check.py \
  /m2b/bench-repos/tallyho /m2b/bench-work/venv/bin/python                         # free check 2
margincode-bench check bench/tasks-heldout                                         # free check 3
m2hcmd.sh first-packs bench/tasks-heldout --out /m2b/r3/results/first-packs-ho-raw # free check 4
m2hcmd.sh first-packs bench/tasks-m0 --dreamed /m2b/r3/results/dreamed-a2.json \
  --out /m2b/r3/results/first-packs-m0c-dreamed           # the M0 row's replay on this database
margincode-bench run bench/tasks-heldout --arms cold --reps 10 ...  # no --confirm: the cold plan
# the freeze (paid, Sonnet 5.5): one review run and one verify run per held-out task
m2hcmd.sh dream-freeze bench/tasks-heldout --limit 50 \
  --dream-cmd 'sh /m2b/dreamer.sh' --verify-cmd 'sh /m2b/verifier.sh' \
  --out /m2b/r3/results/dreamed-ho.json
# free check 5
m2hcmd.sh first-packs bench/tasks-heldout --dreamed /m2b/r3/results/dreamed-ho.json \
  --out /m2b/r3/results/first-packs-ho-dreamed
# the smoke (not a result), then the primary, then the M0 row, then cold; each BUDGET is the
# smaller of the written figure and $5.00 less what has been spent
m2hchain.sh smoke 0.10        # bench/tasks-relay/order-ids.toml --arms av --jit-hooks --reps 1
m2hchain.sh raw-ho 2.30       # bench/tasks-heldout --arms av --jit-hooks --reps 20
m2hchain.sh dreamed-ho 2.30   #   the same with --dreamed /m2b/r3/results/dreamed-ho.json
m2hchain.sh raw-m0c 0.70      # bench/tasks-m0 --arms av --jit-hooks --reps 10
m2hchain.sh dreamed-m0c 0.70  #   the same with --dreamed /m2b/r3/results/dreamed-a2.json
m2hchain.sh cold-ho 0.40      # bench/tasks-heldout --arms cold --reps 10
```

- **The smoke**, under amendment 1's conditions: every row `status=ok` and valid; the model
  answered and made at least one edit; verify ran; `hook-count` not empty; `meta.json` names
  `benchagent`, Claude Code 2.1.318, `models: ["claude-haiku-5-5"]` and `plain_records: true`; no
  single request with a prompt above 100K tokens; no session above $0.04. If it fails, no arm
  runs and the cause is reported.
- A halt: `/m2b/checks/STOP` refuses every command of `m2hcmd.sh`, as amendment 2's.
- **Counting.** The primary is counted from the committed `rows.jsonl` of `m2-raw-ho/` and
  `m2-dreamed-ho/` (valid = `status` `ok`; accepted = `success`), the intervals by
  `margincode_bench::stats::wilson_interval`, and the count is made a second time from the
  committed rows before the README states it.

## Not known before the run

- Whether Haiku sessions follow these notes at all, which decides whether raw has room. Only a
  paid session shows it.
- What the freeze does with the five wrong notes and the five true lessons.
