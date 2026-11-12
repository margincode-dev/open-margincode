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
- rec:5673c904 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:a54aabe4 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:496f059b attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:a2832b4f attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…

## Code
- sym:35018 Order (tallyho/models.py:64-109)
  class Order:
- sym:35124 Workflow._move (tallyho/workflow.py:29-33)
- sym:35126 Workflow.pay (tallyho/workflow.py:41-44)
  def pay(self, order: Order, on: date) -> None:
- sym:35125 Workflow.place (tallyho/workflow.py:35-39)
- sym:35122 Workflow (tallyho/workflow.py:23-50)
- sym:35121 _ALLOWED (tallyho/workflow.py:14-20)
- sym:35128 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:35020 Order.add (tallyho/models.py:76-84)
- sym:35021 Order.remove (tallyho/models.py:86-89)
- sym:34994 ValidationError (tallyho/errors.py:8-17)
- sym:35010 OrderStatus (tallyho/models.py:14-19)
- sym:34993 TallyError (tallyho/errors.py:4-5)

---
tokens: 713/800
omitted 71 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:3a331a5a attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:bfdffb24 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:b9806fc2 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:34a86601 decision: Errors about customer data must not echo the rejected value: they are…
- rec:c92494b5 decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- … and 65 more
