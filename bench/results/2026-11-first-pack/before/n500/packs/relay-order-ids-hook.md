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
- rec:29d691b1 attempt: Tried calling `Repository.order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.order
  A sleep between the calls did not help either. It needs a real invalidation path, not a retry.
- rec:0d7a2637 attempt: Wrapping `Repository.save_order` in a bare try/except with a retry loo…
- rec:d552d4eb attempt: Adding type: ignore comments around `Repository._order_from_record` to…
- rec:de9b0d7c decision: New optional parameters of `Repository._order_from_record` are keyword…
- rec:30a441cf decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:63d2a295 decision: `Repository.save_order` keeps its docstring as the one place its behav…
- rec:9153b684 note: `Repository.save_order` is on the hot path of the nightly batch job; p…

## Code
- sym:9225 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:9224 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:9227 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:9226 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:9217 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:9228 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:9164 Order.add (tallyho/models.py:76-84)
- sym:9200 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:9162 Order (tallyho/models.py:64-109)
  class Order:
- sym:9168 Order.to_dict (tallyho/models.py:98-109)
  def to_dict(self) -> dict:
- sym:9191 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:9203 quote_order (tallyho/pricing/quote.py:30-53)
- sym:9230 RecordStore.put (tallyho/store/base.py:24-25)
- sym:9276 order (tests/conftest.py:22-26)
- sym:9292 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 687/800
