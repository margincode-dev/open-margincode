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
- rec:bd4fc876 note: Deprecation warnings from `ValidationError.__init__` are turned into errors in CI (filterwarnings = error), so fix them when they appear.
  anchors: tallyho/errors.py:ValidationError.__init__
- rec:759a6773 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:0c4d2be7 attempt: Tried calling `ValidationError` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/errors.py:ValidationError

## Code
- sym:39998 Order (tallyho/models.py:64-109)
  class Order:
- sym:40001 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:39990 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:40000 Order.add (tallyho/models.py:76-84)
- sym:40104 Workflow._move (tallyho/workflow.py:29-33)
- sym:40063 Repository.orders (tallyho/repository.py:72-74)
- sym:40106 Workflow.pay (tallyho/workflow.py:41-44)
- sym:40105 Workflow.place (tallyho/workflow.py:35-39)
- sym:40062 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:40047 orders_to_csv (tallyho/report/csv_export.py:26-32)
- sym:39974 ValidationError (tallyho/errors.py:8-17)
- sym:40102 Workflow (tallyho/workflow.py:23-50)

---
tokens: 704/800
omitted 200 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:a7c16308 attempt: Adding type: ignore comments around `Order` to quiet the checker hid a…
- rec:3583b42e decision: Tests for `Order` build their inputs with small factory helpers in con…
- rec:d1910c83 decision: Log lines from `ValidationError` go through the logging module, never…
- rec:bf8e12f6 decision: Constants that `ValidationError` depends on live at the top of its mod…
- rec:415266f5 decision: Type hints on `Order` use the built-in generics (list[str], dict[str,…
- … and 194 more
