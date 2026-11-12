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
- rec:54032c38 decision: New optional parameters of `Repository.save_order` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.save_order
- rec:8382bb09 attempt: Tried calling `Repository.order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.order
- rec:1627c09d attempt: Moving the imports inside `Repository.save_order` to speed up start-up…
- rec:364350a1 attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:e792f3db attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:eb494611 intent: Earlier work on this code

## Code
- sym:37903 Repository.order (tallyho/repository.py:57-59)
- sym:37902 Repository.save_order (tallyho/repository.py:54-55)
- sym:37905 Repository.orders (tallyho/repository.py:72-74)
- sym:37904 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:37895 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:37906 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:37842 Order.add (tallyho/models.py:76-84)
- sym:37878 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:37840 Order (tallyho/models.py:64-109)
- sym:37846 Order.to_dict (tallyho/models.py:98-109)
- sym:37869 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:37881 quote_order (tallyho/pricing/quote.py:30-53)

---
tokens: 684/800
omitted 29 lower-ranked items (expand an id for detail):
- rec:6f1ef5a7 attempt: Tried calling `FixedDiscount` twice to force a refresh; the second cal…
- rec:b9d21a1b attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:5e1c48d6 attempt: Tried calling `EventBus` twice to force a refresh; the second call ret…
- rec:46e9f4c7 attempt: Tried calling `check_sku` twice to force a refresh; the second call re…
- rec:085f7064 attempt: Moving the imports inside `OrderLine.subtotal` to speed up start-up sa…
- … and 24 more
