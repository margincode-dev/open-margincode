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
- rec:cbd3c1cd decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:3a1ecb43 decision: `product_from_record` keeps its docstring as the one place its behavio…
- rec:4b0264ec decision: New optional parameters of `product_to_record` are keyword-only (after…
- rec:c5c86ac1 decision: `RecordStore.list_page` keeps its docstring as the one place its behav…
- rec:d8ad34c6 decision: New optional parameters of `CouponBook.get` are keyword-only (after *)…

## Code
- sym:41385 Repository.products (tallyho/repository.py:44-46)
- sym:41381 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:41300 load_config (tallyho/config.py:23-47)
- sym:41417 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:41319 Product (tallyho/models.py:23-35)
- sym:41323 OrderLine (tallyho/models.py:51-60)
- sym:41383 Repository.save_product (tallyho/repository.py:38-39)
- sym:41384 Repository.product (tallyho/repository.py:41-42)
- sym:41326 Order (tallyho/models.py:64-109)
- sym:41378 product_from_record (tallyho/repository.py:19-21)
- sym:41299 Config (tallyho/config.py:15-20)
- sym:41296 cmd_products (tallyho/cli.py:20-23)
- sym:41298 main (tallyho/cli.py:31-42)

---
tokens: 695/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:36030062 decision: `RecordStore.put` keeps its docstring as the one place its behaviour i…
- rec:015ac330 decision: New optional parameters of `Page` are keyword-only (after *), so call…
- rec:64b1809d decision: New optional parameters of `CouponBook` are keyword-only (after *), so…
- rec:d2ed8553 decision: `Order` keeps its docstring as the one place its behaviour is describe…
- rec:2c5fc47e decision: `Repository.customer` keeps its docstring as the one place its behavio…
- rec:f0c89362 decision: `MemoryStore.put` keeps its docstring as the one place its behaviour i…
- … and 25 more
