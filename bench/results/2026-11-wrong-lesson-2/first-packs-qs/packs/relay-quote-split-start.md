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
- rec:811dbb21 decision: Errors about customer data must not echo the rejected value: they are…
- rec:95f7892e decision: A refund is booked as the exact reverse of the sale: the same three ac…

## Code
- sym:19 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:49 Money.allocate (tallyho/money.py:63-68)
- sym:20 Ledger.post (tallyho/ledger.py:28-35)
- sym:66 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:8 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:32 Order (tallyho/models.py:64-109)
- sym:41 Money (tallyho/money.py:22-83)
- sym:11 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:25 Product (tallyho/models.py:23-35)
- sym:29 OrderLine (tallyho/models.py:51-60)
- sym:65 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:71 Quote (tallyho/pricing/quote.py:16-27)
- sym:31 OrderLine.subtotal (tallyho/models.py:59-60)

---
tokens: 717/800
omitted 37 lower-ranked items (expand an id for detail):
- rec:1bb33865 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:5995cd42 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- sym:22 Ledger.balances (tallyho/ledger.py:46-50)
- sym:72 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:7 TallyError (tallyho/errors.py:4-5)
- sym:26 Product.__post_init__ (tallyho/models.py:30-35)
- sym:34 Order.add (tallyho/models.py:76-84)
- sym:35 Order.remove (tallyho/models.py:86-89)
- … and 29 more
