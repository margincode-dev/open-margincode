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
- rec:3b13fdee attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:d0e4a731 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:6ca0e2bb note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:e45c635f attempt: Adding type: ignore comments around `check_email` to quiet the checker…

## Code
- sym:20758 check_region (tallyho/validate.py:24-26)
- sym:20760 parse_quantity (tallyho/validate.py:34-42)
- sym:20756 check_sku (tallyho/validate.py:14-16)
- sym:20757 check_email (tallyho/validate.py:19-21)
- sym:20759 require_positive (tallyho/validate.py:29-31)
- sym:20652 Product.__post_init__ (tallyho/models.py:30-35)
- sym:20634 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:20658 Order (tallyho/models.py:64-109)
  class Order:
- sym:20731 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:20633 TallyError (tallyho/errors.py:4-5)
- sym:20646 Ledger.post (tallyho/ledger.py:28-35)
- sym:20660 Order.add (tallyho/models.py:76-84)

---
tokens: 713/800
omitted 160 lower-ranked items (expand an id for detail):
- rec:e1ec52ec attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:f4b6b47a attempt: Tried calling `check_email` twice to force a refresh; the second call…
- rec:1effe075 attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- rec:efbd89d0 attempt: Tried calling `Coupon.discount` twice to force a refresh; the second c…
- rec:62dbbfd7 attempt: Wrapping the `errors` module in a bare try/except with a retry loop hi…
- rec:2d1957d9 attempt: Adding type: ignore comments around `Money.allocate` to quiet the chec…
- … and 154 more
