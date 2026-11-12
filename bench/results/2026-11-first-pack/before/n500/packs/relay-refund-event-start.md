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
- rec:0998b110 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:5aa76a2c attempt: Caching what `Order` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/models.py:Order
- rec:37ac80c4 decision: Type hints on `Order` use the built-in generics (list[str], dict[str, int]), not the typing aliases.
  anchors: tallyho/models.py:Order

## Code
- sym:9660 Order (tallyho/models.py:64-109)
  class Order:
- sym:9663 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:9652 OrderStatus (tallyho/models.py:14-19)
- sym:9662 Order.add (tallyho/models.py:76-84)
- sym:9766 Workflow._move (tallyho/workflow.py:29-33)
- sym:9725 Repository.orders (tallyho/repository.py:72-74)
- sym:9768 Workflow.pay (tallyho/workflow.py:41-44)
- sym:9767 Workflow.place (tallyho/workflow.py:35-39)
- sym:9724 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:9709 orders_to_csv (tallyho/report/csv_export.py:26-32)
- sym:9636 ValidationError (tallyho/errors.py:8-17)
- sym:9764 Workflow (tallyho/workflow.py:23-50)

---
tokens: 730/800
omitted 203 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:950c269e decision: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:fbbf85d5 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:d0ce3946 attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- rec:2f4b1e42 attempt: Adding type: ignore comments around `Repository.save_order` to quiet t…
- rec:f3c3ff45 attempt: Replacing the loop in the `models` module with a comprehension made it…
- rec:e529e5ea attempt: Replacing the loop in `Repository.save_order` with a comprehension mad…
- … and 196 more
