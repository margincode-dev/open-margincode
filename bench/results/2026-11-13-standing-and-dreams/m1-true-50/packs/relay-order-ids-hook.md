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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:02fd15f9 decision, open: New optional parameters of `Repository.save_order` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.save_order
- rec:95c0a051 attempt, open: Tried calling `Repository.order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.order
- rec:da0507b9 attempt, open: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:6386293a attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:fc0e14e7 attempt, open: Wrapping `count_records` in a bare try/except with a retry loop hid th…

## Code
- sym:4245 Repository.order (tallyho/repository.py:57-59)
- sym:4244 Repository.save_order (tallyho/repository.py:54-55)
- sym:4247 Repository.orders (tallyho/repository.py:72-74)
- sym:4246 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:4237 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:4248 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:4184 Order.add (tallyho/models.py:76-84)
- sym:4220 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:4182 Order (tallyho/models.py:64-109)
  class Order:
- sym:4188 Order.to_dict (tallyho/models.py:98-109)
- sym:4211 CouponBook.get (tallyho/pricing/coupons.py:49-50)

---
tokens: 699/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:3724cfb8 attempt, open: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:4a431ab3 attempt, open: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:e2754b50 attempt, open: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:e5561821 attempt, open: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:c1bb9993 attempt, open: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- … and 26 more
