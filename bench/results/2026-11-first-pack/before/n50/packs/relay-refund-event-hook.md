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
- rec:39f77be3 attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:626bda0d decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:a15194f5 decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:8000 Order (tallyho/models.py:64-109)
  class Order:
- sym:8106 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:8108 Workflow.pay (tallyho/workflow.py:41-44)
  def pay(self, order: Order, on: date) -> None:
- sym:8107 Workflow.place (tallyho/workflow.py:35-39)
  def place(self, order: Order, on: date) -> None:
- sym:8104 Workflow (tallyho/workflow.py:23-50)
- sym:8103 _ALLOWED (tallyho/workflow.py:14-20)
- sym:8110 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:8002 Order.add (tallyho/models.py:76-84)
- sym:8003 Order.remove (tallyho/models.py:86-89)
- sym:7976 ValidationError (tallyho/errors.py:8-17)
- sym:7992 OrderStatus (tallyho/models.py:14-19)
- sym:7975 TallyError (tallyho/errors.py:4-5)

---
tokens: 707/800
omitted 52 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:717ade97 decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- rec:7b86822c decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:1094903f decision: Event payloads carry money as integer minor units plus the currency co…
- rec:2042bff2 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:f7747ebc attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- … and 46 more
