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
- rec:b431ea1d note: Deprecation warnings from `ValidationError.__init__` are turned into errors in CI (filterwarnings = error), so fix them when they appear.
  anchors: tallyho/errors.py:ValidationError.__init__
- rec:76ff8925 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:aebaac40 attempt: Adding type: ignore comments around `Order` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/models.py:Order
- rec:e9f1ea17 attempt: Moving the imports inside `ValidationError` to speed up start-up saved…

## Code
- sym:41658 Order (tallyho/models.py:64-109)
  class Order:
- sym:41661 Order.remove (tallyho/models.py:86-89)
- sym:41650 OrderStatus (tallyho/models.py:14-19)
- sym:41660 Order.add (tallyho/models.py:76-84)
- sym:41764 Workflow._move (tallyho/workflow.py:29-33)
- sym:41723 Repository.orders (tallyho/repository.py:72-74)
- sym:41766 Workflow.pay (tallyho/workflow.py:41-44)
- sym:41765 Workflow.place (tallyho/workflow.py:35-39)
- sym:41722 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:41707 orders_to_csv (tallyho/report/csv_export.py:26-32)
- sym:41634 ValidationError (tallyho/errors.py:8-17)
- sym:41762 Workflow (tallyho/workflow.py:23-50)
  class Workflow:

---
tokens: 709/800
omitted 204 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:070afe43 attempt: Adding type: ignore comments around `ValidationError` to quiet the che…
- rec:94655765 attempt: Moving the imports inside `Order` to speed up start-up saved under a m…
- rec:ee5987c9 attempt: Wrapping `Order` in a bare try/except with a retry loop hid the real e…
- rec:7b089841 attempt: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:923700a6 decision: Log lines from `Order` go through the logging module, never print(), s…
- … and 198 more
