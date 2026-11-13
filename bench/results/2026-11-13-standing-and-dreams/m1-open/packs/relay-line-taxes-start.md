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
- rec:ce1b20ff decision, open: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable line. The per-line amounts add up to Quote.tax, so no splitting or remainder handling is needed.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:af4ad3f2 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:f6406e89 decision, open: CouponBook keys _by_code by the code exactly as it is passed in. Looku…

## Code
- sym:1028 Order (tallyho/models.py:64-109)
  class Order:
- sym:1069 quote_order (tallyho/pricing/quote.py:30-53)
- sym:1068 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:1067 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:1027 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:1021 Product (tallyho/models.py:23-35)
  class Product:
- sym:1017 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:1031 Order.remove (tallyho/models.py:86-89)
- sym:1030 Order.add (tallyho/models.py:76-84)
- sym:998 cmd_products (tallyho/cli.py:20-23)
- sym:1025 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:1022 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 713/800
omitted 36 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:99d76e56 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:1026 OrderLine.__post_init__ (tallyho/models.py:55-56)
- sym:1083 Repository (tallyho/repository.py:32-74)
- sym:1029 Order.currency (tallyho/models.py:73-74)
- sym:1032 Order.subtotal (tallyho/models.py:92-93)
- sym:1037 Money (tallyho/money.py:22-83)
- … and 29 more
