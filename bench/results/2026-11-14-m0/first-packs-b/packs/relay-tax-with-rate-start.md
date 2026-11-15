## Task
# relay-tax-with-rate · relay-tax-with-rate · open
Goal: Shops want to try out a rate change before adopting it. Add `TaxTable.with_rate(region, percent)`: it returns a copy of the table in which `region` is taxed at `percent`; the table it is called on stays as it was.

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
- sym:337 Config (tallyho/config.py:15-20)
  class Config:
- sym:338 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:357 Product (tallyho/models.py:23-35)
  class Product:
- sym:406 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:407 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:
- sym:364 Order (tallyho/models.py:64-109)
  class Order:
- sym:408 TaxTable.tax (tallyho/pricing/tax.py:25-26)
  def tax(self, amount: Money, region: str) -> Money:
- sym:358 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:356 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:409 default_tax_table (tallyho/pricing/tax.py:29-39)
  def default_tax_table() -> TaxTable:
- sym:405 quote_order (tallyho/pricing/quote.py:30-53)
- sym:334 cmd_products (tallyho/cli.py:20-23)
  def cmd_products(repo: Repository, args: argparse.Namespace) -> int:
- sym:355 Ledger.trial_balance_ok (tallyho/ledger.py:52-53)
  def trial_balance_ok(self) -> bool:
- sym:402 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:336 main (tallyho/cli.py:31-42)
- sym:345 ConfigError (tallyho/errors.py:28-29)
- sym:365 Order.currency (tallyho/models.py:73-74)
- sym:393 CouponBook.get (tallyho/pricing/coupons.py:49-50)

---
tokens: 678/800
