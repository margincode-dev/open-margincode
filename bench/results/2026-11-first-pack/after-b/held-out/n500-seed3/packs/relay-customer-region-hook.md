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
- rec:32f93c62 decision: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:0270fb70 note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:4d2f3c14 attempt: Moving the imports inside `ValidationError` to speed up start-up saved…
- rec:ea40e92a attempt: Adding type: ignore comments around `ValidationError` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/errors.py:ValidationError

## Code
- sym:40596 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:40594 check_sku (tallyho/validate.py:14-16)
- sym:40490 Product.__post_init__ (tallyho/models.py:30-35)
- sym:40598 parse_quantity (tallyho/validate.py:34-42)
- sym:40595 check_email (tallyho/validate.py:19-21)
- sym:40597 require_positive (tallyho/validate.py:29-31)
- sym:40472 ValidationError (tallyho/errors.py:8-17)
- sym:40489 Product (tallyho/models.py:23-35)
- sym:40496 Order (tallyho/models.py:64-109)
- sym:40593 _REGION (tallyho/validate.py:11-11)
- sym:40491 Customer (tallyho/models.py:39-47)
- sym:40569 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 708/800
omitted 197 lower-ranked items (expand an id for detail):
- rec:9a21fe75 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:75840457 attempt: Tried calling `check_region` twice to force a refresh; the second call…
- rec:af77bc1a decision: New optional parameters of the `errors` module are keyword-only (after…
- rec:77b568a3 attempt: Tried calling `ValidationError.__str__` twice to force a refresh; the…
- rec:f7a705cf note: Deprecation warnings from `check_region` are turned into errors in CI…
- rec:c91b844a note: `check_sku` reads files as UTF-8 explicitly; the platform default enco…
- … and 191 more
