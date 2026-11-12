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
- rec:5c01565d attempt: Tried calling `check_sku` twice to force a refresh; the second call re…
- rec:088fd0a4 attempt: Replacing the loop in `TallyError` with a comprehension made it shorte…
- rec:06283143 attempt: Caching what `Product.__post_init__` returns in a module-level dict br…

## Code
- sym:11812 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:11794 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:11918 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:11920 parse_quantity (tallyho/validate.py:34-42)
  def parse_quantity(text: str) -> int:
- sym:11916 check_sku (tallyho/validate.py:14-16)
- sym:11917 check_email (tallyho/validate.py:19-21)
- sym:11919 require_positive (tallyho/validate.py:29-31)
- sym:11826 _exponent (tallyho/money.py:14-18)
- sym:11811 Product (tallyho/models.py:23-35)
- sym:11818 Order (tallyho/models.py:64-109)
- sym:11798 NotFound (tallyho/errors.py:24-25)
- sym:11813 Customer (tallyho/models.py:39-47)

---
tokens: 720/800
omitted 78 lower-ranked items (expand an id for detail):
- rec:ee0abc91 attempt: Tried calling `Ledger.post` twice to force a refresh; the second call…
- rec:9b8a65c7 attempt: Caching what `JsonFileStore._path` returns in a module-level dict brok…
- rec:e2234e1c attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:8880bbda attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:e65f6043 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:17fdd945 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- … and 72 more
