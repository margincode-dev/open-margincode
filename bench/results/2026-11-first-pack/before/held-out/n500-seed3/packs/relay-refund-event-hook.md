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
- rec:00fd36d8 attempt: Wrapping `Order` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/models.py:Order
- rec:8492640f attempt: Adding type: ignore comments around `Order` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/models.py:Order
- rec:c05f1b0c attempt: Moving the imports inside `ValidationError` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/errors.py:ValidationError
- rec:c99ae554 decision: Log lines from `require_positive` go through the logging module, never…

## Code
- sym:16300 Order (tallyho/models.py:64-109)
- sym:16406 Workflow._move (tallyho/workflow.py:29-33)
- sym:16408 Workflow.pay (tallyho/workflow.py:41-44)
- sym:16407 Workflow.place (tallyho/workflow.py:35-39)
- sym:16404 Workflow (tallyho/workflow.py:23-50)
- sym:16403 _ALLOWED (tallyho/workflow.py:14-20)
- sym:16410 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:16302 Order.add (tallyho/models.py:76-84)
- sym:16303 Order.remove (tallyho/models.py:86-89)
- sym:16276 ValidationError (tallyho/errors.py:8-17)
- sym:16292 OrderStatus (tallyho/models.py:14-19)
- sym:16275 TallyError (tallyho/errors.py:4-5)

---
tokens: 712/800
omitted 205 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:a75ca1f2 attempt: Adding type: ignore comments around `ValidationError` to quiet the che…
- rec:871d03ad attempt: Moving the imports inside `Order` to speed up start-up saved under a m…
- rec:a7e5f526 attempt: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:e68d9a80 decision: Log lines from `Order` go through the logging module, never print(), s…
- rec:f7a1908a decision: `Order` keeps its docstring as the one place its behaviour is describe…
- … and 199 more
