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
- rec:4a425e32 decision: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:8d641cda decision: Log lines from `ValidationError` go through the logging module, never…
- rec:3d2838aa attempt: Tried calling `ValidationError` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/errors.py:ValidationError
- rec:53122ce8 decision: Constants that `ValidationError` depends on live at the top of its mod…

## Code
- sym:38830 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:38936 check_region (tallyho/validate.py:24-26)
- sym:38938 parse_quantity (tallyho/validate.py:34-42)
- sym:38934 check_sku (tallyho/validate.py:14-16)
- sym:38935 check_email (tallyho/validate.py:19-21)
- sym:38937 require_positive (tallyho/validate.py:29-31)
- sym:38911 JsonFileStore._path (tallyho/store/jsonfile.py:20-23)
- sym:38844 _exponent (tallyho/money.py:14-18)
- sym:38812 ValidationError (tallyho/errors.py:8-17)
- sym:38829 Product (tallyho/models.py:23-35)
- sym:38836 Order (tallyho/models.py:64-109)
- sym:38831 Customer (tallyho/models.py:39-47)

---
tokens: 708/800
omitted 183 lower-ranked items (expand an id for detail):
- rec:15213cb8 decision: `ValidationError` keeps its docstring as the one place its behaviour i…
- rec:74e22423 note: The repr of objects around `ValidationError` shows up in logs, so keep…
- rec:68847948 attempt: Wrapping `Order.remove` in a bare try/except with a retry loop hid the…
- rec:e7cb916d attempt: Moving the imports inside `check_sku` to speed up start-up saved under…
- rec:55b1eef8 attempt: Moving the imports inside `Money.allocate` to speed up start-up saved…
- rec:f4995aac attempt: Wrapping `JsonFileStore.put` in a bare try/except with a retry loop hi…
- … and 177 more
