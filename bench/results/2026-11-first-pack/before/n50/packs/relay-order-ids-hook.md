## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.
Workspace: /root/bench-work/ws/bfp971f-relay-order-ids-av-1/relay-order-ids
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:66e5ebef attempt: Tried calling `Repository.order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.order
- rec:8cf86f1f decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:6d55fbfb decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:2b4d3a89 decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:533b2db4 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:244fe2c2 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:439b778f intent: Earlier work on this code

## Code
- sym:7565 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:7564 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:7567 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:7566 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:7557 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:7568 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:7504 Order.add (tallyho/models.py:76-84)
- sym:7540 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:7502 Order (tallyho/models.py:64-109)
  class Order:
- sym:7508 Order.to_dict (tallyho/models.py:98-109)
- sym:7531 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:7543 quote_order (tallyho/pricing/quote.py:30-53)
- sym:7570 RecordStore.put (tallyho/store/base.py:24-25)
- sym:7616 order (tests/conftest.py:22-26)
- sym:7632 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 685/800
