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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:d8d28c6e decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:0d83d6d9 decision, open: `Repository.products` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/repository.py:Repository.products
- rec:4bd23973 attempt, open: Wrapping `Repository.product` in a bare try/except with a retry loop h…
- rec:8258552c decision, open: New optional parameters of `Repository.products` are keyword-only (aft…

## Code
- sym:6067 Repository.products (tallyho/repository.py:44-46)
- sym:6063 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:5982 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:6099 slugify (tallyho/text.py:8-19)
- sym:6001 Product (tallyho/models.py:23-35)
  class Product:
- sym:6005 OrderLine (tallyho/models.py:51-60)
- sym:6065 Repository.save_product (tallyho/repository.py:38-39)
- sym:6066 Repository.product (tallyho/repository.py:41-42)
- sym:6008 Order (tallyho/models.py:64-109)
- sym:6060 product_from_record (tallyho/repository.py:19-21)
- sym:5981 Config (tallyho/config.py:15-20)
- sym:5978 cmd_products (tallyho/cli.py:20-23)

---
tokens: 689/800
omitted 33 lower-ranked items (expand an id for detail):
- rec:aa8ff60b decision, open: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:d8324773 decision, open: `product_to_record` keeps its docstring as the one place its behaviour…
- rec:2ce10cb6 decision, open: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:f15ee813 decision, open: New optional parameters of `cmd_products` are keyword-only (after *),…
- rec:6e51d6ff decision, open: Tests for `slugify` build their inputs with small factory helpers in c…
- … and 28 more
