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
- rec:ce6c56a0 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:9e749412 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:dcaa6f80 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError

## Code
- sym:38189 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:38181 Money (tallyho/money.py:22-83)
  class Money:
- sym:38211 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:38212 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:38206 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:38148 ValidationError (tallyho/errors.py:8-17)
- sym:38172 Order (tallyho/models.py:64-109)
- sym:38184 Money._check (tallyho/money.py:39-43)
- sym:38159 Ledger (tallyho/ledger.py:24-53)
- sym:38205 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:38186 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 705/800
omitted 105 lower-ranked items (expand an id for detail):
- rec:4534f323 attempt: Replacing the loop in `TallyError` with a comprehension made it shorte…
- rec:ea717dbe decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:5cc1d723 decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:ec15e243 decision: Errors about customer data must not echo the rejected value: they are…
- rec:5b651396 attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:3a74a748 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- … and 99 more
