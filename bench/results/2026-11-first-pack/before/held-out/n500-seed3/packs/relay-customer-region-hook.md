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
- rec:db18d910 attempt: Moving the imports inside `ValidationError` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/errors.py:ValidationError
- rec:5455f78a attempt: Adding type: ignore comments around `ValidationError` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/errors.py:ValidationError
- rec:ccdeb676 note: Benchmarks for `ValidationError` are in a separate, manual job; the unit tests must not depend on timing.
  anchors: tallyho/errors.py:ValidationError
- rec:c06619b9 attempt: Tried calling `check_region` twice to force a refresh; the second call…

## Code
- sym:15238 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:15236 check_sku (tallyho/validate.py:14-16)
- sym:15132 Product.__post_init__ (tallyho/models.py:30-35)
- sym:15240 parse_quantity (tallyho/validate.py:34-42)
- sym:15237 check_email (tallyho/validate.py:19-21)
- sym:15239 require_positive (tallyho/validate.py:29-31)
- sym:15114 ValidationError (tallyho/errors.py:8-17)
- sym:15131 Product (tallyho/models.py:23-35)
  class Product:
- sym:15138 Order (tallyho/models.py:64-109)
- sym:15235 _REGION (tallyho/validate.py:11-11)
- sym:15133 Customer (tallyho/models.py:39-47)
- sym:15211 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 713/800
omitted 181 lower-ranked items (expand an id for detail):
- rec:f8fb1037 decision: Errors about customer data must not echo the rejected value: they are…
- rec:7ecba55a note: Deprecation warnings from `check_region` are turned into errors in CI…
- rec:358236e4 note: `check_sku` reads files as UTF-8 explicitly; the platform default enco…
- rec:22b6bea6 note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:679c3ce0 note: `Money._check` reads files as UTF-8 explicitly; the platform default e…
- rec:42cb68b2 note: Deprecation warnings from `customer_from_record` are turned into error…
- … and 175 more
