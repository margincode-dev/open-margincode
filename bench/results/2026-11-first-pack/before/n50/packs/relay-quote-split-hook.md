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
- rec:a43cec69 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:57bfb220 decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:61f36b62 decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:b1efa366 decision: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:7851 Money.allocate (tallyho/money.py:63-68)
- sym:7843 Money (tallyho/money.py:22-83)
  class Money:
- sym:7873 Quote (tallyho/pricing/quote.py:16-27)
- sym:7874 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:7868 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:7810 ValidationError (tallyho/errors.py:8-17)
- sym:7834 Order (tallyho/models.py:64-109)
- sym:7846 Money._check (tallyho/money.py:39-43)
- sym:7821 Ledger (tallyho/ledger.py:24-53)
- sym:7867 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:7848 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 710/800
omitted 56 lower-ranked items (expand an id for detail):
- rec:fe98f757 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:d39aea30 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:b8647d38 attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:38d6d10d attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:bcb0a95f attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:bfd19541 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- … and 50 more
