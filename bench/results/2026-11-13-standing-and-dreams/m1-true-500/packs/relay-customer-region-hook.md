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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:5d95b770 decision, open: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:ac4ac827 note, open: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:3f6d7f4e decision, open: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:b4cc3abd decision, open: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:5278 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:5276 check_sku (tallyho/validate.py:14-16)
  def check_sku(sku: str) -> None:
- sym:5172 Product.__post_init__ (tallyho/models.py:30-35)
- sym:5280 parse_quantity (tallyho/validate.py:34-42)
- sym:5277 check_email (tallyho/validate.py:19-21)
- sym:5279 require_positive (tallyho/validate.py:29-31)
- sym:5154 ValidationError (tallyho/errors.py:8-17)
- sym:5171 Product (tallyho/models.py:23-35)
  class Product:
- sym:5178 Order (tallyho/models.py:64-109)
- sym:5275 _REGION (tallyho/validate.py:11-11)
- sym:5173 Customer (tallyho/models.py:39-47)

---
tokens: 700/800
omitted 185 lower-ranked items (expand an id for detail):
- rec:5ef4dd09 note, open: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:b784b75d attempt, open: Wrapping `ValidationError` in a bare try/except with a retry loop hid…
- rec:5897ec07 decision, open: Tests for `check_region` build their inputs with small factory helpers…
- rec:a3ba176d decision, open: Constants that `Customer.__post_init__` depends on live at the top of…
- rec:676949e6 note, open: `Workflow.place` reads files as UTF-8 explicitly; the platform default…
- … and 180 more
