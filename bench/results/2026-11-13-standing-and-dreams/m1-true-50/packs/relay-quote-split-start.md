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
- rec:a54d6cf6 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:3a1c2c5b attempt, open: Moving the imports inside `Ledger.post` to speed up start-up saved und…

## Code
- sym:4501 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:4531 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:4502 Ledger.post (tallyho/ledger.py:28-35)
- sym:4548 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:4490 ValidationError (tallyho/errors.py:8-17)
- sym:4514 Order (tallyho/models.py:64-109)
  class Order:
- sym:4523 Money (tallyho/money.py:22-83)
- sym:4493 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:4507 Product (tallyho/models.py:23-35)
- sym:4511 OrderLine (tallyho/models.py:51-60)
- sym:4547 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:4553 Quote (tallyho/pricing/quote.py:16-27)

---
tokens: 706/800
omitted 52 lower-ranked items (expand an id for detail):
- rec:091e51fe attempt, open: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:8b4ae970 attempt, open: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:71cb2093 decision, open: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:7a3c92f9 decision, open: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:0cd8412d decision, open: `_exponent` keeps its docstring as the one place its behaviour is desc…
- … and 47 more
