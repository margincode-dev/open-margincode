## Task
# relay-quote-split · relay-quote-split · open

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
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:ff0bdb96 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:03cb7ccc attempt: Tried calling `check_sku` twice to force a refresh; the second call re…
- rec:f0628077 attempt: Replacing the loop in `TallyError` with a comprehension made it shorte…
- rec:60016376 attempt: Tried calling `Ledger.post` twice to force a refresh; the second call…

## Code
- sym:12831 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:12823 Money (tallyho/money.py:22-83)
  class Money:
- sym:12801 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:12802 Ledger.post (tallyho/ledger.py:28-35)
- sym:12848 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:12790 ValidationError (tallyho/errors.py:8-17)
- sym:12814 Order (tallyho/models.py:64-109)
  class Order:
- sym:12826 Money._check (tallyho/money.py:39-43)
- sym:12828 Money.__sub__ (tallyho/money.py:49-51)
- sym:12847 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:12827 Money.__add__ (tallyho/money.py:45-47)

---
tokens: 723/800
omitted 83 lower-ranked items (expand an id for detail):
- rec:83c33f57 attempt: Caching what `Product.__post_init__` returns in a module-level dict br…
- rec:978bd135 attempt: Caching what `JsonFileStore._path` returns in a module-level dict brok…
- rec:3d2a6624 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:0c00ee17 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:a6773da1 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:8a6a8f90 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- … and 77 more
