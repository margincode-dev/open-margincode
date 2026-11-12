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
- rec:6b480ebb decision: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:071fd580 note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:dde5a914 decision: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:26b99c4b attempt: Tried calling `check_sku` twice to force a refresh; the second call re…

## Code
- sym:37170 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:37152 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:37276 check_region (tallyho/validate.py:24-26)
- sym:37278 parse_quantity (tallyho/validate.py:34-42)
- sym:37274 check_sku (tallyho/validate.py:14-16)
- sym:37275 check_email (tallyho/validate.py:19-21)
- sym:37277 require_positive (tallyho/validate.py:29-31)
- sym:37184 _exponent (tallyho/money.py:14-18)
- sym:37169 Product (tallyho/models.py:23-35)
  class Product:
- sym:37176 Order (tallyho/models.py:64-109)
  class Order:
- sym:37156 NotFound (tallyho/errors.py:24-25)
- sym:37171 Customer (tallyho/models.py:39-47)
  class Customer:

---
tokens: 720/800
omitted 78 lower-ranked items (expand an id for detail):
- rec:ec914ff9 attempt: Replacing the loop in `TallyError` with a comprehension made it shorte…
- rec:a1a0130e attempt: Caching what `Product.__post_init__` returns in a module-level dict br…
- rec:4c5f02a8 attempt: Tried calling `Ledger.post` twice to force a refresh; the second call…
- rec:fa73ce4b attempt: Caching what `JsonFileStore._path` returns in a module-level dict brok…
- rec:024a486e attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:db925c2d attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- … and 72 more
