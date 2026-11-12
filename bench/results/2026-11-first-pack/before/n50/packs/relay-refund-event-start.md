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
- rec:a3a46b47 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:2042bff2 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:39f77be3 attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:58ffb1e5 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…

## Code
- sym:8000 Order (tallyho/models.py:64-109)
  class Order:
- sym:7976 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:7992 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:8106 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:8108 Workflow.pay (tallyho/workflow.py:41-44)
- sym:8107 Workflow.place (tallyho/workflow.py:35-39)
- sym:7994 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:8003 Order.remove (tallyho/models.py:86-89)
- sym:8104 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:8103 _ALLOWED (tallyho/workflow.py:14-20)
- sym:7980 NotFound (tallyho/errors.py:24-25)
- sym:7993 Product (tallyho/models.py:23-35)

---
tokens: 714/800
omitted 54 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:f7747ebc attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:c1e26efa attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:0683cbdc attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:626bda0d decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:717ade97 decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- … and 48 more
