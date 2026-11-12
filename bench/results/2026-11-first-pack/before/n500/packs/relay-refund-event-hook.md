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
- rec:950c269e decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:0998b110 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:5aa76a2c attempt: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:7781bdcb decision: New optional parameters of `Workflow.cancel` are keyword-only (after *…

## Code
- sym:9660 Order (tallyho/models.py:64-109)
  class Order:
- sym:9766 Workflow._move (tallyho/workflow.py:29-33)
- sym:9768 Workflow.pay (tallyho/workflow.py:41-44)
- sym:9767 Workflow.place (tallyho/workflow.py:35-39)
- sym:9764 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:9763 _ALLOWED (tallyho/workflow.py:14-20)
- sym:9770 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:9662 Order.add (tallyho/models.py:76-84)
- sym:9663 Order.remove (tallyho/models.py:86-89)
- sym:9636 ValidationError (tallyho/errors.py:8-17)
- sym:9652 OrderStatus (tallyho/models.py:14-19)
- sym:9635 TallyError (tallyho/errors.py:4-5)

---
tokens: 710/800
omitted 202 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:37ac80c4 decision: Type hints on `Order` use the built-in generics (list[str], dict[str,…
- rec:fbbf85d5 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:88d6cca7 attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:70df13e4 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:97021a48 decision: New optional parameters of `Repository._order_from_record` are keyword…
- … and 196 more
