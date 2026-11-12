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
- rec:8d7db426 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:8f93b8b3 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:00234004 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:a02fa0a0 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:0ce2af1d decision: Event payloads carry money as integer minor units plus the currency co…
- rec:05c37cad decision: New optional parameters of `parse_quantity` are keyword-only (after *)…

## Code
- sym:19516 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:19518 total (tallyho/money.py:86-91)
- sym:19506 Money.of (tallyho/money.py:29-33)
- sym:19505 Money (tallyho/money.py:22-83)
  class Money:
- sym:19507 Money.zero (tallyho/money.py:36-37)
- sym:19517 Money.__str__ (tallyho/money.py:82-83)
- sym:19520 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:19541 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:19497 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:19519 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:19534 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 656/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:9305a0bb decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- sym:19485 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:19500 Order.subtotal (tallyho/models.py:92-93)
- sym:19504 _exponent (tallyho/money.py:14-18)
- sym:19514 Money.amount (tallyho/money.py:71-72)
- sym:19614 test_of_and_format (tests/test_money.py:9-12)
