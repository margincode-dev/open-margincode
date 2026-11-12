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
- rec:db18d910 attempt: Moving the imports inside `ValidationError` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/errors.py:ValidationError
- rec:5455f78a attempt: Adding type: ignore comments around `ValidationError` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/errors.py:ValidationError
- rec:ccdeb676 note: Benchmarks for `ValidationError` are in a separate, manual job; the unit tests must not depend on timing.
  anchors: tallyho/errors.py:ValidationError
- rec:9adc9a82 attempt: Replacing the loop in `_exponent` with a comprehension made it shorter…

## Code
- sym:15132 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:15238 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:15240 parse_quantity (tallyho/validate.py:34-42)
- sym:15236 check_sku (tallyho/validate.py:14-16)
- sym:15237 check_email (tallyho/validate.py:19-21)
- sym:15239 require_positive (tallyho/validate.py:29-31)
- sym:15213 JsonFileStore._path (tallyho/store/jsonfile.py:20-23)
- sym:15146 _exponent (tallyho/money.py:14-18)
- sym:15114 ValidationError (tallyho/errors.py:8-17)
- sym:15131 Product (tallyho/models.py:23-35)
  class Product:
- sym:15138 Order (tallyho/models.py:64-109)

---
tokens: 711/800
omitted 180 lower-ranked items (expand an id for detail):
- rec:6c92ee09 attempt: Replacing the loop in the `errors` module with a comprehension made it…
- rec:73631240 attempt: Moving the imports inside `check_sku` to speed up start-up saved under…
- rec:3e9bde2f attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:c06619b9 attempt: Tried calling `check_region` twice to force a refresh; the second call…
- rec:261fb36c attempt: Replacing the loop in `MemoryStore.put` with a comprehension made it s…
- rec:412581d7 attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- … and 174 more
