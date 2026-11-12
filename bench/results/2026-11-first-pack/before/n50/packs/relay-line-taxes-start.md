## Task
# relay-line-taxes · relay-line-taxes · open

## Map
- tallyho/pricing/quote.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/pricing/tax.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- tallyho/workflow.py
- tests/conftest.py
- tests/test_pricing.py
- tallyho/cli.py
- tallyho/config.py
- tallyho/ledger.py

## Decisions & dead ends
- rec:eb7bbb6e attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:5053016a attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:a1725c93 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:8c3cb54f attempt: Replacing the loop in `Money` with a comprehension made it shorter but…

## Code
- sym:7170 Order (tallyho/models.py:64-109)
  class Order:
- sym:7211 quote_order (tallyho/pricing/quote.py:30-53)
- sym:7210 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:7209 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:7169 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:7163 Product (tallyho/models.py:23-35)
  class Product:
- sym:7159 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:7173 Order.remove (tallyho/models.py:86-89)
- sym:7172 Order.add (tallyho/models.py:76-84)
- sym:7140 cmd_products (tallyho/cli.py:20-23)
- sym:7167 OrderLine (tallyho/models.py:51-60)
- sym:7164 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 707/800
omitted 51 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:75bd8806 decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- rec:20aac62f decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:3260fcfd attempt: Moving the imports inside `Money.zero` to speed up start-up saved unde…
- rec:82776653 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:1c5d04fc attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- … and 45 more
