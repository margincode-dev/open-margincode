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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:caa756f5 decision, open: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:96ec21f0 attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:93118adb attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:2013 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:2132 Workflow.pay (tallyho/workflow.py:41-44)
- sym:2065 quote_order (tallyho/pricing/quote.py:30-53)
- sym:2063 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:2016 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:2128 Workflow (tallyho/workflow.py:23-50)
- sym:2011 Ledger (tallyho/ledger.py:24-53)
- sym:2127 _ALLOWED (tallyho/workflow.py:14-20)
- sym:2012 Ledger.post (tallyho/ledger.py:28-35)
- sym:2025 Order.currency (tallyho/models.py:73-74)
- sym:2033 Money (tallyho/money.py:22-83)
- sym:2035 Money.zero (tallyho/money.py:36-37)

---
tokens: 716/800
omitted 36 lower-ranked items (expand an id for detail):
- rec:af3df97c attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:8440076c attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- sym:2046 total (tallyho/money.py:86-91)
- sym:2068 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:1998 load_config (tallyho/config.py:23-47)
- sym:2000 ValidationError (tallyho/errors.py:8-17)
- sym:2010 Entry (tallyho/ledger.py:15-20)
- … and 29 more
