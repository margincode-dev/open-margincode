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
- rec:7db0e509 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:805f8c83 note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:d22e6ec4 decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:81aa4a27 decision: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:34011 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:34130 Workflow.pay (tallyho/workflow.py:41-44)
- sym:34009 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:34063 quote_order (tallyho/pricing/quote.py:30-53)
- sym:34061 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:34014 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:34126 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:34091 RecordStore.get (tallyho/store/base.py:28-29)
- sym:34086 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:34002 NotFound (tallyho/errors.py:24-25)
- sym:34125 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {

---
tokens: 710/800
omitted 74 lower-ranked items (expand an id for detail):
- rec:9fd25c07 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:3493d7d8 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:114006dc attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:12adbb34 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:2a154921 decision: The `ledger` module keeps its docstring as the one place its behaviour…
- rec:b459394f attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- … and 68 more
