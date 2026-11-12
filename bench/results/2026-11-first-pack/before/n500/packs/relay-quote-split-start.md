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
- rec:34ee09ab attempt: Adding type: ignore comments around `check_email` to quiet the checker…

## Code
- sym:9511 Money.allocate (tallyho/money.py:63-68)
- sym:9528 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:9470 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:9503 Money (tallyho/money.py:22-83)
- sym:9481 Ledger (tallyho/ledger.py:24-53)
- sym:9527 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:9533 Quote (tallyho/pricing/quote.py:16-27)
- sym:9534 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:9469 TallyError (tallyho/errors.py:4-5)
- sym:9482 Ledger.post (tallyho/ledger.py:28-35)
- sym:9488 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 716/800
omitted 165 lower-ranked items (expand an id for detail):
- rec:2cbb373b attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:e440ecf9 attempt: Tried calling `check_email` twice to force a refresh; the second call…
- rec:5ecf85fb attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- rec:5bfb33c0 attempt: Tried calling `Coupon.discount` twice to force a refresh; the second c…
- rec:16949822 attempt: Wrapping the `errors` module in a bare try/except with a retry loop hi…
- rec:37b375fd attempt: Adding type: ignore comments around `Money.allocate` to quiet the chec…
- … and 159 more
