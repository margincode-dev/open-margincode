## Task
# relay-money-plain · relay-money-plain · open
Goal: Report tables want amounts without the currency code. Add `Money.format_plain()`, returning the amount as text without the currency code, for example "12.34" for 12.34 USD.
Workspace: /root/bench-work/ws/bfpa928-relay-money-plain-av-1/relay-money-plain
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:13996 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:13998 total (tallyho/money.py:86-91)
  def total(amounts, currency: str = "USD") -> Money:
- sym:13986 Money.of (tallyho/money.py:29-33)
  def of(cls, amount: str | int | Decimal, currency: str = "USD") -> Money:
- sym:13985 Money (tallyho/money.py:22-83)
  class Money:
- sym:13987 Money.zero (tallyho/money.py:36-37)
  def zero(cls, currency: str = "USD") -> Money:
- sym:13997 Money.__str__ (tallyho/money.py:82-83)
  def __str__(self) -> str:
- sym:14000 Coupon.discount (tallyho/pricing/coupons.py:23-28)
  def discount(self, currency: str = "USD") -> Discount:
- sym:14021 default_tax_table (tallyho/pricing/tax.py:29-39)
  def default_tax_table() -> TaxTable:
- sym:13977 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:13999 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:14014 apply_discounts (tallyho/pricing/discounts.py:46-67)
  def apply_discounts(subtotal: Money, discounts: list[Discount]) -> tuple[Money, list[tuple[str, Money]]]:
- sym:13965 Ledger.record_sale (tallyho/ledger.py:37-44)
  def record_sale(self, on: date, order_id: str, quote: Quote) -> None:
- sym:13980 Order.subtotal (tallyho/models.py:92-93)
  def subtotal(self) -> Money:
- sym:13984 _exponent (tallyho/money.py:14-18)
  def _exponent(currency: str) -> int:
- sym:13994 Money.amount (tallyho/money.py:71-72)
  def amount(self) -> Decimal:
- sym:14094 test_of_and_format (tests/test_money.py:9-12)
  def test_of_and_format():

---
tokens: 658/800
