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
- rec:c64df5c5 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:8487d333 decision: `Repository.products` keeps its docstring as the one place its behavio…
- rec:0bdf8cb7 attempt: Wrapping `Repository.product` in a bare try/except with a retry loop h…
- rec:d6f22e68 decision: New optional parameters of `Repository.products` are keyword-only (aft…
- rec:19ac6587 decision: New optional parameters of `Repository.product` are keyword-only (afte…

## Code
- sym:36405 Repository.products (tallyho/repository.py:44-46)
- sym:36401 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:36320 load_config (tallyho/config.py:23-47)
- sym:36437 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:36339 Product (tallyho/models.py:23-35)
  class Product:
- sym:36343 OrderLine (tallyho/models.py:51-60)
- sym:36403 Repository.save_product (tallyho/repository.py:38-39)
- sym:36404 Repository.product (tallyho/repository.py:41-42)
- sym:36346 Order (tallyho/models.py:64-109)
- sym:36398 product_from_record (tallyho/repository.py:19-21)
- sym:36319 Config (tallyho/config.py:15-20)
- sym:36316 cmd_products (tallyho/cli.py:20-23)
- sym:36318 main (tallyho/cli.py:31-42)

---
tokens: 693/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:f569f91e decision: `product_to_record` keeps its docstring as the one place its behaviour…
- rec:05a1f56c decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:a2be0baa decision: New optional parameters of `cmd_products` are keyword-only (after *),…
- rec:69f52761 decision: Tests for `slugify` build their inputs with small factory helpers in c…
- rec:b18ac520 decision: `CouponBook` keeps its docstring as the one place its behaviour is des…
- rec:646ca766 decision: `Coupon.usable` keeps its docstring as the one place its behaviour is…
- … and 25 more
