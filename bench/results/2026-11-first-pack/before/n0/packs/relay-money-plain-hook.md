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

## Decisions & dead ends
- rec:063642e9 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:f63708da attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:e2df13ed attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:f2b9b53e attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:22cc0671 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:bcd6302e decision: Errors about customer data must not echo the rejected value: they are…
- rec:5442a73d decision: A refund is booked as the exact reverse of the sale: the same three ac…

## Code
- sym:5696 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:5698 total (tallyho/money.py:86-91)
  def total(amounts, currency: str = "USD") -> Money:
- sym:5686 Money.of (tallyho/money.py:29-33)
  def of(cls, amount: str | int | Decimal, currency: str = "USD") -> Money:
- sym:5685 Money (tallyho/money.py:22-83)
  class Money:
- sym:5687 Money.zero (tallyho/money.py:36-37)
  def zero(cls, currency: str = "USD") -> Money:
- sym:5697 Money.__str__ (tallyho/money.py:82-83)
- sym:5700 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:5721 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:5677 Order.currency (tallyho/models.py:73-74)
- sym:5699 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:5714 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:5665 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:5680 Order.subtotal (tallyho/models.py:92-93)
- sym:5684 _exponent (tallyho/money.py:14-18)
- sym:5694 Money.amount (tallyho/money.py:71-72)
- sym:5794 test_of_and_format (tests/test_money.py:9-12)

---
tokens: 687/800
