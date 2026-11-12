## Task
# relay-product-slug · relay-product-slug · open
Goal: The web shop needs URL slugs for products. Add a read-only `Product.slug` property.
Workspace: /root/bench-work/ws/bfp92cd-relay-product-slug-av-1/relay-product-slug
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:5982 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:5981 Config (tallyho/config.py:15-20)
  class Config:
- sym:6005 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:6008 Order (tallyho/models.py:64-109)
  class Order:
- sym:6001 Product (tallyho/models.py:23-35)
  class Product:
- sym:6063 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:6067 Repository.products (tallyho/repository.py:44-46)
  def products(self, category: str | None = None) -> list[Product]:
- sym:6002 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:6065 Repository.save_product (tallyho/repository.py:38-39)
  def save_product(self, p: Product) -> None:
- sym:5978 cmd_products (tallyho/cli.py:20-23)
  def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
- sym:6066 Repository.product (tallyho/repository.py:41-42)
  def product(self, sku: str) -> Product:
- sym:5980 main (tallyho/cli.py:31-42)
  def main(argv: list[str] | None = None) -> int:
- sym:5989 ConfigError (tallyho/errors.py:28-29)
  class ConfigError(TallyError):
- sym:6009 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:6037 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:6051 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:

---
tokens: 581/800
