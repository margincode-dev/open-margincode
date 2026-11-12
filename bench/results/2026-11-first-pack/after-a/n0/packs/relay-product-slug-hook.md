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
- rec:863d7c01 attempt: CouponBook stores codes as code.upper() and get() looks them up with code.strip().upper(). Indexing _by_code with the code as typed misses ' save10 ' or 'save10'.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:85719d96 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:8658efcc attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:518c133d decision: Errors about customer data must not echo the rejected value: they are…
- rec:42eee994 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:801b087a decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:b6f5eea5 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…

## Code
- sym:18227 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:18223 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:18142 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:18259 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:18161 Product (tallyho/models.py:23-35)
  class Product:
- sym:18165 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:18225 Repository.save_product (tallyho/repository.py:38-39)
- sym:18226 Repository.product (tallyho/repository.py:41-42)
  def product(self, sku: str) -> Product:
- sym:18168 Order (tallyho/models.py:64-109)
- sym:18220 product_from_record (tallyho/repository.py:19-21)
- sym:18141 Config (tallyho/config.py:15-20)
- sym:18138 cmd_products (tallyho/cli.py:20-23)
- sym:18140 main (tallyho/cli.py:31-42)
- sym:18149 ConfigError (tallyho/errors.py:28-29)
- sym:18169 Order.currency (tallyho/models.py:73-74)
- sym:18299 test_slugify (tests/test_text.py:4-5)
- sym:18280 book (tests/conftest.py:12-13)

---
tokens: 688/800
