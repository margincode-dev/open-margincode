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
- rec:62875c83 decision, open: Event payloads carry money as integer minor units plus the currency code: {"id": ..., "amount_minor": 4681, "currency": "USD"}. Never decimals or strings; consumers are in other languages.
  anchors: tallyho/workflow.py:Workflow
- rec:70bb9001 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:5f715a4e attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…

## Code
- sym:3020 Order (tallyho/models.py:64-109)
  class Order:
- sym:3126 Workflow._move (tallyho/workflow.py:29-33)
- sym:3128 Workflow.pay (tallyho/workflow.py:41-44)
- sym:3127 Workflow.place (tallyho/workflow.py:35-39)
- sym:3124 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:3123 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:3130 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:3022 Order.add (tallyho/models.py:76-84)
- sym:3023 Order.remove (tallyho/models.py:86-89)
- sym:2996 ValidationError (tallyho/errors.py:8-17)
- sym:3012 OrderStatus (tallyho/models.py:14-19)
- sym:2995 TallyError (tallyho/errors.py:4-5)

---
tokens: 703/800
omitted 43 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:f37113dc attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:32781930 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:8ce3a5e8 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:be11a61b attempt, open: list_page never returns more than max_page_size (100) records, whateve…
- … and 38 more
