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
- rec:348f09b0 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:48b11c56 decision: Errors about customer data must not echo the rejected value: they are…
- rec:d22a4a25 intent: Earlier work on this code

## Code
- sym:42505 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:42497 Money (tallyho/money.py:22-83)
  class Money:
- sym:42527 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:42528 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:42522 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:42464 ValidationError (tallyho/errors.py:8-17)
- sym:42488 Order (tallyho/models.py:64-109)
- sym:42500 Money._check (tallyho/money.py:39-43)
- sym:42475 Ledger (tallyho/ledger.py:24-53)
- sym:42521 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:42502 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 713/800
omitted 45 lower-ranked items (expand an id for detail):
- rec:22ede6cc decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:30720c9c decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:9659b24b decision: Event payloads carry money as integer minor units plus the currency co…
- rec:98107db0 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:c34b79b4 decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:39f86b97 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- … and 39 more
