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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:5f715a4e attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:32781930 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:f37113dc attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…

## Code
- sym:3020 Order (tallyho/models.py:64-109)
  class Order:
- sym:2996 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:3012 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:3126 Workflow._move (tallyho/workflow.py:29-33)
- sym:3128 Workflow.pay (tallyho/workflow.py:41-44)
- sym:3127 Workflow.place (tallyho/workflow.py:35-39)
- sym:3014 Product.__post_init__ (tallyho/models.py:30-35)
- sym:3023 Order.remove (tallyho/models.py:86-89)
- sym:3124 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:3123 _ALLOWED (tallyho/workflow.py:14-20)
- sym:3000 NotFound (tallyho/errors.py:24-25)
- sym:3013 Product (tallyho/models.py:23-35)

---
tokens: 717/800
omitted 40 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:198bf52d decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:3017 OrderLine (tallyho/models.py:51-60)
- sym:3018 OrderLine.__post_init__ (tallyho/models.py:55-56)
- sym:2995 TallyError (tallyho/errors.py:4-5)
- sym:3008 Ledger.post (tallyho/ledger.py:28-35)
- sym:3022 Order.add (tallyho/models.py:76-84)
- … and 33 more
