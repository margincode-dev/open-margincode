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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:a27b3c1a decision, open: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:e5d46923 attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:bc067c77 attempt, open: Moving the imports inside `Ledger.post` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/ledger.py:Ledger.post

## Code
- sym:3512 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:3494 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:3618 check_region (tallyho/validate.py:24-26)
- sym:3620 parse_quantity (tallyho/validate.py:34-42)
- sym:3617 check_email (tallyho/validate.py:19-21)
- sym:3616 check_sku (tallyho/validate.py:14-16)
- sym:3619 require_positive (tallyho/validate.py:29-31)
- sym:3511 Product (tallyho/models.py:23-35)
  class Product:
- sym:3498 NotFound (tallyho/errors.py:24-25)
  class NotFound(TallyError):
- sym:3518 Order (tallyho/models.py:64-109)
- sym:3513 Customer (tallyho/models.py:39-47)

---
tokens: 703/800
omitted 51 lower-ranked items (expand an id for detail):
- rec:1281bce2 attempt, open: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:3fa380b3 attempt, open: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:e3559239 decision, open: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:1726c433 decision, open: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:d65557d3 decision, open: `_exponent` keeps its docstring as the one place its behaviour is desc…
- … and 46 more
