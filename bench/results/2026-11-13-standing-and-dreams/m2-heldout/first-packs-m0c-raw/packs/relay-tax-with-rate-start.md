## Task
# relay-tax-with-rate · relay-tax-with-rate · open
Goal: Shops want to try out a rate change before adopting it. Add `TaxTable.with_rate(region, percent)`: it returns a copy of the table in which `region` is taxed at `percent`; the table it is called on stays as it was.
Workspace: /m2b/r3/bench-work/ws/bfp5fc-relay-tax-with-rate-av-1/relay-tax-with-rate
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:1997 Config (tallyho/config.py:15-20)
  class Config:
- sym:1998 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:2017 Product (tallyho/models.py:23-35)
  class Product:
- sym:2066 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:2067 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:
- sym:2024 Order (tallyho/models.py:64-109)
  class Order:
- sym:2068 TaxTable.tax (tallyho/pricing/tax.py:25-26)
  def tax(self, amount: Money, region: str) -> Money:
- sym:2018 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:2016 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:2069 default_tax_table (tallyho/pricing/tax.py:29-39)
  def default_tax_table() -> TaxTable:
- sym:2065 quote_order (tallyho/pricing/quote.py:30-53)
  def quote_order( order: Order, coupons: CouponBook | None = None, taxes: TaxTable | None = None, today: date | None = None, ) -> Quote:
- sym:1994 cmd_products (tallyho/cli.py:20-23)
  def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
- sym:2015 Ledger.trial_balance_ok (tallyho/ledger.py:52-53)
  def trial_balance_ok(self) -> bool:
- sym:2062 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:1996 main (tallyho/cli.py:31-42)
- sym:2005 ConfigError (tallyho/errors.py:28-29)
  class ConfigError(TallyError):
- sym:2025 Order.currency (tallyho/models.py:73-74)
- sym:2053 CouponBook.get (tallyho/pricing/coupons.py:49-50)

---
tokens: 673/800
