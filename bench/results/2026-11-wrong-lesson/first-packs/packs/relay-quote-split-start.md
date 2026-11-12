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
- rec:17db4722 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the n parts. Money.allocate(n) hands out uneven parts, so payers would be charged different amounts; do not use it for splits.
  anchors: tallyho/money.py:Money.allocate
- rec:48b11c56 decision: Errors about customer data must not echo the rejected value: they are…
- rec:30720c9c decision: A refund is booked as the exact reverse of the sale: the same three ac…

## Code
- sym:42475 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:42505 Money.allocate (tallyho/money.py:63-68)
- sym:42476 Ledger.post (tallyho/ledger.py:28-35)
- sym:42522 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:42464 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:42488 Order (tallyho/models.py:64-109)
- sym:42497 Money (tallyho/money.py:22-83)
- sym:42467 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:42481 Product (tallyho/models.py:23-35)
- sym:42485 OrderLine (tallyho/models.py:51-60)
- sym:42521 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:42527 Quote (tallyho/pricing/quote.py:16-27)
- sym:42468 NotFound (tallyho/errors.py:24-25)

---
tokens: 707/800
omitted 37 lower-ranked items (expand an id for detail):
- rec:98107db0 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:22ede6cc decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- sym:42487 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:42478 Ledger.balances (tallyho/ledger.py:46-50)
- sym:42528 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:42463 TallyError (tallyho/errors.py:4-5)
- sym:42482 Product.__post_init__ (tallyho/models.py:30-35)
- … and 30 more
