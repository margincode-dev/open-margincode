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
- rec:4f324c9a decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:20758 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:20756 check_sku (tallyho/validate.py:14-16)
- sym:20652 Product.__post_init__ (tallyho/models.py:30-35)
- sym:20760 parse_quantity (tallyho/validate.py:34-42)
- sym:20757 check_email (tallyho/validate.py:19-21)
- sym:20759 require_positive (tallyho/validate.py:29-31)
- sym:20634 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:20651 Product (tallyho/models.py:23-35)
- sym:20658 Order (tallyho/models.py:64-109)
- sym:20755 _REGION (tallyho/validate.py:11-11)
- sym:20653 Customer (tallyho/models.py:39-47)
- sym:20731 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 708/800
omitted 168 lower-ranked items (expand an id for detail):
- rec:88814a8c decision: Tests for `check_region` build their inputs with small factory helpers…
- rec:5670f76a note: The `validate` module reads files as UTF-8 explicitly; the platform de…
- rec:ee5eeaae note: Deprecation warnings from `Customer` are turned into errors in CI (fil…
- rec:77b32c91 note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:7def3958 note: Deprecation warnings from `customer_from_record` are turned into error…
- rec:e45c635f attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- … and 162 more
