## Task
# relay-product-slug · relay-product-slug · open
Goal: The web shop needs URL slugs for products. Add a read-only `Product.slug` property.

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:fe34ec7e decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:abfdd5b0 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:9111edb9 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:0878031b attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:21f86d84 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:33085 Repository.products (tallyho/repository.py:44-46)
- sym:33081 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:33000 load_config (tallyho/config.py:23-47)
- sym:33117 slugify (tallyho/text.py:8-19)
- sym:33019 Product (tallyho/models.py:23-35)
  class Product:
- sym:33023 OrderLine (tallyho/models.py:51-60)
- sym:33083 Repository.save_product (tallyho/repository.py:38-39)
- sym:33084 Repository.product (tallyho/repository.py:41-42)
- sym:33026 Order (tallyho/models.py:64-109)
  class Order:
- sym:33078 product_from_record (tallyho/repository.py:19-21)
- sym:32999 Config (tallyho/config.py:15-20)
- sym:32996 cmd_products (tallyho/cli.py:20-23)
- sym:32998 main (tallyho/cli.py:31-42)

---
tokens: 710/800
omitted 10 lower-ranked items (expand an id for detail):
- rec:7a05dcbc decision: Errors about customer data must not echo the rejected value: they are…
- rec:b9e4094c attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:56d6b837 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:b5c8e3de decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:e06e71ef decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:6ae32ddd intent: Earlier work on this code
- sym:33007 ConfigError (tallyho/errors.py:28-29)
- … and 3 more
