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
- rec:6ca1e6ff attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:d1b057a3 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:c249a31d note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:a39075ff decision: Tests for `check_region` build their inputs with small factory helpers…

## Code
- sym:11171 Money.allocate (tallyho/money.py:63-68)
- sym:11163 Money (tallyho/money.py:22-83)
  class Money:
- sym:11193 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:11194 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:11188 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:11130 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:11154 Order (tallyho/models.py:64-109)
- sym:11166 Money._check (tallyho/money.py:39-43)
- sym:11141 Ledger (tallyho/ledger.py:24-53)
- sym:11187 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:11168 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 716/800
omitted 171 lower-ranked items (expand an id for detail):
- rec:04bc2b50 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:4c74690f attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:49841538 attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:ab03b06e attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:debe6b2a decision: Tests for `Money.format` build their inputs with small factory helpers…
- rec:b6dcf907 decision: Tests for `Money._check` build their inputs with small factory helpers…
- … and 165 more
