## Task
# relay-ledger-refund · relay-ledger-refund · open

## Map
- tallyho/ledger.py
- tallyho/pricing/quote.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/tax.py
- tallyho/workflow.py
- tallyho/config.py
- tallyho/errors.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
- rec:868f5d63 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:67d10eb1 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:4f463894 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:ae96e8d8 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…

## Code
- sym:32351 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:32470 Workflow.pay (tallyho/workflow.py:41-44)
  def pay(self, order: Order, on: date) -> None:
- sym:32349 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:32403 quote_order (tallyho/pricing/quote.py:30-53)
- sym:32401 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:32354 OrderStatus (tallyho/models.py:14-19)
- sym:32466 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:32431 RecordStore.get (tallyho/store/base.py:28-29)
- sym:32426 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:32342 NotFound (tallyho/errors.py:24-25)
- sym:32465 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 700/800
omitted 47 lower-ranked items (expand an id for detail):
- rec:1a66b2ab attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:9b2a7544 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:f2986979 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:3f98d68f decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:05eb78a4 decision: Errors about customer data must not echo the rejected value: they are…
- … and 42 more
