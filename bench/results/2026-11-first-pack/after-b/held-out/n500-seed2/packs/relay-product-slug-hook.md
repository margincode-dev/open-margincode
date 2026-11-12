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
- rec:4b1bd5d3 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:2e17ca3b decision: New optional parameters of `product_from_record` are keyword-only (aft…
- rec:401b7669 attempt: Wrapping `Repository.product` in a bare try/except with a retry loop h…
- rec:fa9b9e14 decision: `Order.add` keeps its docstring as the one place its behaviour is desc…
- rec:81475098 decision: The `discounts` module keeps its docstring as the one place its behavi…

## Code
- sym:39725 Repository.products (tallyho/repository.py:44-46)
- sym:39721 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:39640 load_config (tallyho/config.py:23-47)
- sym:39757 slugify (tallyho/text.py:8-19)
- sym:39659 Product (tallyho/models.py:23-35)
  class Product:
- sym:39663 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:39723 Repository.save_product (tallyho/repository.py:38-39)
- sym:39724 Repository.product (tallyho/repository.py:41-42)
- sym:39666 Order (tallyho/models.py:64-109)
- sym:39718 product_from_record (tallyho/repository.py:19-21)
- sym:39639 Config (tallyho/config.py:15-20)
- sym:39636 cmd_products (tallyho/cli.py:20-23)
- sym:39638 main (tallyho/cli.py:31-42)

---
tokens: 707/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:ee8e891b decision: `CouponBook` keeps its docstring as the one place its behaviour is des…
- rec:0ee1c2f1 decision: `RecordStore.get` keeps its docstring as the one place its behaviour i…
- rec:2b0b2b46 decision: Constants that `slugify` depends on live at the top of its module in U…
- rec:c0f43265 decision: New optional parameters of `CouponBook.get` are keyword-only (after *)…
- rec:5e36a88c decision: `OrderLine` keeps its docstring as the one place its behaviour is desc…
- rec:8610e3ce decision: `Customer.__post_init__` keeps its docstring as the one place its beha…
- … and 25 more
