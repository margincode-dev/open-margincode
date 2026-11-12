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
- rec:1f4f8509 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError
- rec:c16cb1a2 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:6969d1ce note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:bf56e1b9 attempt: Adding type: ignore comments around `check_email` to quiet the checker…

## Code
- sym:10258 check_region (tallyho/validate.py:24-26)
- sym:10260 parse_quantity (tallyho/validate.py:34-42)
- sym:10256 check_sku (tallyho/validate.py:14-16)
- sym:10257 check_email (tallyho/validate.py:19-21)
- sym:10259 require_positive (tallyho/validate.py:29-31)
- sym:10152 Product.__post_init__ (tallyho/models.py:30-35)
- sym:10134 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:10158 Order (tallyho/models.py:64-109)
  class Order:
- sym:10231 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:10133 TallyError (tallyho/errors.py:4-5)
- sym:10146 Ledger.post (tallyho/ledger.py:28-35)
- sym:10160 Order.add (tallyho/models.py:76-84)

---
tokens: 716/800
omitted 160 lower-ranked items (expand an id for detail):
- rec:57ccaff7 attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:2f873bb5 attempt: Tried calling `check_email` twice to force a refresh; the second call…
- rec:a0a67f1c attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- rec:f50a0d41 attempt: Tried calling `Coupon.discount` twice to force a refresh; the second c…
- rec:229c4d21 attempt: Wrapping the `errors` module in a bare try/except with a retry loop hi…
- rec:439d3e71 attempt: Adding type: ignore comments around `Money.allocate` to quiet the chec…
- … and 154 more
