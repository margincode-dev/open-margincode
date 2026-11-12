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
- rec:86742451 decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:8598 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:8596 check_sku (tallyho/validate.py:14-16)
  def check_sku(sku: str) -> None:
- sym:8492 Product.__post_init__ (tallyho/models.py:30-35)
- sym:8600 parse_quantity (tallyho/validate.py:34-42)
- sym:8597 check_email (tallyho/validate.py:19-21)
- sym:8599 require_positive (tallyho/validate.py:29-31)
- sym:8474 ValidationError (tallyho/errors.py:8-17)
- sym:8491 Product (tallyho/models.py:23-35)
- sym:8498 Order (tallyho/models.py:64-109)
- sym:8595 _REGION (tallyho/validate.py:11-11)
- sym:8493 Customer (tallyho/models.py:39-47)
- sym:8571 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 711/800
omitted 168 lower-ranked items (expand an id for detail):
- rec:04022dc3 decision: Tests for `check_region` build their inputs with small factory helpers…
- rec:976e6312 note: The `validate` module reads files as UTF-8 explicitly; the platform de…
- rec:d28eaf2b note: Deprecation warnings from `Customer` are turned into errors in CI (fil…
- rec:5cc5e3da note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:08b6aebf note: Deprecation warnings from `customer_from_record` are turned into error…
- rec:2fca4c77 attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- … and 162 more
