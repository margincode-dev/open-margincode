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
- rec:77517f54 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:d84fd9de attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:2df1c044 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:17493 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:17496 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:17491 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:17612 Workflow.pay (tallyho/workflow.py:41-44)
- sym:17492 Ledger.post (tallyho/ledger.py:28-35)
- sym:17545 quote_order (tallyho/pricing/quote.py:30-53)
- sym:17543 Quote (tallyho/pricing/quote.py:16-27)
- sym:17504 Order (tallyho/models.py:64-109)
- sym:17494 Ledger.balances (tallyho/ledger.py:46-50)
- sym:17490 Entry (tallyho/ledger.py:15-20)
- sym:17484 NotFound (tallyho/errors.py:24-25)
- sym:17608 Workflow (tallyho/workflow.py:23-50)
- sym:17607 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 719/800
omitted 36 lower-ranked items (expand an id for detail):
- rec:b58933e5 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:53aee294 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- sym:17505 Order.currency (tallyho/models.py:73-74)
- sym:17513 Money (tallyho/money.py:22-83)
- sym:17515 Money.zero (tallyho/money.py:36-37)
- sym:17526 total (tallyho/money.py:86-91)
- sym:17548 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:17478 load_config (tallyho/config.py:23-47)
- … and 28 more
