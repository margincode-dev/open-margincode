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
- rec:34b3cfa4 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:a0011349 attempt: Caching what `Order` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/models.py:Order
- rec:b710daba decision: Type hints on `Order` use the built-in generics (list[str], dict[str, int]), not the typing aliases.
  anchors: tallyho/models.py:Order
- rec:17110361 decision: New optional parameters of `ValidationError` are keyword-only (after *…

## Code
- sym:21820 Order (tallyho/models.py:64-109)
- sym:21823 Order.remove (tallyho/models.py:86-89)
- sym:21812 OrderStatus (tallyho/models.py:14-19)
- sym:21822 Order.add (tallyho/models.py:76-84)
- sym:21926 Workflow._move (tallyho/workflow.py:29-33)
- sym:21885 Repository.orders (tallyho/repository.py:72-74)
- sym:21928 Workflow.pay (tallyho/workflow.py:41-44)
- sym:21927 Workflow.place (tallyho/workflow.py:35-39)
- sym:21884 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:21869 orders_to_csv (tallyho/report/csv_export.py:26-32)
- sym:21796 ValidationError (tallyho/errors.py:8-17)
- sym:21924 Workflow (tallyho/workflow.py:23-50)

---
tokens: 713/800
omitted 202 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:0a2a286d note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:4a7f6355 attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- rec:a6f4d6ef attempt: Adding type: ignore comments around `Repository.save_order` to quiet t…
- rec:7b537d2e attempt: Replacing the loop in the `models` module with a comprehension made it…
- rec:96e02fcc attempt: Replacing the loop in `Repository.save_order` with a comprehension mad…
- … and 196 more
