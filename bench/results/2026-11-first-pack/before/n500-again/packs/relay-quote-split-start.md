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
- sym:11188 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:11130 ValidationError (tallyho/errors.py:8-17)
- sym:11163 Money (tallyho/money.py:22-83)
  class Money:
- sym:11141 Ledger (tallyho/ledger.py:24-53)
- sym:11187 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:11193 Quote (tallyho/pricing/quote.py:16-27)
- sym:11194 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:11129 TallyError (tallyho/errors.py:4-5)
- sym:11142 Ledger.post (tallyho/ledger.py:28-35)
- sym:11148 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 714/800
omitted 165 lower-ranked items (expand an id for detail):
- rec:b507702b attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- rec:4c74690f attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:24d37bf4 attempt: Tried calling `check_email` twice to force a refresh; the second call…
- rec:52c66d53 attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- rec:c1897b8b attempt: Tried calling `Coupon.discount` twice to force a refresh; the second c…
- rec:a21bf180 attempt: Wrapping the `errors` module in a bare try/except with a retry loop hi…
- … and 159 more
