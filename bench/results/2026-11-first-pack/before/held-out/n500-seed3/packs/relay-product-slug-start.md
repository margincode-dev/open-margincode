## Task
# relay-product-slug · relay-product-slug · open
Goal: The web shop needs URL slugs for products. Add a read-only `Product.slug` property.
Workspace: /root/bench-work/ws/bfpadaa-relay-product-slug-av-1/relay-product-slug
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:16023 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:16027 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:15942 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:16026 Repository.product (tallyho/repository.py:41-42)
  def product(self, sku: str) -> Product:
- sym:16059 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:15961 Product (tallyho/models.py:23-35)
  class Product:
- sym:15965 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:16020 product_from_record (tallyho/repository.py:19-21)
  def product_from_record(r: dict) -> Product:
- sym:16025 Repository.save_product (tallyho/repository.py:38-39)
  def save_product(self, p: Product) -> None:
- sym:15968 Order (tallyho/models.py:64-109)
  class Order:
- sym:15970 Order.add (tallyho/models.py:76-84)
  def add(self, product: Product, quantity: int = 1) -> None:
- sym:15941 Config (tallyho/config.py:15-20)
  class Config:
- sym:15938 cmd_products (tallyho/cli.py:20-23)
  def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
- sym:15940 main (tallyho/cli.py:31-42)
  def main(argv: list[str] | None = None) -> int:
- sym:15949 ConfigError (tallyho/errors.py:28-29)
  class ConfigError(TallyError):
- sym:15969 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:

---
tokens: 575/800
