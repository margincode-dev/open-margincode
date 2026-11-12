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
- rec:6fef7a17 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:0ab0fe10 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:aa187a93 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:3937e3e5 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:17670 Order (tallyho/models.py:64-109)
  class Order:
- sym:17711 quote_order (tallyho/pricing/quote.py:30-53)
- sym:17710 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:17709 Quote (tallyho/pricing/quote.py:16-27)
- sym:17669 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:17663 Product (tallyho/models.py:23-35)
- sym:17659 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:17673 Order.remove (tallyho/models.py:86-89)
- sym:17672 Order.add (tallyho/models.py:76-84)
- sym:17640 cmd_products (tallyho/cli.py:20-23)
- sym:17667 OrderLine (tallyho/models.py:51-60)
- sym:17664 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 709/800
omitted 35 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- sym:17668 OrderLine.__post_init__ (tallyho/models.py:55-56)
- sym:17725 Repository (tallyho/repository.py:32-74)
- sym:17671 Order.currency (tallyho/models.py:73-74)
- sym:17674 Order.subtotal (tallyho/models.py:92-93)
- sym:17679 Money (tallyho/money.py:22-83)
- sym:17692 total (tallyho/money.py:86-91)
- sym:17696 CouponBook (tallyho/pricing/coupons.py:40-63)
- … and 27 more
