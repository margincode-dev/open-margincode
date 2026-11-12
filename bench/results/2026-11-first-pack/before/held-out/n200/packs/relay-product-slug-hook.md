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
- rec:4fb8b8f8 decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:8e9995ab decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:83baf37b decision: `product_to_record` keeps its docstring as the one place its behaviour…
- rec:8cd4b725 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:425c0d35 decision: `CouponBook` keeps its docstring as the one place its behaviour is des…

## Code
- sym:12707 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:12703 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:12622 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:12739 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:12641 Product (tallyho/models.py:23-35)
  class Product:
- sym:12645 OrderLine (tallyho/models.py:51-60)
- sym:12705 Repository.save_product (tallyho/repository.py:38-39)
- sym:12706 Repository.product (tallyho/repository.py:41-42)
- sym:12648 Order (tallyho/models.py:64-109)
  class Order:
- sym:12700 product_from_record (tallyho/repository.py:19-21)
- sym:12621 Config (tallyho/config.py:15-20)
- sym:12618 cmd_products (tallyho/cli.py:20-23)
- sym:12620 main (tallyho/cli.py:31-42)

---
tokens: 654/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:a667e29b decision: `MemoryStore.list_page` keeps its docstring as the one place its behav…
- rec:23926bd7 decision: New optional parameters of `JsonFileStore.list_page` are keyword-only…
- sym:12629 ConfigError (tallyho/errors.py:28-29)
- sym:12649 Order.currency (tallyho/models.py:73-74)
- sym:12779 test_slugify (tests/test_text.py:4-5)
- sym:12760 book (tests/conftest.py:12-13)
