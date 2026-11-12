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
- rec:c3c04f04 decision: New optional parameters of `Repository.products` are keyword-only (aft…
- rec:62a55f54 decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:1acbcd5a decision: `Repository.products` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/repository.py:Repository.products
- rec:0a6bb4c6 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:157b957b decision: `product_to_record` keeps its docstring as the one place its behaviour…

## Code
- sym:11047 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:11043 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:10962 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:11079 slugify (tallyho/text.py:8-19)
- sym:10981 Product (tallyho/models.py:23-35)
  class Product:
- sym:10985 OrderLine (tallyho/models.py:51-60)
- sym:11045 Repository.save_product (tallyho/repository.py:38-39)
- sym:11046 Repository.product (tallyho/repository.py:41-42)
- sym:10988 Order (tallyho/models.py:64-109)
- sym:11040 product_from_record (tallyho/repository.py:19-21)
- sym:10961 Config (tallyho/config.py:15-20)
- sym:10958 cmd_products (tallyho/cli.py:20-23)
- sym:10960 main (tallyho/cli.py:31-42)

---
tokens: 657/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:bb58a9a9 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:fe0f78eb decision: New optional parameters of `cmd_products` are keyword-only (after *),…
- sym:10969 ConfigError (tallyho/errors.py:28-29)
- sym:10989 Order.currency (tallyho/models.py:73-74)
- sym:11119 test_slugify (tests/test_text.py:4-5)
- sym:11100 book (tests/conftest.py:12-13)
