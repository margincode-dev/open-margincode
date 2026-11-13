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
- rec:2dc2a978 decision, open: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:242e33a5 attempt, open: Tried calling `Ledger.record_sale` twice to force a refresh; the secon…
- rec:28d1f4ad attempt, open: Caching what `Ledger.record_sale` returns in a module-level dict broke…

## Code
- sym:5333 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:5452 Workflow.pay (tallyho/workflow.py:41-44)
- sym:5331 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:5385 quote_order (tallyho/pricing/quote.py:30-53)
- sym:5383 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:5336 OrderStatus (tallyho/models.py:14-19)
- sym:5448 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:5413 RecordStore.get (tallyho/store/base.py:28-29)
- sym:5408 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:5324 NotFound (tallyho/errors.py:24-25)
- sym:5447 _ALLOWED (tallyho/workflow.py:14-20)
- sym:5399 Repository (tallyho/repository.py:32-74)

---
tokens: 707/800
omitted 216 lower-ranked items (expand an id for detail):
- rec:23fec861 attempt, open: Moving the imports inside `Ledger.record_sale` to speed up start-up sa…
- rec:14cd5de8 decision, open: New optional parameters of `Ledger.record_sale` are keyword-only (afte…
- rec:adb30db3 decision, open: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:982034e0 decision, open: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:50962e79 decision, open: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- … and 211 more
