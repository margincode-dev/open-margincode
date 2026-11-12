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
- rec:69d58886 attempt: Tried calling `Ledger.record_sale` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:e5d7dbe6 attempt: Caching what `Ledger.record_sale` returns in a module-level dict broke…
- rec:3140b6c2 attempt: Moving the imports inside `Ledger.record_sale` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:cbc82dd7 decision: `Quote` keeps its docstring as the one place its behaviour is describe…

## Code
- sym:8653 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:8705 quote_order (tallyho/pricing/quote.py:30-53)
- sym:8651 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:8772 Workflow.pay (tallyho/workflow.py:41-44)
- sym:8703 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:8656 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:8733 RecordStore.get (tallyho/store/base.py:28-29)
- sym:8768 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:8728 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:8644 NotFound (tallyho/errors.py:24-25)
- sym:8719 Repository (tallyho/repository.py:32-74)

---
tokens: 717/800
omitted 203 lower-ranked items (expand an id for detail):
- rec:2ff3d95d attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:58d950bb decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:86912096 decision: New optional parameters of `Ledger.record_sale` are keyword-only (afte…
- rec:a05c655e decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- rec:32ca61b6 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:e6567d43 decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- … and 197 more
