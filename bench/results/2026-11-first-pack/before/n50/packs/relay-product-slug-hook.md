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
- rec:40e184d1 decision: New optional parameters of `Repository.product` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.product
- rec:aa43449d decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:0dbd9961 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:31724d00 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:eb5d44aa decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- rec:a2a81166 decision: New optional parameters of the `discounts` module are keyword-only (af…
- rec:4ee75f40 decision: `JsonFileStore._save` keeps its docstring as the one place its behavio…

## Code
- sym:7727 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:7723 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:7642 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:7759 slugify (tallyho/text.py:8-19)
- sym:7661 Product (tallyho/models.py:23-35)
  class Product:
- sym:7665 OrderLine (tallyho/models.py:51-60)
- sym:7725 Repository.save_product (tallyho/repository.py:38-39)
- sym:7726 Repository.product (tallyho/repository.py:41-42)
- sym:7668 Order (tallyho/models.py:64-109)
- sym:7720 product_from_record (tallyho/repository.py:19-21)
- sym:7641 Config (tallyho/config.py:15-20)
- sym:7638 cmd_products (tallyho/cli.py:20-23)
- sym:7640 main (tallyho/cli.py:31-42)
- sym:7649 ConfigError (tallyho/errors.py:28-29)
- sym:7669 Order.currency (tallyho/models.py:73-74)
- sym:7799 test_slugify (tests/test_text.py:4-5)
- sym:7780 book (tests/conftest.py:12-13)

---
tokens: 688/800
