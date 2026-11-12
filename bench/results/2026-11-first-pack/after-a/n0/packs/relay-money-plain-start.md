## Task
# relay-money-plain · relay-money-plain · open
Goal: Report tables want amounts without the currency code. Add `Money.format_plain()`, returning the amount as text without the currency code, for example "12.34" for 12.34 USD.

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:17823 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:17856 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:17815 CurrencyMismatch (tallyho/errors.py:20-21)
  class CurrencyMismatch(TallyError):
- sym:17858 total (tallyho/money.py:86-91)
  def total(amounts, currency: str = "USD") -> Money:
- sym:17809 Config (tallyho/config.py:15-20)
  class Config:
- sym:17845 Money (tallyho/money.py:22-83)
  class Money:
- sym:17825 Ledger.record_sale (tallyho/ledger.py:37-44)
  def record_sale(self, on: date, order_id: str, quote: Quote) -> None:
- sym:17846 Money.of (tallyho/money.py:29-33)
  def of(cls, amount: str | int | Decimal, currency: str = "USD") -> Money:
- sym:17826 Ledger.balances (tallyho/ledger.py:46-50)
  def balances(self) -> dict[str, Money]:
- sym:17829 Product (tallyho/models.py:23-35)
  class Product:
- sym:17860 Coupon.discount (tallyho/pricing/coupons.py:23-28)
  def discount(self, currency: str = "USD") -> Discount:
- sym:17824 Ledger.post (tallyho/ledger.py:28-35)
  def post(self, on: date, memo: str, *legs: tuple[str, Money, Money]) -> None:
- sym:17859 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:17810 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:17874 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:17837 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:17840 Order.subtotal (tallyho/models.py:92-93)
  def subtotal(self) -> Money:
- sym:17844 _exponent (tallyho/money.py:14-18)
- sym:17847 Money.zero (tallyho/money.py:36-37)
- sym:17954 test_of_and_format (tests/test_money.py:9-12)
  def test_of_and_format():

---
tokens: 676/800
