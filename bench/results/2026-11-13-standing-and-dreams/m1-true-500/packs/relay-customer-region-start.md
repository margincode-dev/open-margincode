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
- rec:5d95b770 decision, open: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:b784b75d attempt, open: Wrapping `ValidationError` in a bare try/except with a retry loop hid…
- rec:3f6d7f4e decision, open: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:5897ec07 decision, open: Tests for `check_region` build their inputs with small factory helpers…

## Code
- sym:5172 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:5278 check_region (tallyho/validate.py:24-26)
- sym:5280 parse_quantity (tallyho/validate.py:34-42)
- sym:5276 check_sku (tallyho/validate.py:14-16)
- sym:5277 check_email (tallyho/validate.py:19-21)
- sym:5279 require_positive (tallyho/validate.py:29-31)
- sym:5253 JsonFileStore._path (tallyho/store/jsonfile.py:20-23)
- sym:5186 _exponent (tallyho/money.py:14-18)
- sym:5154 ValidationError (tallyho/errors.py:8-17)
- sym:5171 Product (tallyho/models.py:23-35)
  class Product:
- sym:5178 Order (tallyho/models.py:64-109)
  class Order:

---
tokens: 707/800
omitted 165 lower-ranked items (expand an id for detail):
- rec:5ef4dd09 note, open: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:8592b9f9 attempt, open: Adding type: ignore comments around `check_email` to quiet the checker…
- rec:64bd19c2 attempt, open: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:ad318c6d attempt, open: Tried calling `check_email` twice to force a refresh; the second call…
- rec:a5029afd attempt, open: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- … and 160 more
