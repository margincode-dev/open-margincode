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
- rec:d56a6198 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:bd0e3b52 note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:e1e81051 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid…

## Code
- sym:36678 Order (tallyho/models.py:64-109)
  class Order:
- sym:36681 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:36670 OrderStatus (tallyho/models.py:14-19)
- sym:36680 Order.add (tallyho/models.py:76-84)
- sym:36784 Workflow._move (tallyho/workflow.py:29-33)
- sym:36743 Repository.orders (tallyho/repository.py:72-74)
- sym:36786 Workflow.pay (tallyho/workflow.py:41-44)
- sym:36785 Workflow.place (tallyho/workflow.py:35-39)
- sym:36742 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:36727 orders_to_csv (tallyho/report/csv_export.py:26-32)
- sym:36654 ValidationError (tallyho/errors.py:8-17)
- sym:36782 Workflow (tallyho/workflow.py:23-50)
- sym:36658 NotFound (tallyho/errors.py:24-25)

---
tokens: 730/800
omitted 202 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:fea44599 attempt: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:bebcc601 decision: Type hints on `Order` use the built-in generics (list[str], dict[str,…
- rec:2bb32981 decision: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:3964d858 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:0f2687be attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- rec:4ec78e6d attempt: Adding type: ignore comments around `Repository.save_order` to quiet t…
- … and 195 more
