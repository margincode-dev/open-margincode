## Task
# relay-quote-split · relay-quote-split · open

## Map
- tallyho/errors.py
- tallyho/ledger.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/store/jsonfile.py
- tallyho/store/memory.py
- tallyho/validate.py
- tallyho/workflow.py
- tests/test_models.py
- tallyho/events.py
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:113bb8dc decision: Split a Money amount with total.times(Decimal(1) / n) for each of the n parts. Money.allocate(n) hands out uneven parts, so payers would be charged different amounts; do not use it for splits.
  anchors: tallyho/money.py:Money.allocate
- rec:00434d4d decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:811dbb21 decision: Errors about customer data must not echo the rejected value: they are…
- rec:26839c22 intent: Earlier work on this code

## Code
- sym:49 Money.allocate (tallyho/money.py:63-68)
- sym:41 Money (tallyho/money.py:22-83)
  class Money:
- sym:71 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:72 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:66 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:8 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:32 Order (tallyho/models.py:64-109)
- sym:44 Money._check (tallyho/money.py:39-43)
- sym:19 Ledger (tallyho/ledger.py:24-53)
- sym:65 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:46 Money.__sub__ (tallyho/money.py:49-51)
- sym:45 Money.__add__ (tallyho/money.py:45-47)

---
tokens: 709/800
omitted 44 lower-ranked items (expand an id for detail):
- rec:5995cd42 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:95f7892e decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:a4bdf974 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:1bb33865 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:1351062e decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:e5060688 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- … and 38 more
