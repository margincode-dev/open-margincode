## Task
# relay-product-slug · relay-product-slug · open
Goal: The web shop needs URL slugs for products. Add a read-only `Product.slug` property.
Workspace: /root/bench-work/ws/bfp971f-relay-product-slug-av-1/relay-product-slug
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:7642 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:7641 Config (tallyho/config.py:15-20)
  class Config:
- sym:7665 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:7668 Order (tallyho/models.py:64-109)
  class Order:
- sym:7661 Product (tallyho/models.py:23-35)
  class Product:
- sym:7723 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:7727 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:7662 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:7725 Repository.save_product (tallyho/repository.py:38-39)
  def save_product(self, p: Product) -> None:
- sym:7638 cmd_products (tallyho/cli.py:20-23)
  def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
- sym:7726 Repository.product (tallyho/repository.py:41-42)
  def product(self, sku: str) -> Product:
- sym:7640 main (tallyho/cli.py:31-42)
  def main(argv: list[str] | None = None) -> int:
- sym:7649 ConfigError (tallyho/errors.py:28-29)
  class ConfigError(TallyError):
- sym:7669 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:7697 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:7711 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:

---
tokens: 581/800
