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
- rec:0be387d3 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:e7afb175 decision: Errors about customer data must not echo the rejected value: they are…
- rec:06ae0e77 intent: Earlier work on this code

## Code
- sym:33209 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:33201 Money (tallyho/money.py:22-83)
  class Money:
- sym:33231 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:33232 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:33226 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:33168 ValidationError (tallyho/errors.py:8-17)
- sym:33192 Order (tallyho/models.py:64-109)
  class Order:
- sym:33204 Money._check (tallyho/money.py:39-43)
- sym:33179 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:33225 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:33206 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 718/800
omitted 45 lower-ranked items (expand an id for detail):
- rec:34f00d87 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:b640464d attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:5a5ab02e decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:6b5ba18a attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:1758755c decision: Event payloads carry money as integer minor units plus the currency co…
- rec:20944dc4 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- … and 39 more
