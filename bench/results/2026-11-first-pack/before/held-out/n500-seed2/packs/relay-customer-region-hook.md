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
- sym:13578 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:13576 check_sku (tallyho/validate.py:14-16)
- sym:13472 Product.__post_init__ (tallyho/models.py:30-35)
- sym:13580 parse_quantity (tallyho/validate.py:34-42)
- sym:13577 check_email (tallyho/validate.py:19-21)
- sym:13579 require_positive (tallyho/validate.py:29-31)
- sym:13454 ValidationError (tallyho/errors.py:8-17)
- sym:13471 Product (tallyho/models.py:23-35)
- sym:13478 Order (tallyho/models.py:64-109)
- sym:13575 _REGION (tallyho/validate.py:11-11)
- sym:13473 Customer (tallyho/models.py:39-47)
- sym:13551 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 701/800
omitted 187 lower-ranked items (expand an id for detail):
- rec:5b8452fd note: The repr of objects around `ValidationError` shows up in logs, so keep…
- rec:e4875d1e decision: Errors about customer data must not echo the rejected value: they are…
- rec:efd0301b decision: Event payloads carry money as integer minor units plus the currency co…
- rec:5ec4e756 note: Deprecation warnings from `Money.format` are turned into errors in CI…
- rec:739c7c86 note: Deprecation warnings from `CurrencyMismatch` are turned into errors in…
- rec:737bf7c9 note: Deprecation warnings from `Repository.save_customer` are turned into e…
- … and 181 more
