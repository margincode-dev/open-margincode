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
- sym:37276 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:37274 check_sku (tallyho/validate.py:14-16)
  def check_sku(sku: str) -> None:
- sym:37170 Product.__post_init__ (tallyho/models.py:30-35)
- sym:37278 parse_quantity (tallyho/validate.py:34-42)
- sym:37275 check_email (tallyho/validate.py:19-21)
- sym:37277 require_positive (tallyho/validate.py:29-31)
- sym:37152 ValidationError (tallyho/errors.py:8-17)
- sym:37169 Product (tallyho/models.py:23-35)
  class Product:
- sym:37176 Order (tallyho/models.py:64-109)
- sym:37273 _REGION (tallyho/validate.py:11-11)
- sym:37171 Customer (tallyho/models.py:39-47)
- sym:37249 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 710/800
omitted 104 lower-ranked items (expand an id for detail):
- rec:797c7ed1 attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:5cdc5806 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:edb946da attempt: Wrapping `cmd_products` in a bare try/except with a retry loop hid the…
- rec:65fe953a decision: Constants that the `coupons` module depends on live at the top of its…
- rec:970fdb32 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:89aae5d1 note: `JsonFileStore.put` reads files as UTF-8 explicitly; the platform defa…
- … and 98 more
