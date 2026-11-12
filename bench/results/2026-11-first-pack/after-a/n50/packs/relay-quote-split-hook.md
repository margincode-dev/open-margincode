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
- rec:d01c4b13 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:ae5d2c6c decision: New optional parameters of `parse_quantity` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/validate.py:parse_quantity
- rec:aecd989b decision: Tests for `TallyError` build their inputs with small factory helpers i…

## Code
- sym:20011 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:20003 Money (tallyho/money.py:22-83)
  class Money:
- sym:20033 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:20034 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:20028 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:19970 ValidationError (tallyho/errors.py:8-17)
- sym:19994 Order (tallyho/models.py:64-109)
- sym:20006 Money._check (tallyho/money.py:39-43)
- sym:19981 Ledger (tallyho/ledger.py:24-53)
- sym:20027 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:20008 Money.__sub__ (tallyho/money.py:49-51)
- sym:20007 Money.__add__ (tallyho/money.py:45-47)

---
tokens: 712/800
omitted 56 lower-ranked items (expand an id for detail):
- rec:aafc2f2d attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:ab67399e attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:665ef687 attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:7389d1d0 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:71e7aa69 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:cde4b530 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- … and 50 more
