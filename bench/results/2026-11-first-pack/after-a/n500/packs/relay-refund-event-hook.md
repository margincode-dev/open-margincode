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
- rec:17110361 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:34b3cfa4 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:a0011349 attempt: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:b710daba decision: Type hints on `Order` use the built-in generics (list[str], dict[str,…

## Code
- sym:21820 Order (tallyho/models.py:64-109)
  class Order:
- sym:21926 Workflow._move (tallyho/workflow.py:29-33)
- sym:21928 Workflow.pay (tallyho/workflow.py:41-44)
- sym:21927 Workflow.place (tallyho/workflow.py:35-39)
- sym:21924 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:21923 _ALLOWED (tallyho/workflow.py:14-20)
- sym:21930 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:21822 Order.add (tallyho/models.py:76-84)
- sym:21823 Order.remove (tallyho/models.py:86-89)
- sym:21796 ValidationError (tallyho/errors.py:8-17)
- sym:21812 OrderStatus (tallyho/models.py:14-19)
- sym:21795 TallyError (tallyho/errors.py:4-5)

---
tokens: 708/800
omitted 202 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:0a2a286d note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:1b19c184 attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:af5b1f87 decision: New optional parameters of `Workflow.cancel` are keyword-only (after *…
- rec:796b7287 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:a6b644d5 decision: New optional parameters of `Repository._order_from_record` are keyword…
- … and 196 more
