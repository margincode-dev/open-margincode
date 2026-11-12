## Task
# relay-customer-region · relay-customer-region · open

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
- rec:15d50cf9 decision: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:6288e2dc attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:6ade7a19 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:0263a568 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…

## Code
- sym:33850 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:33832 ValidationError (tallyho/errors.py:8-17)
- sym:33956 check_region (tallyho/validate.py:24-26)
- sym:33958 parse_quantity (tallyho/validate.py:34-42)
- sym:33954 check_sku (tallyho/validate.py:14-16)
- sym:33955 check_email (tallyho/validate.py:19-21)
- sym:33957 require_positive (tallyho/validate.py:29-31)
- sym:33849 Product (tallyho/models.py:23-35)
  class Product:
- sym:33836 NotFound (tallyho/errors.py:24-25)
- sym:33856 Order (tallyho/models.py:64-109)
  class Order:
- sym:33851 Customer (tallyho/models.py:39-47)
- sym:33834 ValidationError.__str__ (tallyho/errors.py:15-17)

---
tokens: 711/800
omitted 49 lower-ranked items (expand an id for detail):
- rec:4dd06a5a attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:b9c14c49 decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:af0195ce decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:bbf147b0 decision: `_exponent` keeps its docstring as the one place its behaviour is desc…
- rec:791161c1 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:9c501ad9 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- … and 43 more
