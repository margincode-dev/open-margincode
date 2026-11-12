## Task
# relay-quote-split · relay-quote-split · open
Goal: Customers want to split a bill evenly between several payers. Add `Quote.split(parts)`, returning what each of the `parts` payers pays, as a list of Money. Raise ValidationError when parts is less than 1.

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
- rec:71e7aa69 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:cde4b530 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:573fbaeb attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…

## Code
- sym:19981 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:20011 Money.allocate (tallyho/money.py:63-68)
- sym:19982 Ledger.post (tallyho/ledger.py:28-35)
- sym:20028 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:19970 ValidationError (tallyho/errors.py:8-17)
- sym:19994 Order (tallyho/models.py:64-109)
- sym:20003 Money (tallyho/money.py:22-83)
- sym:19973 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:19987 Product (tallyho/models.py:23-35)
- sym:19991 OrderLine (tallyho/models.py:51-60)
- sym:20027 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:20033 Quote (tallyho/pricing/quote.py:16-27)

---
tokens: 714/800
omitted 52 lower-ranked items (expand an id for detail):
- rec:d01c4b13 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:135000eb decision: Errors about customer data must not echo the rejected value: they are…
- rec:aecd989b decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:ae5d2c6c decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:babba7e2 decision: `_exponent` keeps its docstring as the one place its behaviour is desc…
- rec:3026f9e8 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- … and 46 more
