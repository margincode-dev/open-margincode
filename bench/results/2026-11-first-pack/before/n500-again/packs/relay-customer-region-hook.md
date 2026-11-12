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
- rec:4f467e1f decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:10258 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:10256 check_sku (tallyho/validate.py:14-16)
- sym:10152 Product.__post_init__ (tallyho/models.py:30-35)
- sym:10260 parse_quantity (tallyho/validate.py:34-42)
- sym:10257 check_email (tallyho/validate.py:19-21)
- sym:10259 require_positive (tallyho/validate.py:29-31)
- sym:10134 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:10151 Product (tallyho/models.py:23-35)
- sym:10158 Order (tallyho/models.py:64-109)
- sym:10255 _REGION (tallyho/validate.py:11-11)
- sym:10153 Customer (tallyho/models.py:39-47)
- sym:10231 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 706/800
omitted 168 lower-ranked items (expand an id for detail):
- rec:9bd22f09 decision: Tests for `check_region` build their inputs with small factory helpers…
- rec:df4ddc39 note: The `validate` module reads files as UTF-8 explicitly; the platform de…
- rec:b086d408 note: Deprecation warnings from `Customer` are turned into errors in CI (fil…
- rec:bb297173 note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:689d53dd note: Deprecation warnings from `customer_from_record` are turned into error…
- rec:bf56e1b9 attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- … and 162 more
