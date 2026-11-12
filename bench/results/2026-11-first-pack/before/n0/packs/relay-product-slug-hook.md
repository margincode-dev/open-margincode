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
- rec:fc78d8eb attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:75ec05f5 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:39c37351 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:48d92c54 decision: Errors about customer data must not echo the rejected value: they are…
- rec:d65d2ba4 decision: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:6067 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:6063 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:5982 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:6099 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:6001 Product (tallyho/models.py:23-35)
  class Product:
- sym:6005 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:6065 Repository.save_product (tallyho/repository.py:38-39)
- sym:6066 Repository.product (tallyho/repository.py:41-42)
- sym:6008 Order (tallyho/models.py:64-109)
  class Order:
- sym:6060 product_from_record (tallyho/repository.py:19-21)
- sym:5981 Config (tallyho/config.py:15-20)
- sym:5978 cmd_products (tallyho/cli.py:20-23)
- sym:5980 main (tallyho/cli.py:31-42)

---
tokens: 660/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:36ef44e6 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:6091a708 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- sym:5989 ConfigError (tallyho/errors.py:28-29)
- sym:6009 Order.currency (tallyho/models.py:73-74)
- sym:6139 test_slugify (tests/test_text.py:4-5)
- sym:6120 book (tests/conftest.py:12-13)
