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
- rec:e5032707 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:2b6127ac attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:daf5e02b attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:6856cdec attempt: list_page never returns more than max_page_size (100) records, whateve…

## Code
- sym:5333 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:5452 Workflow.pay (tallyho/workflow.py:41-44)
- sym:5331 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:5385 quote_order (tallyho/pricing/quote.py:30-53)
- sym:5383 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:5336 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:5448 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:5413 RecordStore.get (tallyho/store/base.py:28-29)
- sym:5408 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:5324 NotFound (tallyho/errors.py:24-25)
- sym:5447 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {

---
tokens: 724/800
omitted 45 lower-ranked items (expand an id for detail):
- rec:d64ab117 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:175a3a88 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:a8fdead4 decision: Errors about customer data must not echo the rejected value: they are…
- rec:e0fab675 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:b890a6c9 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- sym:5399 Repository (tallyho/repository.py:32-74)
- sym:5409 Repository.orders (tallyho/repository.py:72-74)
- … and 38 more
