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
- rec:e80319b4 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:a1b2b16a decision: Event payloads carry money as integer minor units plus the currency co…
- rec:4dbeb85e decision: New optional parameters of `Money.is_zero` are keyword-only (after *),…
- rec:9c28b3ca note: `Money.zero` reads files as UTF-8 explicitly; the platform default enc…
- rec:913092c3 note: `Discount.amount_off` reads files as UTF-8 explicitly; the platform de…
- rec:151ab280 note: Deprecation warnings from `Money.amount` are turned into errors in CI…

## Code
- sym:15656 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:15658 total (tallyho/money.py:86-91)
- sym:15646 Money.of (tallyho/money.py:29-33)
- sym:15645 Money (tallyho/money.py:22-83)
  class Money:
- sym:15647 Money.zero (tallyho/money.py:36-37)
- sym:15657 Money.__str__ (tallyho/money.py:82-83)
- sym:15660 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:15681 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:15637 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:15659 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:15674 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 656/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:f5621c79 note: `Money._check` reads files as UTF-8 explicitly; the platform default e…
- sym:15625 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:15640 Order.subtotal (tallyho/models.py:92-93)
- sym:15644 _exponent (tallyho/money.py:14-18)
- sym:15654 Money.amount (tallyho/money.py:71-72)
- sym:15754 test_of_and_format (tests/test_money.py:9-12)
