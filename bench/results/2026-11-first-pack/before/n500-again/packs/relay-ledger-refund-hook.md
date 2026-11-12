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
- rec:c9c9575f attempt: Tried calling `Ledger.record_sale` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:69973444 attempt: Caching what `Ledger.record_sale` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:d9fd6af8 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:03130aca decision: New optional parameters of `Ledger.record_sale` are keyword-only (afte…

## Code
- sym:10313 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:10432 Workflow.pay (tallyho/workflow.py:41-44)
- sym:10311 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:10365 quote_order (tallyho/pricing/quote.py:30-53)
- sym:10363 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:10316 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:10428 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:10393 RecordStore.get (tallyho/store/base.py:28-29)
- sym:10388 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:10304 NotFound (tallyho/errors.py:24-25)
- sym:10427 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 717/800
omitted 203 lower-ranked items (expand an id for detail):
- rec:24aa0a10 decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:eda21718 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:1d65e7ae note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:154780dd attempt: Moving the imports inside `Ledger.record_sale` to speed up start-up sa…
- rec:258d67f4 attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:c7209d59 decision: `Quote` keeps its docstring as the one place its behaviour is describe…
- … and 197 more
