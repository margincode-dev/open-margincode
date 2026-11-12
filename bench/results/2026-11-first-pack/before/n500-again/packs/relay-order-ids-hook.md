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
- rec:6f0b4dc9 attempt: Tried calling `Repository.order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.order
  A sleep between the calls did not help either. It needs a real invalidation path, not a retry.
- rec:8d4673ff attempt: Wrapping `Repository.save_order` in a bare try/except with a retry loo…
- rec:79e8c277 attempt: Adding type: ignore comments around `Repository._order_from_record` to…
- rec:4beb62e2 decision: New optional parameters of `Repository._order_from_record` are keyword…
- rec:3abd04e6 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:48604b70 decision: `Repository.save_order` keeps its docstring as the one place its behav…
- rec:217b163e note: `Repository.save_order` is on the hot path of the nightly batch job; p…

## Code
- sym:10885 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:10884 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:10887 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:10886 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:10877 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:10888 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:10824 Order.add (tallyho/models.py:76-84)
- sym:10860 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:10822 Order (tallyho/models.py:64-109)
  class Order:
- sym:10828 Order.to_dict (tallyho/models.py:98-109)
  def to_dict(self) -> dict:
- sym:10851 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:10863 quote_order (tallyho/pricing/quote.py:30-53)
- sym:10890 RecordStore.put (tallyho/store/base.py:24-25)
- sym:10936 order (tests/conftest.py:22-26)
- sym:10952 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 687/800
