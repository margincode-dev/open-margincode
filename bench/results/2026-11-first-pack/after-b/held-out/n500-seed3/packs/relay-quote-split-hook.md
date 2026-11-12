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
- rec:f1703578 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:be0b1961 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:9f1fc5d9 note: Benchmarks for `ValidationError` are in a separate, manual job; the unit tests must not depend on timing.
  anchors: tallyho/errors.py:ValidationError

## Code
- sym:41509 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:41501 Money (tallyho/money.py:22-83)
  class Money:
- sym:41531 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:41532 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:41526 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:41468 ValidationError (tallyho/errors.py:8-17)
- sym:41492 Order (tallyho/models.py:64-109)
- sym:41504 Money._check (tallyho/money.py:39-43)
- sym:41479 Ledger (tallyho/ledger.py:24-53)
- sym:41525 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:41506 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 704/800
omitted 204 lower-ranked items (expand an id for detail):
- rec:63489a9f attempt: Moving the imports inside `ValidationError` to speed up start-up saved…
- rec:ed3c2532 attempt: Adding type: ignore comments around `ValidationError` to quiet the che…
- rec:23e25b36 attempt: Replacing the loop in `parse_quantity` with a comprehension made it sh…
- rec:853ea520 attempt: Wrapping `CurrencyMismatch` in a bare try/except with a retry loop hid…
- rec:a4c923de attempt: Tried calling `Money.format` twice to force a refresh; the second call…
- rec:6021971b attempt: Wrapping the `validate` module in a bare try/except with a retry loop…
- … and 198 more
