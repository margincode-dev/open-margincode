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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:be1df648 attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:1dbecf72 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:2c7743b9 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…

## Code
- sym:2190 Order (tallyho/models.py:64-109)
  class Order:
- sym:2231 quote_order (tallyho/pricing/quote.py:30-53)
- sym:2230 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:2229 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:2189 OrderLine.subtotal (tallyho/models.py:59-60)
  def subtotal(self) -> Money:
- sym:2183 Product (tallyho/models.py:23-35)
- sym:2179 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:2193 Order.remove (tallyho/models.py:86-89)
- sym:2192 Order.add (tallyho/models.py:76-84)
- sym:2160 cmd_products (tallyho/cli.py:20-23)
- sym:2187 OrderLine (tallyho/models.py:51-60)
- sym:2184 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 713/800
omitted 36 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:72620cd8 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:2188 OrderLine.__post_init__ (tallyho/models.py:55-56)
- sym:2245 Repository (tallyho/repository.py:32-74)
- sym:2191 Order.currency (tallyho/models.py:73-74)
- sym:2194 Order.subtotal (tallyho/models.py:92-93)
- sym:2199 Money (tallyho/money.py:22-83)
- … and 29 more
