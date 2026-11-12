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
- rec:ca0451d5 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:8c34a7d2 decision: Errors about customer data must not echo the rejected value: they are…
- rec:a678a792 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…

## Code
- sym:34839 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:34869 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:34840 Ledger.post (tallyho/ledger.py:28-35)
- sym:34886 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:34828 ValidationError (tallyho/errors.py:8-17)
- sym:34852 Order (tallyho/models.py:64-109)
- sym:34861 Money (tallyho/money.py:22-83)
- sym:34831 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:34845 Product (tallyho/models.py:23-35)
- sym:34849 OrderLine (tallyho/models.py:51-60)
- sym:34885 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:34891 Quote (tallyho/pricing/quote.py:16-27)

---
tokens: 710/800
omitted 52 lower-ranked items (expand an id for detail):
- rec:0eccfaaa attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:a321ad41 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:3d09a671 decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:58688a38 decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:2d807ed0 decision: `_exponent` keeps its docstring as the one place its behaviour is desc…
- rec:bea894da attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- … and 46 more
