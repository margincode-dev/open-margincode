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
- rec:d31aadff attempt: Moving the imports inside `Ledger.record_sale` to speed up start-up sa…
- rec:0747a9af decision: `Quote` keeps its docstring as the one place its behaviour is describe…

## Code
- sym:20813 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:20865 quote_order (tallyho/pricing/quote.py:30-53)
- sym:20811 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:20932 Workflow.pay (tallyho/workflow.py:41-44)
- sym:20863 Quote (tallyho/pricing/quote.py:16-27)
- sym:20816 OrderStatus (tallyho/models.py:14-19)
- sym:20893 RecordStore.get (tallyho/store/base.py:28-29)
- sym:20928 Workflow (tallyho/workflow.py:23-50)
- sym:20888 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:20804 NotFound (tallyho/errors.py:24-25)
- sym:20879 Repository (tallyho/repository.py:32-74)
- sym:20800 ValidationError (tallyho/errors.py:8-17)

---
tokens: 713/800
omitted 202 lower-ranked items (expand an id for detail):
- rec:dc615cc1 attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:fd957eff decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:c30ab739 decision: New optional parameters of `Ledger.record_sale` are keyword-only (afte…
- rec:414d00e7 decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- rec:b1c72d45 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:f298f02d decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- … and 196 more
