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
- rec:8f3f706c decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:a71c774a attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:b9502202 attempt: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:4b414695 decision: Type hints on `Order` use the built-in generics (list[str], dict[str,…

## Code
- sym:11320 Order (tallyho/models.py:64-109)
  class Order:
- sym:11426 Workflow._move (tallyho/workflow.py:29-33)
- sym:11428 Workflow.pay (tallyho/workflow.py:41-44)
- sym:11427 Workflow.place (tallyho/workflow.py:35-39)
- sym:11424 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:11423 _ALLOWED (tallyho/workflow.py:14-20)
- sym:11430 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:11322 Order.add (tallyho/models.py:76-84)
- sym:11323 Order.remove (tallyho/models.py:86-89)
- sym:11296 ValidationError (tallyho/errors.py:8-17)
- sym:11312 OrderStatus (tallyho/models.py:14-19)
- sym:11295 TallyError (tallyho/errors.py:4-5)

---
tokens: 709/800
omitted 202 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:92b53db0 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:4748c6ee attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:7a6f39cd decision: New optional parameters of `Workflow.cancel` are keyword-only (after *…
- rec:6df5e243 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:f218bd1d decision: New optional parameters of `Repository._order_from_record` are keyword…
- … and 196 more
