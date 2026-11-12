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
- rec:ef22c9a9 attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:a2d30f95 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:3e8e6694 decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:20160 Order (tallyho/models.py:64-109)
  class Order:
- sym:20266 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:20268 Workflow.pay (tallyho/workflow.py:41-44)
  def pay(self, order: Order, on: date) -> None:
- sym:20267 Workflow.place (tallyho/workflow.py:35-39)
- sym:20264 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:20263 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:20270 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:20162 Order.add (tallyho/models.py:76-84)
- sym:20163 Order.remove (tallyho/models.py:86-89)
- sym:20136 ValidationError (tallyho/errors.py:8-17)
- sym:20152 OrderStatus (tallyho/models.py:14-19)
- sym:20135 TallyError (tallyho/errors.py:4-5)

---
tokens: 706/800
omitted 52 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:37dad4b1 decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- rec:d08b0262 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:95dbf154 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:2b844499 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:4e66b588 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- … and 46 more
