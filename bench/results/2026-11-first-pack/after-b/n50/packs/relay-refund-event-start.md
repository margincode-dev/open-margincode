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
- rec:bfdffb24 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:e8e23444 note: `Workflow._move` is on the hot path of the nightly batch job; profile before adding work to it.
  anchors: tallyho/workflow.py:Workflow._move
- rec:496f059b attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:34a86601 decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:35018 Order (tallyho/models.py:64-109)
- sym:34994 ValidationError (tallyho/errors.py:8-17)
- sym:35010 OrderStatus (tallyho/models.py:14-19)
- sym:35124 Workflow._move (tallyho/workflow.py:29-33)
- sym:35126 Workflow.pay (tallyho/workflow.py:41-44)
- sym:35125 Workflow.place (tallyho/workflow.py:35-39)
- sym:35012 Product.__post_init__ (tallyho/models.py:30-35)
- sym:35021 Order.remove (tallyho/models.py:86-89)
- sym:35122 Workflow (tallyho/workflow.py:23-50)
- sym:35121 _ALLOWED (tallyho/workflow.py:14-20)
- sym:34998 NotFound (tallyho/errors.py:24-25)
- sym:35011 Product (tallyho/models.py:23-35)

---
tokens: 716/800
omitted 54 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:0f6d5669 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:a2832b4f attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:3a331a5a attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:22d37683 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:b05e0c4f attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- … and 48 more
