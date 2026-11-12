## Task
# relay-product-slug · relay-product-slug · open
Goal: The web shop needs URL slugs for products. Add a read-only `Product.slug` property.
Workspace: /root/bench-work/ws/bfpa01f-relay-product-slug-av-1/relay-product-slug
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:11043 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:11047 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:10962 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:11046 Repository.product (tallyho/repository.py:41-42)
  def product(self, sku: str) -> Product:
- sym:11079 slugify (tallyho/text.py:8-19)
  def slugify(text: str) -> str:
- sym:10981 Product (tallyho/models.py:23-35)
  class Product:
- sym:10985 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:11040 product_from_record (tallyho/repository.py:19-21)
  def product_from_record(r: dict) -> Product:
- sym:11045 Repository.save_product (tallyho/repository.py:38-39)
  def save_product(self, p: Product) -> None:
- sym:10988 Order (tallyho/models.py:64-109)
  class Order:
- sym:10990 Order.add (tallyho/models.py:76-84)
  def add(self, product: Product, quantity: int = 1) -> None:
- sym:10961 Config (tallyho/config.py:15-20)
  class Config:
- sym:10958 cmd_products (tallyho/cli.py:20-23)
  def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
- sym:10960 main (tallyho/cli.py:31-42)
  def main(argv: list[str] | None = None) -> int:
- sym:10969 ConfigError (tallyho/errors.py:28-29)
  class ConfigError(TallyError):
- sym:10989 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:

---
tokens: 576/800
