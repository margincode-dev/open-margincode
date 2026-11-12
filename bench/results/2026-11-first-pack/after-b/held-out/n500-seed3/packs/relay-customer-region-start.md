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
- rec:32f93c62 decision: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:4d2f3c14 attempt: Moving the imports inside `ValidationError` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/errors.py:ValidationError
- rec:ea40e92a attempt: Adding type: ignore comments around `ValidationError` to quiet the che…
- rec:9a21fe75 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…

## Code
- sym:40490 Product.__post_init__ (tallyho/models.py:30-35)
- sym:40596 check_region (tallyho/validate.py:24-26)
- sym:40598 parse_quantity (tallyho/validate.py:34-42)
- sym:40594 check_sku (tallyho/validate.py:14-16)
- sym:40595 check_email (tallyho/validate.py:19-21)
- sym:40597 require_positive (tallyho/validate.py:29-31)
- sym:40571 JsonFileStore._path (tallyho/store/jsonfile.py:20-23)
- sym:40504 _exponent (tallyho/money.py:14-18)
- sym:40472 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:40489 Product (tallyho/models.py:23-35)
- sym:40496 Order (tallyho/models.py:64-109)
- sym:40491 Customer (tallyho/models.py:39-47)

---
tokens: 709/800
omitted 179 lower-ranked items (expand an id for detail):
- rec:2f883672 attempt: Replacing the loop in `_exponent` with a comprehension made it shorter…
- rec:78a437db attempt: Replacing the loop in the `errors` module with a comprehension made it…
- rec:c1349c91 attempt: Moving the imports inside `check_sku` to speed up start-up saved under…
- rec:814c9b9d attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:75840457 attempt: Tried calling `check_region` twice to force a refresh; the second call…
- rec:edc2b21c attempt: Replacing the loop in `MemoryStore.put` with a comprehension made it s…
- … and 173 more
