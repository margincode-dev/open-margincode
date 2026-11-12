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
- rec:baa4b3e0 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:cb4f52c9 decision: Errors about customer data must not echo the rejected value: they are…
- rec:7ced3775 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:d1e80b96 intent: Earlier work on this code

## Code
- sym:18500 Order (tallyho/models.py:64-109)
  class Order:
- sym:18606 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:18608 Workflow.pay (tallyho/workflow.py:41-44)
  def pay(self, order: Order, on: date) -> None:
- sym:18607 Workflow.place (tallyho/workflow.py:35-39)
  def place(self, order: Order, on: date) -> None:
- sym:18604 Workflow (tallyho/workflow.py:23-50)
- sym:18603 _ALLOWED (tallyho/workflow.py:14-20)
- sym:18610 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:18502 Order.add (tallyho/models.py:76-84)
- sym:18503 Order.remove (tallyho/models.py:86-89)
- sym:18476 ValidationError (tallyho/errors.py:8-17)
- sym:18492 OrderStatus (tallyho/models.py:14-19)
- sym:18475 TallyError (tallyho/errors.py:4-5)

---
tokens: 717/800
omitted 40 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:b2d98834 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:e88aa325 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:1c9095f0 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:9e8beabf attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:fe6a0092 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- … and 34 more
