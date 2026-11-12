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
- rec:f22ddda5 decision: New optional parameters of `Repository.products` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.products
  A positional bool added in the middle of a signature bit us once.
- rec:66ddc66d decision: New optional parameters of `Repository.product` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.product
- rec:93a60161 decision: `Repository.products` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/repository.py:Repository.products
- rec:29cc6463 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:c857a903 decision: `product_to_record` keeps its docstring as the one place its behaviour…
- rec:9b1a17fa decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:e75b922d decision: New optional parameters of `cmd_products` are keyword-only (after *),…

## Code
- sym:9387 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:9383 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:9302 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:9419 slugify (tallyho/text.py:8-19)
- sym:9321 Product (tallyho/models.py:23-35)
- sym:9325 OrderLine (tallyho/models.py:51-60)
- sym:9385 Repository.save_product (tallyho/repository.py:38-39)
- sym:9386 Repository.product (tallyho/repository.py:41-42)
- sym:9328 Order (tallyho/models.py:64-109)
- sym:9380 product_from_record (tallyho/repository.py:19-21)
- sym:9301 Config (tallyho/config.py:15-20)
- sym:9298 cmd_products (tallyho/cli.py:20-23)
- sym:9300 main (tallyho/cli.py:31-42)
- sym:9309 ConfigError (tallyho/errors.py:28-29)
- sym:9329 Order.currency (tallyho/models.py:73-74)
- sym:9459 test_slugify (tests/test_text.py:4-5)
- sym:9440 book (tests/conftest.py:12-13)

---
tokens: 689/800
