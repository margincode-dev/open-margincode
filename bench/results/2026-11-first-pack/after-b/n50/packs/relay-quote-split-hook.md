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
- rec:fa43567f attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:0eccfaaa attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…

## Code
- sym:34869 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:34861 Money (tallyho/money.py:22-83)
- sym:34891 Quote (tallyho/pricing/quote.py:16-27)
- sym:34892 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:34886 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:34828 ValidationError (tallyho/errors.py:8-17)
- sym:34852 Order (tallyho/models.py:64-109)
- sym:34864 Money._check (tallyho/money.py:39-43)
- sym:34839 Ledger (tallyho/ledger.py:24-53)
- sym:34885 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:34866 Money.__sub__ (tallyho/money.py:49-51)
- sym:34865 Money.__add__ (tallyho/money.py:45-47)

---
tokens: 706/800
omitted 74 lower-ranked items (expand an id for detail):
- rec:a321ad41 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:58688a38 decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:3d09a671 decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:8c34a7d2 decision: Errors about customer data must not echo the rejected value: they are…
- rec:55ed5a66 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:bea894da attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- … and 68 more
