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
- rec:52c8b89b decision: Type hints on `ValidationError.__str__` use the built-in generics (lis…
- rec:63489a9f attempt: Moving the imports inside `ValidationError` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/errors.py:ValidationError

## Code
- sym:41509 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:41526 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:41468 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:41501 Money (tallyho/money.py:22-83)
- sym:41479 Ledger (tallyho/ledger.py:24-53)
- sym:41525 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:41531 Quote (tallyho/pricing/quote.py:16-27)
- sym:41532 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:41467 TallyError (tallyho/errors.py:4-5)
- sym:41480 Ledger.post (tallyho/ledger.py:28-35)
- sym:41486 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 707/800
omitted 181 lower-ranked items (expand an id for detail):
- rec:ed3c2532 attempt: Adding type: ignore comments around `ValidationError` to quiet the che…
- rec:9f1fc5d9 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:fceda8ff attempt: Replacing the loop in `_exponent` with a comprehension made it shorter…
- rec:4cf8febe attempt: Replacing the loop in the `errors` module with a comprehension made it…
- rec:219945c4 attempt: Moving the imports inside `check_sku` to speed up start-up saved under…
- rec:acb2ae8d attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- … and 175 more
