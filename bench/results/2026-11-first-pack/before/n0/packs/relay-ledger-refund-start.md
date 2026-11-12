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
- rec:e0fab675 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:5333 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:5336 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:5331 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:5452 Workflow.pay (tallyho/workflow.py:41-44)
- sym:5332 Ledger.post (tallyho/ledger.py:28-35)
- sym:5385 quote_order (tallyho/pricing/quote.py:30-53)
- sym:5383 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:5344 Order (tallyho/models.py:64-109)
- sym:5334 Ledger.balances (tallyho/ledger.py:46-50)
- sym:5330 Entry (tallyho/ledger.py:15-20)
- sym:5324 NotFound (tallyho/errors.py:24-25)
- sym:5448 Workflow (tallyho/workflow.py:23-50)
- sym:5447 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 723/800
omitted 36 lower-ranked items (expand an id for detail):
- rec:daf5e02b attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:b890a6c9 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- sym:5345 Order.currency (tallyho/models.py:73-74)
- sym:5353 Money (tallyho/money.py:22-83)
- sym:5355 Money.zero (tallyho/money.py:36-37)
- sym:5366 total (tallyho/money.py:86-91)
- sym:5388 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:5318 load_config (tallyho/config.py:23-47)
- … and 28 more
