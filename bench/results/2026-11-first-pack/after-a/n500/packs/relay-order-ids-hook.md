## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.
Workspace: /root/bench-work/ws/bfpd911-relay-order-ids-av-1/relay-order-ids
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:3e29038c attempt: Tried calling `Repository.order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.order
- rec:9c14a4a5 attempt: Wrapping `Repository.save_order` in a bare try/except with a retry loo…
- rec:89b432ba attempt: Adding type: ignore comments around `Repository._order_from_record` to…
- rec:b81e5d65 decision: New optional parameters of `Repository._order_from_record` are keyword…
- rec:9064936c decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:46a76e7f decision: `Repository.save_order` keeps its docstring as the one place its behav…
- rec:9918c643 note: `Repository.save_order` is on the hot path of the nightly batch job; p…

## Code
- sym:21385 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:21384 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:21387 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:21386 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:21377 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:21388 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:21324 Order.add (tallyho/models.py:76-84)
- sym:21360 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:21322 Order (tallyho/models.py:64-109)
  class Order:
- sym:21328 Order.to_dict (tallyho/models.py:98-109)
- sym:21351 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:21363 quote_order (tallyho/pricing/quote.py:30-53)
- sym:21390 RecordStore.put (tallyho/store/base.py:24-25)
- sym:21436 order (tests/conftest.py:22-26)
- sym:21452 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 689/800
