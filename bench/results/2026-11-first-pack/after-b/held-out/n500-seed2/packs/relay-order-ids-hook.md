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
- rec:0b2583b1 decision: New optional parameters of `Repository.orders` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.orders
- rec:f416e066 note: Remember `Repository.order` is not thread-safe; each worker needs its own instance or an external lock.
  anchors: tallyho/repository.py:Repository.order
- rec:6ee54192 attempt: Tried calling `Repository.save_order` twice to force a refresh; the se…
- rec:f929d88e attempt: Tried calling `quote_order` twice to force a refresh; the second call…
- rec:40fc3930 attempt: Tried calling `Order.remove` twice to force a refresh; the second call…
- rec:78b96083 attempt: Tried calling `Repository.save_customer` twice to force a refresh; the…

## Code
- sym:39563 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:39562 Repository.save_order (tallyho/repository.py:54-55)
- sym:39565 Repository.orders (tallyho/repository.py:72-74)
- sym:39564 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:39555 Repository (tallyho/repository.py:32-74)
- sym:39566 Page (tallyho/store/base.py:10-14)
- sym:39502 Order.add (tallyho/models.py:76-84)
- sym:39538 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:39500 Order (tallyho/models.py:64-109)
- sym:39506 Order.to_dict (tallyho/models.py:98-109)
- sym:39529 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:39614 order (tests/conftest.py:22-26)

---
tokens: 703/800
omitted 29 lower-ranked items (expand an id for detail):
- rec:a59e3421 attempt: Moving the imports inside `Repository.save_order` to speed up start-up…
- rec:8eca74db attempt: Wrapping `Order.to_dict` in a bare try/except with a retry loop hid th…
- rec:5ef3d0dc attempt: Tried calling `Repository.__init__` twice to force a refresh; the seco…
- rec:c3664fe8 attempt: Wrapping `Order.subtotal` in a bare try/except with a retry loop hid t…
- rec:26b6609e attempt: Replacing the loop in `OrderStatus` with a comprehension made it short…
- rec:ec58a5ed attempt: Tried calling `PercentDiscount` twice to force a refresh; the second c…
- … and 23 more
