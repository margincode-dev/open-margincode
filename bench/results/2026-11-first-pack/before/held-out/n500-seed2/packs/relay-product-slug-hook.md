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
- rec:01f8816b decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:c174c75c decision: New optional parameters of `product_from_record` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:product_from_record
- rec:5c2a4f70 decision: `Order.add` keeps its docstring as the one place its behaviour is desc…
- rec:aa86e135 decision: The `discounts` module keeps its docstring as the one place its behavi…
- rec:b6b81105 decision: `CouponBook` keeps its docstring as the one place its behaviour is des…
- rec:393f3d92 decision: `RecordStore.get` keeps its docstring as the one place its behaviour i…
- rec:f91ff1e9 note: `slugify` reads files as UTF-8 explicitly; the platform default encodi…

## Code
- sym:14367 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:14363 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:14282 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:14399 slugify (tallyho/text.py:8-19)
- sym:14301 Product (tallyho/models.py:23-35)
  class Product:
- sym:14305 OrderLine (tallyho/models.py:51-60)
- sym:14365 Repository.save_product (tallyho/repository.py:38-39)
- sym:14366 Repository.product (tallyho/repository.py:41-42)
- sym:14308 Order (tallyho/models.py:64-109)
- sym:14360 product_from_record (tallyho/repository.py:19-21)
- sym:14281 Config (tallyho/config.py:15-20)
- sym:14278 cmd_products (tallyho/cli.py:20-23)
- sym:14280 main (tallyho/cli.py:31-42)
- sym:14289 ConfigError (tallyho/errors.py:28-29)
- sym:14309 Order.currency (tallyho/models.py:73-74)
- sym:14439 test_slugify (tests/test_text.py:4-5)
- sym:14420 book (tests/conftest.py:12-13)

---
tokens: 689/800
