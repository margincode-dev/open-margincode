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
- rec:08b80d02 attempt: Moving the imports inside `ValidationError` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/errors.py:ValidationError
- rec:852be649 attempt: Adding type: ignore comments around `ValidationError` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/errors.py:ValidationError
- rec:90c1f367 note: Benchmarks for `ValidationError` are in a separate, manual job; the unit tests must not depend on timing.
  anchors: tallyho/errors.py:ValidationError
- rec:e501c760 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…

## Code
- sym:16151 Money.allocate (tallyho/money.py:63-68)
- sym:16143 Money (tallyho/money.py:22-83)
  class Money:
- sym:16173 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:16174 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:16168 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:16110 ValidationError (tallyho/errors.py:8-17)
- sym:16134 Order (tallyho/models.py:64-109)
- sym:16146 Money._check (tallyho/money.py:39-43)
- sym:16121 Ledger (tallyho/ledger.py:24-53)
- sym:16167 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:16148 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 721/800
omitted 188 lower-ranked items (expand an id for detail):
- rec:ec13b732 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:826505a2 attempt: Wrapping `Money._check` in a bare try/except with a retry loop hid the…
- rec:f0401197 attempt: Wrapping `Money.__sub__` in a bare try/except with a retry loop hid th…
- rec:a0b8ea9e note: Remember `Money.format` is not thread-safe; each worker needs its own…
- rec:0dc1bb38 attempt: Replacing the loop in `_exponent` with a comprehension made it shorter…
- rec:a09f5823 attempt: Replacing the loop in the `errors` module with a comprehension made it…
- … and 182 more
