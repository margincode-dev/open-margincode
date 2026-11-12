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
- rec:955aaeb2 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:ebd34257 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:63c94cca note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:bc90fa0c decision: Tests for `Money.format` build their inputs with small factory helpers…

## Code
- sym:21671 Money.allocate (tallyho/money.py:63-68)
- sym:21663 Money (tallyho/money.py:22-83)
  class Money:
- sym:21693 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:21694 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:21688 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:21630 ValidationError (tallyho/errors.py:8-17)
- sym:21654 Order (tallyho/models.py:64-109)
- sym:21666 Money._check (tallyho/money.py:39-43)
- sym:21641 Ledger (tallyho/ledger.py:24-53)
- sym:21687 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:21668 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 720/800
omitted 171 lower-ranked items (expand an id for detail):
- rec:9e6ddea8 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:1dfe601c attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:3123a2cb attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:109fda8f attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:d438b54e decision: Tests for `Money._check` build their inputs with small factory helpers…
- rec:87405c2d attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- … and 165 more
