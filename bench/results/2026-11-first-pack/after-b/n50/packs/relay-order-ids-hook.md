## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:f0164f51 decision: New optional parameters of `Repository.save_order` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.save_order
- rec:659c6fa8 attempt: Tried calling `Repository.order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.order
- rec:1dc8c993 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:d6ae92d2 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:e2390706 attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:d1d0d900 intent: Earlier work on this code

## Code
- sym:34583 Repository.order (tallyho/repository.py:57-59)
- sym:34582 Repository.save_order (tallyho/repository.py:54-55)
- sym:34585 Repository.orders (tallyho/repository.py:72-74)
- sym:34584 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:34575 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:34586 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:34522 Order.add (tallyho/models.py:76-84)
- sym:34558 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:34520 Order (tallyho/models.py:64-109)
- sym:34526 Order.to_dict (tallyho/models.py:98-109)
- sym:34549 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:34634 order (tests/conftest.py:22-26)

---
tokens: 707/800
omitted 29 lower-ranked items (expand an id for detail):
- rec:31dd374d attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:18995ddf attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:7fb1aca2 attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:92a00528 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:deaf505e attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:9336733b attempt: Wrapping `cmd_products` in a bare try/except with a retry loop hid the…
- … and 23 more
