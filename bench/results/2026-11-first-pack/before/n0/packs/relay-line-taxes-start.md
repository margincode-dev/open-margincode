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
- rec:4701242c attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:f68886d5 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:78abef5e decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:65b76169 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:5551 quote_order (tallyho/pricing/quote.py:30-53)
- sym:5510 Order (tallyho/models.py:64-109)
  class Order:
- sym:5550 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:5549 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:5513 Order.remove (tallyho/models.py:86-89)
- sym:5512 Order.add (tallyho/models.py:76-84)
- sym:5565 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:5509 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:5511 Order.currency (tallyho/models.py:73-74)
- sym:5514 Order.subtotal (tallyho/models.py:92-93)
- sym:5519 Money (tallyho/money.py:22-83)
- sym:5532 total (tallyho/money.py:86-91)

---
tokens: 710/800
omitted 31 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- sym:5536 CouponBook (tallyho/pricing/coupons.py:40-63)
- sym:5540 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:5548 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:5552 TaxTable (tallyho/pricing/tax.py:12-26)
- sym:5554 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:5555 default_tax_table (tallyho/pricing/tax.py:29-39)
- … and 24 more
