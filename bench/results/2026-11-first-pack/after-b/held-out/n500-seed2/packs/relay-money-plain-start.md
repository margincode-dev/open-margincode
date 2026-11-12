## Task
# relay-money-plain · relay-money-plain · open
Goal: Report tables want amounts without the currency code. Add `Money.format_plain()`, returning the amount as text without the currency code, for example "12.34" for 12.34 USD.
Workspace: /root/bench-work/ws/bfpa75c-relay-money-plain-av-1/relay-money-plain
Changed (0): nothing yet

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
- sym:39354 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:39356 total (tallyho/money.py:86-91)
  def total(amounts, currency: str = "USD") -> Money:
- sym:39343 Money (tallyho/money.py:22-83)
  class Money:
- sym:39344 Money.of (tallyho/money.py:29-33)
  def of(cls, amount: str | int | Decimal, currency: str = "USD") -> Money:
- sym:39358 Coupon.discount (tallyho/pricing/coupons.py:23-28)
  def discount(self, currency: str = "USD") -> Discount:
- sym:39357 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:39372 apply_discounts (tallyho/pricing/discounts.py:46-67)
  def apply_discounts(subtotal: Money, discounts: list[Discount]) -> tuple[Money, list[tuple[str, Money]]]:
- sym:39323 Ledger.record_sale (tallyho/ledger.py:37-44)
  def record_sale(self, on: date, order_id: str, quote: Quote) -> None:
- sym:39335 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:39338 Order.subtotal (tallyho/models.py:92-93)
  def subtotal(self) -> Money:
- sym:39342 _exponent (tallyho/money.py:14-18)
  def _exponent(currency: str) -> int:
- sym:39452 test_of_and_format (tests/test_money.py:9-12)
  def test_of_and_format():

---
tokens: 643/800
