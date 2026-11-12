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
- rec:efb21a68 attempt: Tried calling `Ledger.record_sale` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:87b35545 attempt: Caching what `Ledger.record_sale` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:fd957eff decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:c30ab739 decision: New optional parameters of `Ledger.record_sale` are keyword-only (afte…

## Code
- sym:20813 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:20932 Workflow.pay (tallyho/workflow.py:41-44)
- sym:20811 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:20865 quote_order (tallyho/pricing/quote.py:30-53)
- sym:20863 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:20816 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:20928 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:20893 RecordStore.get (tallyho/store/base.py:28-29)
- sym:20888 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:20804 NotFound (tallyho/errors.py:24-25)
- sym:20927 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 714/800
omitted 203 lower-ranked items (expand an id for detail):
- rec:f298f02d decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:b1c72d45 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:baeee932 note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:d31aadff attempt: Moving the imports inside `Ledger.record_sale` to speed up start-up sa…
- rec:dc615cc1 attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:0747a9af decision: `Quote` keeps its docstring as the one place its behaviour is describe…
- … and 197 more
