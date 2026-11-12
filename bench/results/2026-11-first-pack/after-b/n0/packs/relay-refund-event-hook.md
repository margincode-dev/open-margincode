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
- rec:a920a3d0 decision: Event payloads carry money as integer minor units plus the currency code: {"id": ..., "amount_minor": 4681, "currency": "USD"}. Never decimals or strings; consumers are in other languages.
  anchors: tallyho/workflow.py:Workflow
- rec:b685751c decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:063280a1 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:80aff875 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…

## Code
- sym:33358 Order (tallyho/models.py:64-109)
  class Order:
- sym:33464 Workflow._move (tallyho/workflow.py:29-33)
- sym:33466 Workflow.pay (tallyho/workflow.py:41-44)
- sym:33465 Workflow.place (tallyho/workflow.py:35-39)
- sym:33462 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:33461 _ALLOWED (tallyho/workflow.py:14-20)
- sym:33468 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:33360 Order.add (tallyho/models.py:76-84)
- sym:33361 Order.remove (tallyho/models.py:86-89)
- sym:33334 ValidationError (tallyho/errors.py:8-17)
- sym:33350 OrderStatus (tallyho/models.py:14-19)
- sym:33333 TallyError (tallyho/errors.py:4-5)

---
tokens: 720/800
omitted 42 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:1c7c2b33 decision: Errors about customer data must not echo the rejected value: they are…
- rec:2ffda034 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:68f8c7a9 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:070fd91a attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:91adda02 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- … and 36 more
