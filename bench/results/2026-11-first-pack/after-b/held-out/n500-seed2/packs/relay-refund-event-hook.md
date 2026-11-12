## Task
# relay-refund-event · relay-refund-event · open

## Map
- tallyho/errors.py
- tallyho/models.py
- tests/conftest.py
- tallyho/ledger.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/pricing/quote.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- tallyho/store/jsonfile.py
- tallyho/store/memory.py
- tallyho/validate.py
- tallyho/workflow.py
- tests/test_models.py
- tests/test_pricing.py
- tallyho/cli.py

## Decisions & dead ends
- rec:d7d3bbbc decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:a924ef54 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:0c4d2be7 attempt: Tried calling `ValidationError` twice to force a refresh; the second c…
- rec:cc23fc82 note: The repr of objects around `ValidationError` shows up in logs, so keep…

## Code
- sym:39998 Order (tallyho/models.py:64-109)
  class Order:
- sym:40104 Workflow._move (tallyho/workflow.py:29-33)
- sym:40106 Workflow.pay (tallyho/workflow.py:41-44)
- sym:40105 Workflow.place (tallyho/workflow.py:35-39)
- sym:40102 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:40101 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:40108 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:40000 Order.add (tallyho/models.py:76-84)
- sym:40001 Order.remove (tallyho/models.py:86-89)
- sym:39974 ValidationError (tallyho/errors.py:8-17)
- sym:39990 OrderStatus (tallyho/models.py:14-19)
- sym:39973 TallyError (tallyho/errors.py:4-5)

---
tokens: 730/800
omitted 211 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:a7c16308 attempt: Adding type: ignore comments around `Order` to quiet the checker hid a…
- rec:3583b42e decision: Tests for `Order` build their inputs with small factory helpers in con…
- rec:d1910c83 decision: Log lines from `ValidationError` go through the logging module, never…
- rec:bf8e12f6 decision: Constants that `ValidationError` depends on live at the top of its mod…
- rec:415266f5 decision: Type hints on `Order` use the built-in generics (list[str], dict[str,…
- rec:3f2121be decision: Log lines from `Order` go through the logging module, never print(), s…
- … and 204 more
