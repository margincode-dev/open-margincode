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
- rec:3f26e474 note: Deprecation warnings from `ValidationError.__init__` are turned into e…
- rec:3d2838aa attempt: Tried calling `ValidationError` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/errors.py:ValidationError
- rec:53122ce8 decision: Constants that `ValidationError` depends on live at the top of its mod…

## Code
- sym:38936 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:38934 check_sku (tallyho/validate.py:14-16)
- sym:38830 Product.__post_init__ (tallyho/models.py:30-35)
- sym:38938 parse_quantity (tallyho/validate.py:34-42)
- sym:38935 check_email (tallyho/validate.py:19-21)
- sym:38937 require_positive (tallyho/validate.py:29-31)
- sym:38812 ValidationError (tallyho/errors.py:8-17)
- sym:38829 Product (tallyho/models.py:23-35)
- sym:38836 Order (tallyho/models.py:64-109)
- sym:38933 _REGION (tallyho/validate.py:11-11)
- sym:38831 Customer (tallyho/models.py:39-47)
- sym:38909 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 703/800
omitted 201 lower-ranked items (expand an id for detail):
- rec:15213cb8 decision: `ValidationError` keeps its docstring as the one place its behaviour i…
- rec:74e22423 note: The repr of objects around `ValidationError` shows up in logs, so keep…
- rec:8d641cda decision: Log lines from `ValidationError` go through the logging module, never…
- rec:bdd1240f attempt: Caching what `check_region` returns in a module-level dict broke tests…
- rec:633c0e6f attempt: Tried calling `ValidationError.__str__` twice to force a refresh; the…
- rec:f5af0a15 note: `Workflow.place` reads files as UTF-8 explicitly; the platform default…
- … and 195 more
