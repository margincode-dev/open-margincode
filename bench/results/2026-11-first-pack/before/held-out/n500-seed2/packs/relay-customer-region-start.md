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
- rec:303e4225 attempt: Tried calling `ValidationError` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/errors.py:ValidationError
- rec:dfa9ea23 decision: Log lines from `ValidationError` go through the logging module, never print(), so operators can filter them by logger name.
  anchors: tallyho/errors.py:ValidationError
- rec:31c70c45 decision: Constants that `ValidationError` depends on live at the top of its module in UPPER_CASE, not as magic numbers inline.
  anchors: tallyho/errors.py:ValidationError
- rec:b61cc598 decision: `ValidationError` keeps its docstring as the one place its behaviour i…

## Code
- sym:13472 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:13578 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:13580 parse_quantity (tallyho/validate.py:34-42)
- sym:13576 check_sku (tallyho/validate.py:14-16)
- sym:13577 check_email (tallyho/validate.py:19-21)
- sym:13579 require_positive (tallyho/validate.py:29-31)
- sym:13553 JsonFileStore._path (tallyho/store/jsonfile.py:20-23)
- sym:13486 _exponent (tallyho/money.py:14-18)
- sym:13454 ValidationError (tallyho/errors.py:8-17)
- sym:13471 Product (tallyho/models.py:23-35)
  class Product:
- sym:13478 Order (tallyho/models.py:64-109)

---
tokens: 715/800
omitted 184 lower-ranked items (expand an id for detail):
- rec:5b8452fd note: The repr of objects around `ValidationError` shows up in logs, so keep…
- rec:807eef28 attempt: Wrapping `Order.remove` in a bare try/except with a retry loop hid the…
- rec:de2c9dd4 attempt: Moving the imports inside `check_sku` to speed up start-up saved under…
- rec:efcb47e2 attempt: Moving the imports inside `Money.allocate` to speed up start-up saved…
- rec:08e7d060 attempt: Wrapping `JsonFileStore.put` in a bare try/except with a retry loop hi…
- rec:e1d319ef attempt: Moving the imports inside `Workflow._move` to speed up start-up saved…
- … and 178 more
