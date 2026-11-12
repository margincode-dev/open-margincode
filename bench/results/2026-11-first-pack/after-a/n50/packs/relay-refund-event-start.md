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
- rec:39a062e7 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:2b844499 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:ef22c9a9 attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:a2d30f95 decision: New optional parameters of `Repository.save_order` are keyword-only (a…

## Code
- sym:20160 Order (tallyho/models.py:64-109)
  class Order:
- sym:20136 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:20152 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:20266 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:20268 Workflow.pay (tallyho/workflow.py:41-44)
- sym:20267 Workflow.place (tallyho/workflow.py:35-39)
- sym:20154 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:20163 Order.remove (tallyho/models.py:86-89)
- sym:20264 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:20263 _ALLOWED (tallyho/workflow.py:14-20)
- sym:20140 NotFound (tallyho/errors.py:24-25)
- sym:20153 Product (tallyho/models.py:23-35)

---
tokens: 720/800
omitted 54 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:4e66b588 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:85a5e0ae attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:c84056f7 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:3d4515af attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:37dad4b1 decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- … and 48 more
