## Task
# relay-money-plain · relay-money-plain · open

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:9522bd97 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:2440ae13 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed :.2f or minor/100 prints 1200 yen as 12.00 and drops a KWD digit. Take the decimals from _exponent(currency).
  anchors: tallyho/money.py:Money.format
- rec:e8184bcb attempt: Replacing the loop in `Money.is_zero` with a comprehension made it sho…
- rec:5e1f362a attempt: Wrapping `Money.amount` in a bare try/except with a retry loop hid the…
- rec:e72674b5 attempt: Replacing the loop in `default_tax_table` with a comprehension made it…
- rec:8226f781 attempt: Replacing the loop in `Money.of` with a comprehension made it shorter…

## Code
- sym:39354 Money.format (tallyho/money.py:77-80)
- sym:39356 total (tallyho/money.py:86-91)
- sym:39344 Money.of (tallyho/money.py:29-33)
- sym:39343 Money (tallyho/money.py:22-83)
  class Money:
- sym:39345 Money.zero (tallyho/money.py:36-37)
- sym:39355 Money.__str__ (tallyho/money.py:82-83)
- sym:39358 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:39379 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:39335 Order.currency (tallyho/models.py:73-74)
- sym:39357 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:39372 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:39338 Order.subtotal (tallyho/models.py:92-93)

---
tokens: 700/800
omitted 30 lower-ranked items (expand an id for detail):
- rec:5f9197bc attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:cbab48b3 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:773fcdfe decision: New optional parameters of `Money.format` are keyword-only (after *),…
- rec:368fb04e decision: New optional parameters of `Money.amount` are keyword-only (after *),…
- rec:bd999e7d decision: Constants that `Money.amount` depends on live at the top of its module…
- rec:fc8d965e decision: Constants that `Money.zero` depends on live at the top of its module i…
- … and 24 more
