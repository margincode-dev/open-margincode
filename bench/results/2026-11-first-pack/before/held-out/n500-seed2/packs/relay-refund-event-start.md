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
- rec:a3da4394 attempt: Tried calling `ValidationError` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/errors.py:ValidationError
- rec:8146936b attempt: Adding type: ignore comments around `Order` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/models.py:Order
- rec:4b36e1ee decision: Tests for `Order` build their inputs with small factory helpers in conftest, not shared module-level objects.
  anchors: tallyho/models.py:Order
- rec:e496c2dd decision: Log lines from `ValidationError` go through the logging module, never…

## Code
- sym:14640 Order (tallyho/models.py:64-109)
  class Order:
- sym:14746 Workflow._move (tallyho/workflow.py:29-33)
- sym:14748 Workflow.pay (tallyho/workflow.py:41-44)
- sym:14747 Workflow.place (tallyho/workflow.py:35-39)
- sym:14643 Order.remove (tallyho/models.py:86-89)
- sym:14616 ValidationError (tallyho/errors.py:8-17)
- sym:14744 Workflow (tallyho/workflow.py:23-50)
- sym:14743 _ALLOWED (tallyho/workflow.py:14-20)
- sym:14632 OrderStatus (tallyho/models.py:14-19)
- sym:14615 TallyError (tallyho/errors.py:4-5)
- sym:14628 Ledger.post (tallyho/ledger.py:28-35)
- sym:14634 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 713/800
omitted 198 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:acd747d5 decision: Constants that `ValidationError` depends on live at the top of its mod…
- rec:31af1a89 decision: Type hints on `Order` use the built-in generics (list[str], dict[str,…
- rec:ec723395 decision: Log lines from `Order` go through the logging module, never print(), s…
- rec:5ab95dbe decision: `ValidationError` keeps its docstring as the one place its behaviour i…
- rec:05f68fe6 note: Benchmarks for `Order` are in a separate, manual job; the unit tests m…
- … and 192 more
