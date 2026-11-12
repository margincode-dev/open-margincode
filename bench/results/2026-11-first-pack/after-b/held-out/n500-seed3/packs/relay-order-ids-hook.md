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
- rec:4d498c3b note: Remember `Repository._order_from_record` is not thread-safe; each worker needs its own instance or an external lock.
  anchors: tallyho/repository.py:Repository._order_from_record
- rec:6ed84c8a decision: New optional parameters of `Repository.order` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.order
- rec:5f872159 attempt: Tried calling `Repository.orders` twice to force a refresh; the second…
- rec:01147142 attempt: Tried calling `OrderStatus` twice to force a refresh; the second call…
- rec:3d979648 attempt: Tried calling `Order.currency` twice to force a refresh; the second ca…
- rec:a34dc445 attempt: Tried calling `Repository.save_customer` twice to force a refresh; the…

## Code
- sym:41223 Repository.order (tallyho/repository.py:57-59)
- sym:41222 Repository.save_order (tallyho/repository.py:54-55)
- sym:41225 Repository.orders (tallyho/repository.py:72-74)
- sym:41224 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:41215 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:41226 Page (tallyho/store/base.py:10-14)
- sym:41162 Order.add (tallyho/models.py:76-84)
- sym:41198 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:41160 Order (tallyho/models.py:64-109)
- sym:41166 Order.to_dict (tallyho/models.py:98-109)
- sym:41189 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:41274 order (tests/conftest.py:22-26)

---
tokens: 700/800
omitted 29 lower-ranked items (expand an id for detail):
- rec:c26ad3d9 attempt: Tried calling `CouponBook.add` twice to force a refresh; the second ca…
- rec:f96da5b8 attempt: Wrapping `Order.to_dict` in a bare try/except with a retry loop hid th…
- rec:6cb5349e attempt: Wrapping `Order` in a bare try/except with a retry loop hid the real e…
- rec:14adc88f attempt: Moving the imports inside `Repository.orders` to speed up start-up sav…
- rec:38090436 attempt: Replacing the loop in `Order.add` with a comprehension made it shorter…
- rec:4e2752fe attempt: Tried calling `RecordStore.list_page` twice to force a refresh; the se…
- … and 23 more
