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
- rec:90c1f367 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:0dc1bb38 attempt: Replacing the loop in `_exponent` with a comprehension made it shorter…

## Code
- sym:16151 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
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
tokens: 710/800
omitted 183 lower-ranked items (expand an id for detail):
- rec:a09f5823 attempt: Replacing the loop in the `errors` module with a comprehension made it…
- rec:38f29db7 attempt: Moving the imports inside `check_sku` to speed up start-up saved under…
- rec:088c87a7 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:33970ffa attempt: Tried calling `check_region` twice to force a refresh; the second call…
- rec:d6a34135 attempt: Replacing the loop in `MemoryStore.put` with a comprehension made it s…
- rec:782881f3 attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- … and 177 more
