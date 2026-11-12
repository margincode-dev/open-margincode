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
- rec:85a9797b decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable line. The per-line amounts add up to Quote.tax, so no splitting or remainder handling is needed.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:0dc1e150 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:7bd21e1a decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:789e9757 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:42156 Order (tallyho/models.py:64-109)
  class Order:
- sym:42197 quote_order (tallyho/pricing/quote.py:30-53)
- sym:42196 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:42195 Quote (tallyho/pricing/quote.py:16-27)
- sym:42155 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:42149 Product (tallyho/models.py:23-35)
- sym:42145 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:42159 Order.remove (tallyho/models.py:86-89)
- sym:42158 Order.add (tallyho/models.py:76-84)
- sym:42126 cmd_products (tallyho/cli.py:20-23)
- sym:42153 OrderLine (tallyho/models.py:51-60)
- sym:42150 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 708/800
omitted 35 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- sym:42154 OrderLine.__post_init__ (tallyho/models.py:55-56)
- sym:42211 Repository (tallyho/repository.py:32-74)
- sym:42157 Order.currency (tallyho/models.py:73-74)
- sym:42160 Order.subtotal (tallyho/models.py:92-93)
- sym:42165 Money (tallyho/money.py:22-83)
- sym:42178 total (tallyho/money.py:86-91)
- sym:42182 CouponBook (tallyho/pricing/coupons.py:40-63)
- … and 27 more
