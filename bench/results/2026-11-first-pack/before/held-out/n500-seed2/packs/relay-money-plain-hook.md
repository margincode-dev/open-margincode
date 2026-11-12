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
- rec:6fb6f503 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:438ae370 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:aa8f4ac6 decision: New optional parameters of `Money.format` are keyword-only (after *),…
- rec:de3cfd69 decision: New optional parameters of `Money.amount` are keyword-only (after *),…
- rec:3ef8a889 decision: Constants that `Money.amount` depends on live at the top of its module…
- rec:76614666 decision: Constants that `Money.zero` depends on live at the top of its module i…

## Code
- sym:13996 Money.format (tallyho/money.py:77-80)
- sym:13998 total (tallyho/money.py:86-91)
- sym:13986 Money.of (tallyho/money.py:29-33)
- sym:13985 Money (tallyho/money.py:22-83)
  class Money:
- sym:13987 Money.zero (tallyho/money.py:36-37)
- sym:13997 Money.__str__ (tallyho/money.py:82-83)
- sym:14000 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:14021 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:13977 Order.currency (tallyho/models.py:73-74)
- sym:13999 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:14014 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:13965 Ledger.record_sale (tallyho/ledger.py:37-44)

---
tokens: 629/800
omitted 5 lower-ranked items (expand an id for detail):
- rec:78090c23 decision: New optional parameters of `CurrencyMismatch` are keyword-only (after…
- sym:13980 Order.subtotal (tallyho/models.py:92-93)
- sym:13984 _exponent (tallyho/money.py:14-18)
- sym:13994 Money.amount (tallyho/money.py:71-72)
- sym:14094 test_of_and_format (tests/test_money.py:9-12)
