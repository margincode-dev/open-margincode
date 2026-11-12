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
- rec:77f6737d attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:1fb7dc75 decision: Errors about customer data must not echo the rejected value: they are…
- rec:ce2cc79e attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:6191 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:6183 Money (tallyho/money.py:22-83)
  class Money:
- sym:6213 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:6214 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:6208 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:6150 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:6174 Order (tallyho/models.py:64-109)
- sym:6186 Money._check (tallyho/money.py:39-43)
- sym:6161 Ledger (tallyho/ledger.py:24-53)
- sym:6207 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:6188 Money.__sub__ (tallyho/money.py:49-51)
- sym:6187 Money.__add__ (tallyho/money.py:45-47)

---
tokens: 720/800
omitted 42 lower-ranked items (expand an id for detail):
- rec:4177b0cf attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:d32f0245 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:03ab6414 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:a2b148cd decision: Event payloads carry money as integer minor units plus the currency co…
- rec:3db793fa decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:6149 TallyError (tallyho/errors.py:4-5)
- sym:6162 Ledger.post (tallyho/ledger.py:28-35)
- … and 35 more
