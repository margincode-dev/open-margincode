## Task
# relay-product-slug · relay-product-slug · open
Goal: The web shop needs URL slugs for products. Add a read-only `Product.slug` property.
Workspace: /root/bench-work/ws/bfp9e4e-relay-product-slug-av-1/relay-product-slug
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
    __init__.py
    cli.py
    config.py
    errors.py
    events.py
    ledger.py
    models.py
    money.py
    pricing/ (5 files)
    report/ (3 files)
    repository.py
    store/ (5 files)
    text.py
    validate.py
    workflow.py
  tests/ (7 files)
    conftest.py
    test_models.py
    test_money.py
    test_pricing.py
    test_report.py
    test_store.py
    test_text.py

## Code
- sym:36320 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:36401 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:36405 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:36343 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:36403 Repository.save_product (tallyho/repository.py:38-39)
  def save_product(self, p: Product) -> None:
- sym:36404 Repository.product (tallyho/repository.py:41-42)
  def product(self, sku: str) -> Product:
- sym:36346 Order (tallyho/models.py:64-109)
  class Order:
- sym:36319 Config (tallyho/config.py:15-20)
  class Config:
- sym:36316 cmd_products (tallyho/cli.py:20-23)
  def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
- sym:36318 main (tallyho/cli.py:31-42)
  def main(argv: list[str] | None = None) -> int:
- sym:36327 ConfigError (tallyho/errors.py:28-29)
  class ConfigError(TallyError):
- sym:36339 Product (tallyho/models.py:23-35)
  class Product:

---
tokens: 562/800
