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
- rec:965d5e1b attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:7fc18f9d attempt: Replacing the loop in `Order.currency` with a comprehension made it sh…
- rec:5055e5b3 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:f5e6bf5a decision: New optional parameters of `Money.format` are keyword-only (after *),…
- rec:d0eda796 note: `Discount.amount_off` reads files as UTF-8 explicitly; the platform de…
- rec:2755f348 note: The `money` module reads files as UTF-8 explicitly; the platform defau…

## Code
- sym:10676 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:10678 total (tallyho/money.py:86-91)
- sym:10666 Money.of (tallyho/money.py:29-33)
- sym:10665 Money (tallyho/money.py:22-83)
  class Money:
- sym:10667 Money.zero (tallyho/money.py:36-37)
- sym:10677 Money.__str__ (tallyho/money.py:82-83)
- sym:10680 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:10701 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:10657 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:10679 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:10694 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 654/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:ad8cd157 note: `CurrencyMismatch` reads files as UTF-8 explicitly; the platform defau…
- sym:10645 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:10660 Order.subtotal (tallyho/models.py:92-93)
- sym:10664 _exponent (tallyho/money.py:14-18)
- sym:10674 Money.amount (tallyho/money.py:71-72)
- sym:10774 test_of_and_format (tests/test_money.py:9-12)
