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
- rec:02638c0c decision: New optional parameters of `Repository.product` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.product
- rec:a8f8e161 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:93304404 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:1e5f2bc3 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:07f44232 decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- rec:d579b74a decision: New optional parameters of the `discounts` module are keyword-only (af…
- rec:7794f129 decision: `JsonFileStore._save` keeps its docstring as the one place its behavio…

## Code
- sym:19887 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:19883 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:19802 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:19919 slugify (tallyho/text.py:8-19)
- sym:19821 Product (tallyho/models.py:23-35)
  class Product:
- sym:19825 OrderLine (tallyho/models.py:51-60)
- sym:19885 Repository.save_product (tallyho/repository.py:38-39)
- sym:19886 Repository.product (tallyho/repository.py:41-42)
- sym:19828 Order (tallyho/models.py:64-109)
- sym:19880 product_from_record (tallyho/repository.py:19-21)
- sym:19801 Config (tallyho/config.py:15-20)
- sym:19798 cmd_products (tallyho/cli.py:20-23)
- sym:19800 main (tallyho/cli.py:31-42)
- sym:19809 ConfigError (tallyho/errors.py:28-29)
- sym:19829 Order.currency (tallyho/models.py:73-74)
- sym:19959 test_slugify (tests/test_text.py:4-5)
- sym:19940 book (tests/conftest.py:12-13)

---
tokens: 688/800
