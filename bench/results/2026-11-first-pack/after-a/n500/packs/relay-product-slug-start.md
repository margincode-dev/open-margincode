## Task
# relay-product-slug · relay-product-slug · open
Goal: The web shop needs URL slugs for products. Add a read-only `Product.slug` property.
Workspace: /root/bench-work/ws/bfpd911-relay-product-slug-av-1/relay-product-slug
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:21543 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:21547 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:21462 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:21546 Repository.product (tallyho/repository.py:41-42)
  def product(self, sku: str) -> Product:
- sym:21579 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:21481 Product (tallyho/models.py:23-35)
  class Product:
- sym:21485 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:21540 product_from_record (tallyho/repository.py:19-21)
  def product_from_record(r: dict) -> Product:
- sym:21545 Repository.save_product (tallyho/repository.py:38-39)
  def save_product(self, p: Product) -> None:
- sym:21488 Order (tallyho/models.py:64-109)
  class Order:
- sym:21490 Order.add (tallyho/models.py:76-84)
  def add(self, product: Product, quantity: int = 1) -> None:
- sym:21461 Config (tallyho/config.py:15-20)
  class Config:
- sym:21458 cmd_products (tallyho/cli.py:20-23)
  def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
- sym:21460 main (tallyho/cli.py:31-42)
  def main(argv: list[str] | None = None) -> int:
- sym:21469 ConfigError (tallyho/errors.py:28-29)
  class ConfigError(TallyError):
- sym:21489 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:

---
tokens: 575/800
