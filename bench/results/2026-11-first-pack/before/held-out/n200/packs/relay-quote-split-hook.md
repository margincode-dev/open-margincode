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
- rec:ff0bdb96 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:8a6a8f90 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:ab547572 attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…

## Code
- sym:12831 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:12823 Money (tallyho/money.py:22-83)
  class Money:
- sym:12853 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:12854 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:12848 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:12790 ValidationError (tallyho/errors.py:8-17)
- sym:12814 Order (tallyho/models.py:64-109)
- sym:12826 Money._check (tallyho/money.py:39-43)
- sym:12801 Ledger (tallyho/ledger.py:24-53)
- sym:12847 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:12828 Money.__sub__ (tallyho/money.py:49-51)
- sym:12827 Money.__add__ (tallyho/money.py:45-47)

---
tokens: 712/800
omitted 88 lower-ranked items (expand an id for detail):
- rec:47a3e3b1 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:6db892ed attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:961211de decision: Tests for `Money._check` build their inputs with small factory helpers…
- rec:03cb7ccc attempt: Tried calling `check_sku` twice to force a refresh; the second call re…
- rec:f0628077 attempt: Replacing the loop in `TallyError` with a comprehension made it shorte…
- rec:83c33f57 attempt: Caching what `Product.__post_init__` returns in a module-level dict br…
- … and 82 more
