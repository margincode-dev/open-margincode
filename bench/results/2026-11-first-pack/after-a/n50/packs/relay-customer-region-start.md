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
- rec:02cbef94 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:33828ee8 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:e1312b79 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:60ab2ce7 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…

## Code
- sym:18992 Product.__post_init__ (tallyho/models.py:30-35)
- sym:18974 ValidationError (tallyho/errors.py:8-17)
- sym:19098 check_region (tallyho/validate.py:24-26)
- sym:19100 parse_quantity (tallyho/validate.py:34-42)
- sym:19096 check_sku (tallyho/validate.py:14-16)
- sym:19097 check_email (tallyho/validate.py:19-21)
- sym:19099 require_positive (tallyho/validate.py:29-31)
- sym:18991 Product (tallyho/models.py:23-35)
  class Product:
- sym:18978 NotFound (tallyho/errors.py:24-25)
- sym:18998 Order (tallyho/models.py:64-109)
  class Order:
- sym:18993 Customer (tallyho/models.py:39-47)
- sym:18976 ValidationError.__str__ (tallyho/errors.py:15-17)

---
tokens: 709/800
omitted 49 lower-ranked items (expand an id for detail):
- rec:997ec113 decision: Errors about customer data must not echo the rejected value: they are…
- rec:77a5ee9c decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:9f34210e decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:bc93f19c decision: `_exponent` keeps its docstring as the one place its behaviour is desc…
- rec:e600ad2c attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:6c1865d0 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- … and 43 more
