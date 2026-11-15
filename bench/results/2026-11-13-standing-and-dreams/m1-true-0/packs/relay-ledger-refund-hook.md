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
- rec:68d747a0 decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:96ec21f0 attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:2ac39169 intent, open: Earlier work on this code

## Code
- sym:2013 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:2132 Workflow.pay (tallyho/workflow.py:41-44)
- sym:2011 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:2065 quote_order (tallyho/pricing/quote.py:30-53)
- sym:2063 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:2016 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:2128 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:2093 RecordStore.get (tallyho/store/base.py:28-29)
- sym:2088 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:2004 NotFound (tallyho/errors.py:24-25)
- sym:2127 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 722/800
omitted 47 lower-ranked items (expand an id for detail):
- rec:af3df97c attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:93118adb attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:8440076c attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:36e2676d attempt, open: list_page never returns more than max_page_size (100) records, whateve…
- rec:cadf2a8c decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- … and 42 more
