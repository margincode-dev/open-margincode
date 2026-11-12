## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.
Workspace: /root/bench-work/ws/bfp92cd-relay-order-ids-av-1/relay-order-ids
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:8d7c880f attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:5e857dc8 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:216a2c2b decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000002, ...): the store lists records in id order, so padded sequence ids keep listings chronological. No UUIDs.
  anchors: tallyho/repository.py:Repository
- rec:c0cc55ad decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:8f9b5838 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:9afe1d0c decision: Errors about customer data must not echo the rejected value: they are…
- rec:8cc77458 intent: Earlier work on this code

## Code
- sym:5905 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:5904 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:5907 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:5906 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:5897 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:5908 Page (tallyho/store/base.py:10-14)
- sym:5844 Order.add (tallyho/models.py:76-84)
- sym:5880 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:5842 Order (tallyho/models.py:64-109)
- sym:5848 Order.to_dict (tallyho/models.py:98-109)
- sym:5871 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:5883 quote_order (tallyho/pricing/quote.py:30-53)
- sym:5910 RecordStore.put (tallyho/store/base.py:24-25)
- sym:5956 order (tests/conftest.py:22-26)
- sym:5972 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 690/800
