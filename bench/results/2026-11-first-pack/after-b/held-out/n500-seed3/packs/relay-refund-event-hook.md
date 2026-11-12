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
- rec:245da62b decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:6153cec3 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:ee5987c9 attempt: Wrapping `Order` in a bare try/except with a retry loop hid the real e…
- rec:aebaac40 attempt: Adding type: ignore comments around `Order` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/models.py:Order

## Code
- sym:41658 Order (tallyho/models.py:64-109)
- sym:41764 Workflow._move (tallyho/workflow.py:29-33)
- sym:41766 Workflow.pay (tallyho/workflow.py:41-44)
- sym:41765 Workflow.place (tallyho/workflow.py:35-39)
- sym:41762 Workflow (tallyho/workflow.py:23-50)
- sym:41761 _ALLOWED (tallyho/workflow.py:14-20)
- sym:41768 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:41660 Order.add (tallyho/models.py:76-84)
- sym:41661 Order.remove (tallyho/models.py:86-89)
- sym:41634 ValidationError (tallyho/errors.py:8-17)
- sym:41650 OrderStatus (tallyho/models.py:14-19)
- sym:41633 TallyError (tallyho/errors.py:4-5)

---
tokens: 729/800
omitted 217 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:e9f1ea17 attempt: Moving the imports inside `ValidationError` to speed up start-up saved…
- rec:070afe43 attempt: Adding type: ignore comments around `ValidationError` to quiet the che…
- rec:94655765 attempt: Moving the imports inside `Order` to speed up start-up saved under a m…
- rec:7b089841 attempt: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:923700a6 decision: Log lines from `Order` go through the logging module, never print(), s…
- rec:cf77b41f decision: `Order` keeps its docstring as the one place its behaviour is describe…
- … and 210 more
