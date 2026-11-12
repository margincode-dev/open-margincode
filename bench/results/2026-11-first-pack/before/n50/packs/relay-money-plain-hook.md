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
- rec:43653a8a attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:afaba360 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:3ab00fe8 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:99e9dbbd attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:443ab5c5 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:b183004d decision: New optional parameters of `parse_quantity` are keyword-only (after *)…

## Code
- sym:7356 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:7358 total (tallyho/money.py:86-91)
- sym:7346 Money.of (tallyho/money.py:29-33)
- sym:7345 Money (tallyho/money.py:22-83)
  class Money:
- sym:7347 Money.zero (tallyho/money.py:36-37)
- sym:7357 Money.__str__ (tallyho/money.py:82-83)
- sym:7360 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:7381 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:7337 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:7359 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:7374 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 655/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:e6204cb1 decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- sym:7325 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:7340 Order.subtotal (tallyho/models.py:92-93)
- sym:7344 _exponent (tallyho/money.py:14-18)
- sym:7354 Money.amount (tallyho/money.py:71-72)
- sym:7454 test_of_and_format (tests/test_money.py:9-12)
