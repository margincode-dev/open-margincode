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
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:a1dea4f6 attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:eb5f1399 attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:091e51fe attempt, open: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:8baad586 intent, open: Earlier work on this code

## Code
- sym:4531 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:4523 Money (tallyho/money.py:22-83)
- sym:4553 Quote (tallyho/pricing/quote.py:16-27)
- sym:4554 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:4548 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:4490 ValidationError (tallyho/errors.py:8-17)
- sym:4514 Order (tallyho/models.py:64-109)
- sym:4526 Money._check (tallyho/money.py:39-43)
- sym:4501 Ledger (tallyho/ledger.py:24-53)
- sym:4547 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:4528 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 700/800
omitted 74 lower-ranked items (expand an id for detail):
- rec:8b4ae970 attempt, open: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:7a3c92f9 decision, open: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:71cb2093 decision, open: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:a54d6cf6 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:83acd78e attempt, open: Replacing the loop in `Money` with a comprehension made it shorter but…
- … and 69 more
