## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.
Workspace: /root/bench-work/ws/bfpd022-relay-order-ids-av-1/relay-order-ids
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:c1c39f47 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:4970221c attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:413804e8 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000002, ...): the store lists records in id order, so padded sequence ids keep listings chronological. No UUIDs.
  anchors: tallyho/repository.py:Repository
- rec:db46f8fd decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:da4c7162 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:53c405fa decision: Errors about customer data must not echo the rejected value: they are…
- rec:6605b7fb intent: Earlier work on this code

## Code
- sym:18065 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:18064 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:18067 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:18066 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:18057 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:18068 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:18004 Order.add (tallyho/models.py:76-84)
- sym:18040 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:18002 Order (tallyho/models.py:64-109)
- sym:18008 Order.to_dict (tallyho/models.py:98-109)
- sym:18031 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:18043 quote_order (tallyho/pricing/quote.py:30-53)
- sym:18070 RecordStore.put (tallyho/store/base.py:24-25)
- sym:18116 order (tests/conftest.py:22-26)
- sym:18132 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 689/800
