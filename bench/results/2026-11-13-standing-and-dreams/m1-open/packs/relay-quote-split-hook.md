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
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:e9290d33 decision, open: Split a Money amount with total.times(Decimal(1) / n) for each of the n parts. Money.allocate(n) hands out uneven parts, so payers would be charged different amounts; do not use it for splits.
  anchors: tallyho/money.py:Money.allocate
- rec:18ad5e43 decision, open: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:50f3b567 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:5b0475fe intent, open: Earlier work on this code

## Code
- sym:1377 Money.allocate (tallyho/money.py:63-68)
- sym:1369 Money (tallyho/money.py:22-83)
  class Money:
- sym:1399 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:1400 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:1394 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:1336 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:1360 Order (tallyho/models.py:64-109)
- sym:1372 Money._check (tallyho/money.py:39-43)
- sym:1347 Ledger (tallyho/ledger.py:24-53)
- sym:1393 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:1374 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 705/800
omitted 45 lower-ranked items (expand an id for detail):
- rec:0e812edd decision, open: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:3761f803 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:e422b8b7 decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:1aa40c3f decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:18ad6afe decision, open: Count through the base class: len(self.list_page(collection, None, 10*…
- … and 40 more
