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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:c0b06271 decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000002, ...): the store lists records in id order, so padded sequence ids keep listings chronological. No UUIDs.
  anchors: tallyho/repository.py:Repository
- rec:823dd042 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:c52d6e14 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:b2a6964e attempt, open: list_page never returns more than max_page_size (100) records, whateve…
- rec:3c46189f decision, open: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:2585 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:2584 Repository.save_order (tallyho/repository.py:54-55)
- sym:2587 Repository.orders (tallyho/repository.py:72-74)
- sym:2586 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:2577 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:2588 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:2524 Order.add (tallyho/models.py:76-84)
- sym:2560 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:2522 Order (tallyho/models.py:64-109)
  class Order:
- sym:2528 Order.to_dict (tallyho/models.py:98-109)
- sym:2551 CouponBook.get (tallyho/pricing/coupons.py:49-50)

---
tokens: 704/800
omitted 10 lower-ranked items (expand an id for detail):
- rec:c67ec1ab attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:9f0079b8 attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:7c549b09 attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:8ca52fd8 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:56a4a001 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- … and 5 more
