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
- rec:00b8121f decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000002, ...): the store lists records in id order, so padded sequence ids keep listings chronological. No UUIDs.
  anchors: tallyho/repository.py:Repository
- rec:987d2901 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:b88c5eb2 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:4e8514ad attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:3869a7c3 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…

## Code
- sym:32923 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:32922 Repository.save_order (tallyho/repository.py:54-55)
- sym:32925 Repository.orders (tallyho/repository.py:72-74)
- sym:32924 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:32915 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:32926 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:32862 Order.add (tallyho/models.py:76-84)
- sym:32898 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:32860 Order (tallyho/models.py:64-109)
- sym:32866 Order.to_dict (tallyho/models.py:98-109)
- sym:32889 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:32901 quote_order (tallyho/pricing/quote.py:30-53)

---
tokens: 695/800
omitted 9 lower-ranked items (expand an id for detail):
- rec:8da1e50f attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:1fbd0bc9 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:793c309b decision: Event payloads carry money as integer minor units plus the currency co…
- rec:16f7df5a decision: Errors about customer data must not echo the rejected value: they are…
- rec:6a019ba5 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:fe093381 intent: Earlier work on this code
- … and 3 more
