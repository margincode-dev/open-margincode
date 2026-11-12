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
- rec:4aaa1d7b attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:c16cb4d1 decision: Errors about customer data must not echo the rejected value: they are…
- rec:c513fa99 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:333d6234 decision: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:6340 Order (tallyho/models.py:64-109)
  class Order:
- sym:6446 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:6448 Workflow.pay (tallyho/workflow.py:41-44)
  def pay(self, order: Order, on: date) -> None:
- sym:6447 Workflow.place (tallyho/workflow.py:35-39)
- sym:6444 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:6443 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:6450 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:6342 Order.add (tallyho/models.py:76-84)
- sym:6343 Order.remove (tallyho/models.py:86-89)
- sym:6316 ValidationError (tallyho/errors.py:8-17)
- sym:6332 OrderStatus (tallyho/models.py:14-19)
- sym:6315 TallyError (tallyho/errors.py:4-5)

---
tokens: 724/800
omitted 40 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:053541fa decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:85e8549a decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:e2a97081 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:2b2bc44c decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:a03d5161 intent: Earlier work on this code
- sym:6328 Ledger.post (tallyho/ledger.py:28-35)
- … and 33 more
