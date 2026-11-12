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
- rec:bce600a9 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:c96831fa attempt: Replacing the loop in `Order.currency` with a comprehension made it sh…
- rec:2e3fa61b decision: Event payloads carry money as integer minor units plus the currency co…
- rec:7cac1cad decision: New optional parameters of `Money.format` are keyword-only (after *),…
- rec:4edff89c note: `Discount.amount_off` reads files as UTF-8 explicitly; the platform de…
- rec:488ca64a note: The `money` module reads files as UTF-8 explicitly; the platform default encoding differs on the Windows build agents.
  anchors: tallyho/money.py

## Code
- sym:9016 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:9018 total (tallyho/money.py:86-91)
- sym:9006 Money.of (tallyho/money.py:29-33)
- sym:9005 Money (tallyho/money.py:22-83)
  class Money:
- sym:9007 Money.zero (tallyho/money.py:36-37)
- sym:9017 Money.__str__ (tallyho/money.py:82-83)
- sym:9020 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:9041 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:8997 Order.currency (tallyho/models.py:73-74)
- sym:9019 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:9034 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 657/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:de06d4ff note: `CurrencyMismatch` reads files as UTF-8 explicitly; the platform defau…
- sym:8985 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:9000 Order.subtotal (tallyho/models.py:92-93)
- sym:9004 _exponent (tallyho/money.py:14-18)
- sym:9014 Money.amount (tallyho/money.py:71-72)
- sym:9114 test_of_and_format (tests/test_money.py:9-12)
