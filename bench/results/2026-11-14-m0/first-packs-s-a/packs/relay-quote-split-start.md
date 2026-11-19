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
- rec:b6f6c66c decision: Errors about customer data must not echo the rejected value: they are…
- rec:b6b5edea decision: Merge orders with self.lines.extend(other.lines) after the draft check…

## Code
- sym:215 Money.allocate (tallyho/money.py:63-68)
- sym:232 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:174 ValidationError (tallyho/errors.py:8-17)
- sym:207 Money (tallyho/money.py:22-83)
  class Money:
- sym:185 Ledger (tallyho/ledger.py:24-53)
- sym:231 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:237 Quote (tallyho/pricing/quote.py:16-27)
- sym:238 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:173 TallyError (tallyho/errors.py:4-5)
- sym:186 Ledger.post (tallyho/ledger.py:28-35)
- sym:192 Product.__post_init__ (tallyho/models.py:30-35)
- sym:200 Order.add (tallyho/models.py:76-84)
- sym:201 Order.remove (tallyho/models.py:86-89)

---
tokens: 717/800
omitted 36 lower-ranked items (expand an id for detail):
- rec:2348f3ca decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:ee60e1d3 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:9bdf44ce decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- sym:206 _exponent (tallyho/money.py:14-18)
- sym:222 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:273 JsonFileStore._path (tallyho/store/jsonfile.py:20-23)
- sym:276 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- … and 29 more
