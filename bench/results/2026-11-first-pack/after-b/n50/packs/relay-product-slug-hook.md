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
- rec:9daf9b26 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:672c309a decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:84e9d910 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:4bc9be44 attempt: Wrapping `cmd_products` in a bare try/except with a retry loop hid the…
- rec:63a88253 attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…

## Code
- sym:34745 Repository.products (tallyho/repository.py:44-46)
- sym:34741 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:34660 load_config (tallyho/config.py:23-47)
- sym:34777 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:34679 Product (tallyho/models.py:23-35)
- sym:34683 OrderLine (tallyho/models.py:51-60)
- sym:34743 Repository.save_product (tallyho/repository.py:38-39)
- sym:34744 Repository.product (tallyho/repository.py:41-42)
- sym:34686 Order (tallyho/models.py:64-109)
- sym:34738 product_from_record (tallyho/repository.py:19-21)
- sym:34659 Config (tallyho/config.py:15-20)
- sym:34656 cmd_products (tallyho/cli.py:20-23)
- sym:34658 main (tallyho/cli.py:31-42)

---
tokens: 704/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:59e261bd attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:2eb3f8fa attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:ccdfe919 attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:21c7e79e attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:4589824d decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:da04a4a7 attempt: list_page never returns more than max_page_size (100) records, whateve…
- … and 25 more
