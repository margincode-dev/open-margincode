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
- rec:91089c9b attempt: Tried calling `Repository.order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.order
  A sleep between the calls did not help either. It needs a real invalidation path, not a retry.
- rec:c3802044 decision: New optional parameters of `Repository.save_order` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.save_order
- rec:ed3671db decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:c0e5a25d decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:0aaf3af2 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:39fbeccf decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:e006fa87 intent: Earlier work on this code

## Code
- sym:19725 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:19724 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:19727 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:19726 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:19717 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:19728 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:19664 Order.add (tallyho/models.py:76-84)
- sym:19700 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:19662 Order (tallyho/models.py:64-109)
- sym:19668 Order.to_dict (tallyho/models.py:98-109)
- sym:19691 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:19703 quote_order (tallyho/pricing/quote.py:30-53)
- sym:19730 RecordStore.put (tallyho/store/base.py:24-25)
- sym:19776 order (tests/conftest.py:22-26)
- sym:19792 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 687/800
