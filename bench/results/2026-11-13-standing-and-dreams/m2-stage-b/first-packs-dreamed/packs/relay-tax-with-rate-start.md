## Task
# relay-tax-with-rate · relay-tax-with-rate · open
Goal: Shops want to try out a rate change before adopting it. Add `TaxTable.with_rate(region, percent)`: it returns a copy of the table in which `region` is taxed at `percent`; the table it is called on stays as it was.
Workspace: /m2b/bench-work/ws/bfp3fe-relay-tax-with-rate-av-1/relay-tax-with-rate
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:1499 Config (tallyho/config.py:15-20)
  class Config:
- sym:1500 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:1519 Product (tallyho/models.py:23-35)
  class Product:
- sym:1568 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:1569 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:
- sym:1526 Order (tallyho/models.py:64-109)
  class Order:
- sym:1570 TaxTable.tax (tallyho/pricing/tax.py:25-26)
  def tax(self, amount: Money, region: str) -> Money:
- sym:1520 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:1518 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:1571 default_tax_table (tallyho/pricing/tax.py:29-39)
  def default_tax_table() -> TaxTable:
- sym:1567 quote_order (tallyho/pricing/quote.py:30-53)
  def quote_order( order: Order, coupons: CouponBook | None = None, taxes: TaxTable | None = None, today: date | None = None, ) -> Quote:
- sym:1496 cmd_products (tallyho/cli.py:20-23)
  def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
- sym:1517 Ledger.trial_balance_ok (tallyho/ledger.py:52-53)
  def trial_balance_ok(self) -> bool:
- sym:1564 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:1498 main (tallyho/cli.py:31-42)
  def main(argv: list[str] | None = None) -> int:
- sym:1507 ConfigError (tallyho/errors.py:28-29)
- sym:1527 Order.currency (tallyho/models.py:73-74)
- sym:1555 CouponBook.get (tallyho/pricing/coupons.py:49-50)

---
tokens: 679/800
