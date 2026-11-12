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
- rec:c9cd7f2e decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:5e760c99 decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:c8dcb09c attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:aa39e02f decision: `product_to_record` keeps its docstring as the one place its behaviour…
- rec:203ce73a decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…

## Code
- sym:38065 Repository.products (tallyho/repository.py:44-46)
- sym:38061 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:37980 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:38097 slugify (tallyho/text.py:8-19)
- sym:37999 Product (tallyho/models.py:23-35)
- sym:38003 OrderLine (tallyho/models.py:51-60)
- sym:38063 Repository.save_product (tallyho/repository.py:38-39)
- sym:38064 Repository.product (tallyho/repository.py:41-42)
- sym:38006 Order (tallyho/models.py:64-109)
- sym:38058 product_from_record (tallyho/repository.py:19-21)
- sym:37979 Config (tallyho/config.py:15-20)
- sym:37976 cmd_products (tallyho/cli.py:20-23)
- sym:37978 main (tallyho/cli.py:31-42)

---
tokens: 699/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:be14c6c8 decision: `CouponBook` keeps its docstring as the one place its behaviour is des…
- rec:4191d77c decision: `MemoryStore.list_page` keeps its docstring as the one place its behav…
- rec:36cd749e decision: New optional parameters of `JsonFileStore.list_page` are keyword-only…
- rec:7e13929a decision: `Coupon.discount` keeps its docstring as the one place its behaviour i…
- rec:8c9ade64 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:087de398 decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- … and 25 more
