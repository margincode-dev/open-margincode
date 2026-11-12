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
- rec:61c99223 decision: New optional parameters of `Repository.products` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.products
  A positional bool added in the middle of a signature bit us once.
- rec:cd30642e decision: New optional parameters of `Repository.product` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.product
- rec:d88655a0 decision: `Repository.products` keeps its docstring as the one place its behavio…
- rec:a8fbea48 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:1e13d4b4 decision: `product_to_record` keeps its docstring as the one place its behaviour…
- rec:b90451d4 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:d6c3db0b decision: New optional parameters of `cmd_products` are keyword-only (after *),…

## Code
- sym:21547 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:21543 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:21462 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:21579 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:21481 Product (tallyho/models.py:23-35)
- sym:21485 OrderLine (tallyho/models.py:51-60)
- sym:21545 Repository.save_product (tallyho/repository.py:38-39)
- sym:21546 Repository.product (tallyho/repository.py:41-42)
- sym:21488 Order (tallyho/models.py:64-109)
- sym:21540 product_from_record (tallyho/repository.py:19-21)
- sym:21461 Config (tallyho/config.py:15-20)
- sym:21458 cmd_products (tallyho/cli.py:20-23)
- sym:21460 main (tallyho/cli.py:31-42)
- sym:21469 ConfigError (tallyho/errors.py:28-29)
- sym:21489 Order.currency (tallyho/models.py:73-74)
- sym:21619 test_slugify (tests/test_text.py:4-5)
- sym:21600 book (tests/conftest.py:12-13)

---
tokens: 685/800
