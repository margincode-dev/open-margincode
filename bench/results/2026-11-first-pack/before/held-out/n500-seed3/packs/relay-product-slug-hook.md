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
- rec:aaa6ea5b decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:737fb803 decision: `product_from_record` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/repository.py:product_from_record
  The README is for operators, not API detail.
- rec:2ea72c36 decision: New optional parameters of `product_to_record` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:product_to_record
- rec:5db4a0c7 decision: `RecordStore.list_page` keeps its docstring as the one place its behav…
- rec:6f7da234 decision: New optional parameters of `CouponBook.get` are keyword-only (after *)…
- rec:93f3fad6 decision: `RecordStore.put` keeps its docstring as the one place its behaviour i…
- rec:35cddc64 note: `slugify` reads files as UTF-8 explicitly; the platform default encodi…

## Code
- sym:16027 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:16023 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:15942 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:16059 slugify (tallyho/text.py:8-19)
- sym:15961 Product (tallyho/models.py:23-35)
  class Product:
- sym:15965 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:16025 Repository.save_product (tallyho/repository.py:38-39)
- sym:16026 Repository.product (tallyho/repository.py:41-42)
- sym:15968 Order (tallyho/models.py:64-109)
- sym:16020 product_from_record (tallyho/repository.py:19-21)
- sym:15941 Config (tallyho/config.py:15-20)
- sym:15938 cmd_products (tallyho/cli.py:20-23)
- sym:15940 main (tallyho/cli.py:31-42)
- sym:15949 ConfigError (tallyho/errors.py:28-29)
- sym:15969 Order.currency (tallyho/models.py:73-74)
- sym:16099 test_slugify (tests/test_text.py:4-5)
- sym:16080 book (tests/conftest.py:12-13)

---
tokens: 687/800
