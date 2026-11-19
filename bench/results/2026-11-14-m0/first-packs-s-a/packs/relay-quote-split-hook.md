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
- rec:30dd9768 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the n parts. Money.allocate(n) hands out uneven parts, so payers would be charged different amounts; do not use it for splits.
  anchors: tallyho/money.py:Money.allocate
- rec:82d61052 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:b6f6c66c decision: Errors about customer data must not echo the rejected value: they are…
- rec:f4aa03cf intent: Earlier work on this code

## Code
- sym:215 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:207 Money (tallyho/money.py:22-83)
- sym:237 Quote (tallyho/pricing/quote.py:16-27)
- sym:238 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:232 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:174 ValidationError (tallyho/errors.py:8-17)
- sym:198 Order (tallyho/models.py:64-109)
- sym:210 Money._check (tallyho/money.py:39-43)
- sym:185 Ledger (tallyho/ledger.py:24-53)
- sym:231 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:212 Money.__sub__ (tallyho/money.py:49-51)
- sym:211 Money.__add__ (tallyho/money.py:45-47)

---
tokens: 708/800
omitted 47 lower-ranked items (expand an id for detail):
- rec:9bdf44ce decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:b6b5edea decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:2348f3ca decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:16584885 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:ee60e1d3 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:c7cae52f decision: Count through the base class: len(self.list_page(collection, None, 10*…
- … and 41 more
