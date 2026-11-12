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
- rec:a71c774a attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:b9502202 attempt: Caching what `Order` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/models.py:Order
- rec:4b414695 decision: Type hints on `Order` use the built-in generics (list[str], dict[str, int]), not the typing aliases.
  anchors: tallyho/models.py:Order
- rec:8f3f706c decision: New optional parameters of `ValidationError` are keyword-only (after *…

## Code
- sym:11320 Order (tallyho/models.py:64-109)
- sym:11426 Workflow._move (tallyho/workflow.py:29-33)
- sym:11428 Workflow.pay (tallyho/workflow.py:41-44)
- sym:11427 Workflow.place (tallyho/workflow.py:35-39)
- sym:11323 Order.remove (tallyho/models.py:86-89)
- sym:11296 ValidationError (tallyho/errors.py:8-17)
- sym:11424 Workflow (tallyho/workflow.py:23-50)
- sym:11423 _ALLOWED (tallyho/workflow.py:14-20)
- sym:11312 OrderStatus (tallyho/models.py:14-19)
- sym:11295 TallyError (tallyho/errors.py:4-5)
- sym:11308 Ledger.post (tallyho/ledger.py:28-35)
- sym:11322 Order.add (tallyho/models.py:76-84)

---
tokens: 731/800
omitted 201 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:92b53db0 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:1ebcb30b attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- rec:fa8c6a06 attempt: Adding type: ignore comments around `Repository.save_order` to quiet t…
- rec:f163788c attempt: Replacing the loop in the `models` module with a comprehension made it…
- rec:4041ea04 attempt: Replacing the loop in `Repository.save_order` with a comprehension mad…
- rec:e84634fa attempt: Adding type: ignore comments around `Repository._order_from_record` to…
- … and 194 more
