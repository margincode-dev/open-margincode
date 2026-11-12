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
- rec:d0a91d38 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:e7101a95 decision: Errors about customer data must not echo the rejected value: they are…
- rec:ad4bfb23 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:3fced398 note: The `validate` module reads files as UTF-8 explicitly; the platform default encoding differs on the Windows build agents.
  anchors: tallyho/validate.py

## Code
- sym:11918 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:11916 check_sku (tallyho/validate.py:14-16)
  def check_sku(sku: str) -> None:
- sym:11812 Product.__post_init__ (tallyho/models.py:30-35)
- sym:11920 parse_quantity (tallyho/validate.py:34-42)
- sym:11917 check_email (tallyho/validate.py:19-21)
- sym:11919 require_positive (tallyho/validate.py:29-31)
- sym:11794 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:11811 Product (tallyho/models.py:23-35)
- sym:11818 Order (tallyho/models.py:64-109)
- sym:11915 _REGION (tallyho/validate.py:11-11)
- sym:11813 Customer (tallyho/models.py:39-47)
- sym:11891 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 713/800
omitted 83 lower-ranked items (expand an id for detail):
- rec:85bc3cc2 note: Deprecation warnings from the `money` module are turned into errors in…
- rec:1cd5cdbe note: Deprecation warnings from `Customer` are turned into errors in CI (fil…
- rec:d35b4689 note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:5c01565d attempt: Tried calling `check_sku` twice to force a refresh; the second call re…
- rec:088fd0a4 attempt: Replacing the loop in `TallyError` with a comprehension made it shorte…
- rec:06283143 attempt: Caching what `Product.__post_init__` returns in a module-level dict br…
- … and 77 more
