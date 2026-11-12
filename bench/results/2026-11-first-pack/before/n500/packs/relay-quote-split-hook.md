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
- rec:cdd7d276 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:411ed629 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:c016800e note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:a67a0b11 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…

## Code
- sym:9511 Money.allocate (tallyho/money.py:63-68)
- sym:9503 Money (tallyho/money.py:22-83)
  class Money:
- sym:9533 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:9534 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:9528 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:9470 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:9494 Order (tallyho/models.py:64-109)
- sym:9506 Money._check (tallyho/money.py:39-43)
- sym:9481 Ledger (tallyho/ledger.py:24-53)
- sym:9527 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:9508 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 713/800
omitted 171 lower-ranked items (expand an id for detail):
- rec:887751e4 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:2cbb373b attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:a5249aef attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:596c1ce7 decision: Tests for `Money.format` build their inputs with small factory helpers…
- rec:03e5c826 decision: Tests for `Money._check` build their inputs with small factory helpers…
- rec:34ee09ab attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- … and 165 more
