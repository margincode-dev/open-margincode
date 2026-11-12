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
- rec:37723df2 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:ec1ee52d decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:36bffe1a note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:2fca4c77 attempt: Adding type: ignore comments around `check_email` to quiet the checker…

## Code
- sym:8598 check_region (tallyho/validate.py:24-26)
- sym:8600 parse_quantity (tallyho/validate.py:34-42)
- sym:8597 check_email (tallyho/validate.py:19-21)
- sym:8596 check_sku (tallyho/validate.py:14-16)
- sym:8599 require_positive (tallyho/validate.py:29-31)
- sym:8492 Product.__post_init__ (tallyho/models.py:30-35)
- sym:8474 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:8498 Order (tallyho/models.py:64-109)
  class Order:
- sym:8571 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:8473 TallyError (tallyho/errors.py:4-5)
- sym:8486 Ledger.post (tallyho/ledger.py:28-35)
- sym:8500 Order.add (tallyho/models.py:76-84)

---
tokens: 711/800
omitted 160 lower-ranked items (expand an id for detail):
- rec:42575255 attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:4cef8ce4 attempt: Tried calling `check_email` twice to force a refresh; the second call…
- rec:b0ee25ae attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- rec:7b59a50d attempt: Tried calling `Coupon.discount` twice to force a refresh; the second c…
- rec:2f3adad6 attempt: Wrapping the `errors` module in a bare try/except with a retry loop hi…
- rec:741bc492 attempt: Adding type: ignore comments around `Money.allocate` to quiet the chec…
- … and 154 more
