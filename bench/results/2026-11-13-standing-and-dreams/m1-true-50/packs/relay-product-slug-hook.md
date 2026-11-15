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
- rec:0d226b9e decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug" -> "mug-1-blue-mug". Names alone collide (two SKUs called "Blue mug"), so the SKU always leads.
  anchors: tallyho/models.py:Product
- rec:715dfc9a decision, open: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:c2a849b2 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:d50aee5a attempt, open: Wrapping `cmd_products` in a bare try/except with a retry loop hid the…
- rec:a09eecb2 decision, open: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:4407 Repository.products (tallyho/repository.py:44-46)
- sym:4403 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:4322 load_config (tallyho/config.py:23-47)
- sym:4439 slugify (tallyho/text.py:8-19)
- sym:4341 Product (tallyho/models.py:23-35)
  class Product:
- sym:4345 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:4405 Repository.save_product (tallyho/repository.py:38-39)
- sym:4406 Repository.product (tallyho/repository.py:41-42)
- sym:4348 Order (tallyho/models.py:64-109)
- sym:4400 product_from_record (tallyho/repository.py:19-21)
- sym:4321 Config (tallyho/config.py:15-20)
- sym:4318 cmd_products (tallyho/cli.py:20-23)

---
tokens: 699/800
omitted 32 lower-ranked items (expand an id for detail):
- rec:8c213a1e attempt, open: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:cd9457be attempt, open: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:c31b2708 attempt, open: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:c0f055e0 attempt, open: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:6387231b attempt, open: Replacing the loop in `Money` with a comprehension made it shorter but…
- … and 27 more
