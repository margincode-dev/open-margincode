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
- rec:4f463894 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:1a66b2ab attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:32351 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:32354 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:32349 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:32470 Workflow.pay (tallyho/workflow.py:41-44)
- sym:32350 Ledger.post (tallyho/ledger.py:28-35)
- sym:32403 quote_order (tallyho/pricing/quote.py:30-53)
- sym:32401 Quote (tallyho/pricing/quote.py:16-27)
- sym:32362 Order (tallyho/models.py:64-109)
- sym:32352 Ledger.balances (tallyho/ledger.py:46-50)
- sym:32348 Entry (tallyho/ledger.py:15-20)
- sym:32342 NotFound (tallyho/errors.py:24-25)
- sym:32466 Workflow (tallyho/workflow.py:23-50)
- sym:32465 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 707/800
omitted 36 lower-ranked items (expand an id for detail):
- rec:ae96e8d8 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:9b2a7544 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- sym:32363 Order.currency (tallyho/models.py:73-74)
- sym:32371 Money (tallyho/money.py:22-83)
- sym:32373 Money.zero (tallyho/money.py:36-37)
- sym:32384 total (tallyho/money.py:86-91)
- sym:32406 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- … and 29 more
