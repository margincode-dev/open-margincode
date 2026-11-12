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
- rec:69973444 attempt: Caching what `Ledger.record_sale` returns in a module-level dict broke…
- rec:154780dd attempt: Moving the imports inside `Ledger.record_sale` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:c7209d59 decision: `Quote` keeps its docstring as the one place its behaviour is describe…

## Code
- sym:10313 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:10365 quote_order (tallyho/pricing/quote.py:30-53)
- sym:10311 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:10432 Workflow.pay (tallyho/workflow.py:41-44)
- sym:10363 Quote (tallyho/pricing/quote.py:16-27)
- sym:10316 OrderStatus (tallyho/models.py:14-19)
- sym:10393 RecordStore.get (tallyho/store/base.py:28-29)
- sym:10428 Workflow (tallyho/workflow.py:23-50)
- sym:10388 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:10304 NotFound (tallyho/errors.py:24-25)
- sym:10379 Repository (tallyho/repository.py:32-74)
- sym:10389 Repository.orders (tallyho/repository.py:72-74)

---
tokens: 716/800
omitted 202 lower-ranked items (expand an id for detail):
- rec:258d67f4 attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:d9fd6af8 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:03130aca decision: New optional parameters of `Ledger.record_sale` are keyword-only (afte…
- rec:916a6e2e decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- rec:eda21718 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:24aa0a10 decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- … and 196 more
