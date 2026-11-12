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
- rec:10ad8de7 attempt: Tried calling `Repository.order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.order
- rec:c425a9cf attempt: Moving the imports inside `Repository.save_order` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/repository.py:Repository.save_order
- rec:ea5c6c71 attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:805d76c1 attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:cb25b6ed decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:46f48541 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:c66c6bd8 decision: New optional parameters of `OrderLine.__post_init__` are keyword-only…

## Code
- sym:12545 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:12544 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:12547 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:12546 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:12537 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:12548 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:12484 Order.add (tallyho/models.py:76-84)
- sym:12520 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:12482 Order (tallyho/models.py:64-109)
  class Order:
- sym:12488 Order.to_dict (tallyho/models.py:98-109)
- sym:12511 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:12523 quote_order (tallyho/pricing/quote.py:30-53)
- sym:12550 RecordStore.put (tallyho/store/base.py:24-25)
- sym:12596 order (tests/conftest.py:22-26)
- sym:12612 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 685/800
