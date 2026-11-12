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
- rec:5857010d attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:e7afb175 decision: Errors about customer data must not echo the rejected value: they are…
- rec:34f00d87 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:33209 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:33226 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:33168 ValidationError (tallyho/errors.py:8-17)
- sym:33201 Money (tallyho/money.py:22-83)
  class Money:
- sym:33179 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:33225 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:33231 Quote (tallyho/pricing/quote.py:16-27)
- sym:33232 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:33167 TallyError (tallyho/errors.py:4-5)
- sym:33180 Ledger.post (tallyho/ledger.py:28-35)
- sym:33186 Product.__post_init__ (tallyho/models.py:30-35)
- sym:33194 Order.add (tallyho/models.py:76-84)

---
tokens: 715/800
omitted 36 lower-ranked items (expand an id for detail):
- rec:5a5ab02e decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:20944dc4 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:33195 Order.remove (tallyho/models.py:86-89)
- sym:33200 _exponent (tallyho/money.py:14-18)
- sym:33216 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:33267 JsonFileStore._path (tallyho/store/jsonfile.py:20-23)
- sym:33270 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- … and 29 more
