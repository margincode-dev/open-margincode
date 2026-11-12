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
- rec:5c0d6c33 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:436499f3 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:fd114f5d attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:02991b70 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:67c37869 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:c4807775 decision: Errors about customer data must not echo the rejected value: they are…
- rec:40dd97ce decision: A refund is booked as the exact reverse of the sale: the same three ac…

## Code
- sym:17856 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:17858 total (tallyho/money.py:86-91)
  def total(amounts, currency: str = "USD") -> Money:
- sym:17846 Money.of (tallyho/money.py:29-33)
  def of(cls, amount: str | int | Decimal, currency: str = "USD") -> Money:
- sym:17845 Money (tallyho/money.py:22-83)
  class Money:
- sym:17847 Money.zero (tallyho/money.py:36-37)
  def zero(cls, currency: str = "USD") -> Money:
- sym:17857 Money.__str__ (tallyho/money.py:82-83)
- sym:17860 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:17881 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:17837 Order.currency (tallyho/models.py:73-74)
- sym:17859 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:17874 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:17825 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:17840 Order.subtotal (tallyho/models.py:92-93)
- sym:17844 _exponent (tallyho/money.py:14-18)
- sym:17854 Money.amount (tallyho/money.py:71-72)
- sym:17954 test_of_and_format (tests/test_money.py:9-12)

---
tokens: 684/800
