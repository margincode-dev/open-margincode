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
- rec:063280a1 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:1c7c2b33 decision: Errors about customer data must not echo the rejected value: they are…
- rec:80aff875 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…

## Code
- sym:33358 Order (tallyho/models.py:64-109)
  class Order:
- sym:33334 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:33350 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:33464 Workflow._move (tallyho/workflow.py:29-33)
- sym:33466 Workflow.pay (tallyho/workflow.py:41-44)
- sym:33465 Workflow.place (tallyho/workflow.py:35-39)
- sym:33352 Product.__post_init__ (tallyho/models.py:30-35)
- sym:33361 Order.remove (tallyho/models.py:86-89)
- sym:33462 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:33461 _ALLOWED (tallyho/workflow.py:14-20)
- sym:33338 NotFound (tallyho/errors.py:24-25)
- sym:33351 Product (tallyho/models.py:23-35)
- sym:33355 OrderLine (tallyho/models.py:51-60)

---
tokens: 725/800
omitted 39 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:ebb1487f decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:33356 OrderLine.__post_init__ (tallyho/models.py:55-56)
- sym:33333 TallyError (tallyho/errors.py:4-5)
- sym:33346 Ledger.post (tallyho/ledger.py:28-35)
- sym:33360 Order.add (tallyho/models.py:76-84)
- sym:33366 _exponent (tallyho/money.py:14-18)
- sym:33375 Money.allocate (tallyho/money.py:63-68)
- … and 31 more
