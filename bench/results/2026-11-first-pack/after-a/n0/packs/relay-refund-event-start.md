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
- rec:9e8beabf attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:baa4b3e0 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:cb4f52c9 decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:18500 Order (tallyho/models.py:64-109)
  class Order:
- sym:18606 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:18608 Workflow.pay (tallyho/workflow.py:41-44)
  def pay(self, order: Order, on: date) -> None:
- sym:18607 Workflow.place (tallyho/workflow.py:35-39)
- sym:18503 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:18476 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:18604 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:18603 _ALLOWED (tallyho/workflow.py:14-20)
- sym:18492 OrderStatus (tallyho/models.py:14-19)
- sym:18475 TallyError (tallyho/errors.py:4-5)
- sym:18488 Ledger.post (tallyho/ledger.py:28-35)
- sym:18494 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 707/800
omitted 36 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:fe6a0092 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:18502 Order.add (tallyho/models.py:76-84)
- sym:18508 _exponent (tallyho/money.py:14-18)
- sym:18517 Money.allocate (tallyho/money.py:63-68)
- sym:18524 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:18528 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- … and 29 more
