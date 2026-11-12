## Task
# relay-customer-region · relay-customer-region · open
Goal: Customer regions are never checked, so typos such as "us-ca" end up in the store. Validate the region when a Customer is created and raise ValidationError (with field "region") for an invalid one.

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
- rec:06c9c012 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:f825fa52 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:f393fc54 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:506bb54b attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…

## Code
- sym:6832 Product.__post_init__ (tallyho/models.py:30-35)
- sym:6814 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:6938 check_region (tallyho/validate.py:24-26)
- sym:6940 parse_quantity (tallyho/validate.py:34-42)
- sym:6936 check_sku (tallyho/validate.py:14-16)
- sym:6937 check_email (tallyho/validate.py:19-21)
- sym:6939 require_positive (tallyho/validate.py:29-31)
- sym:6831 Product (tallyho/models.py:23-35)
- sym:6818 NotFound (tallyho/errors.py:24-25)
- sym:6838 Order (tallyho/models.py:64-109)
- sym:6833 Customer (tallyho/models.py:39-47)
- sym:6816 ValidationError.__str__ (tallyho/errors.py:15-17)

---
tokens: 702/800
omitted 49 lower-ranked items (expand an id for detail):
- rec:f6716ed2 decision: Errors about customer data must not echo the rejected value: they are…
- rec:1ffcbe58 decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:e5c689fd decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:090e47e9 decision: `_exponent` keeps its docstring as the one place its behaviour is desc…
- rec:a2fe4334 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:c98c8afd attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- … and 43 more
