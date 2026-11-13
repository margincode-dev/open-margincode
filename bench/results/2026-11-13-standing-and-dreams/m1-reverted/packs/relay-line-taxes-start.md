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
- rec:e0e8f06d decision, open: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:1dd0c297 decision, reverted: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:c9d2cf73 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:198 Order (tallyho/models.py:64-109)
  class Order:
- sym:239 quote_order (tallyho/pricing/quote.py:30-53)
- sym:238 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:237 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:197 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:191 Product (tallyho/models.py:23-35)
  class Product:
- sym:187 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:201 Order.remove (tallyho/models.py:86-89)
- sym:200 Order.add (tallyho/models.py:76-84)
- sym:168 cmd_products (tallyho/cli.py:20-23)
- sym:195 OrderLine (tallyho/models.py:51-60)
- sym:192 Product.__post_init__ (tallyho/models.py:30-35)
- sym:196 OrderLine.__post_init__ (tallyho/models.py:55-56)

---
tokens: 725/800
omitted 35 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:26f1c373 decision, reverted: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- sym:253 Repository (tallyho/repository.py:32-74)
- sym:199 Order.currency (tallyho/models.py:73-74)
- sym:202 Order.subtotal (tallyho/models.py:92-93)
- sym:207 Money (tallyho/money.py:22-83)
- sym:220 total (tallyho/money.py:86-91)
- sym:224 CouponBook (tallyho/pricing/coupons.py:40-63)
- … and 27 more
