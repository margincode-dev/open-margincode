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
- rec:fdd3c27e decision: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:45201186 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid…
- rec:0a62fad3 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:7d33ee90 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…

## Code
- sym:35616 check_region (tallyho/validate.py:24-26)
- sym:35618 parse_quantity (tallyho/validate.py:34-42)
- sym:35614 check_sku (tallyho/validate.py:14-16)
- sym:35615 check_email (tallyho/validate.py:19-21)
- sym:35617 require_positive (tallyho/validate.py:29-31)
- sym:35510 Product.__post_init__ (tallyho/models.py:30-35)
- sym:35492 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:35516 Order (tallyho/models.py:64-109)
- sym:35589 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:35491 TallyError (tallyho/errors.py:4-5)
- sym:35504 Ledger.post (tallyho/ledger.py:28-35)
- sym:35518 Order.add (tallyho/models.py:76-84)

---
tokens: 715/800
omitted 160 lower-ranked items (expand an id for detail):
- rec:1d128513 attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- rec:c3dd2ad2 attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:a03fc966 attempt: Tried calling `check_email` twice to force a refresh; the second call…
- rec:7fef73b0 attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- rec:33b93c54 attempt: Tried calling `Coupon.discount` twice to force a refresh; the second c…
- rec:60f58cb0 attempt: Wrapping the `errors` module in a bare try/except with a retry loop hi…
- … and 154 more
