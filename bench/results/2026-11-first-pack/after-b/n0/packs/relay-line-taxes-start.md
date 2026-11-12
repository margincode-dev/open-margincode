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
- rec:0b59f4e0 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:b6dcafb3 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:c53bc982 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:91728260 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:32569 quote_order (tallyho/pricing/quote.py:30-53)
- sym:32528 Order (tallyho/models.py:64-109)
  class Order:
- sym:32568 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:32567 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:32531 Order.remove (tallyho/models.py:86-89)
- sym:32530 Order.add (tallyho/models.py:76-84)
- sym:32583 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:32527 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:32529 Order.currency (tallyho/models.py:73-74)
- sym:32532 Order.subtotal (tallyho/models.py:92-93)
- sym:32537 Money (tallyho/money.py:22-83)
- sym:32550 total (tallyho/money.py:86-91)

---
tokens: 711/800
omitted 31 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- sym:32554 CouponBook (tallyho/pricing/coupons.py:40-63)
- sym:32558 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:32566 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:32570 TaxTable (tallyho/pricing/tax.py:12-26)
- sym:32572 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:32573 default_tax_table (tallyho/pricing/tax.py:29-39)
- … and 24 more
