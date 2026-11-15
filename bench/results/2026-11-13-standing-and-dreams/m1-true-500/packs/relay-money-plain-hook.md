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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:fa4452b7 decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:e76d771b attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed :.2f or minor/100 prints 1200 yen as 12.00 and drops a KWD digit. Take the decimals from _exponent(currency).
  anchors: tallyho/money.py:Money.format
- rec:b2697605 attempt, open: Replacing the loop in `Order.currency` with a comprehension made it sh…
- rec:c05dfd62 attempt, open: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:dcba74cf attempt, open: Replacing the loop in `default_tax_table` with a comprehension made it…
- rec:e095b10d attempt, open: Replacing the loop in `TaxTable.rate` with a comprehension made it sho…

## Code
- sym:5696 Money.format (tallyho/money.py:77-80)
- sym:5698 total (tallyho/money.py:86-91)
- sym:5686 Money.of (tallyho/money.py:29-33)
- sym:5685 Money (tallyho/money.py:22-83)
  class Money:
- sym:5687 Money.zero (tallyho/money.py:36-37)
- sym:5697 Money.__str__ (tallyho/money.py:82-83)
- sym:5700 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:5721 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:5677 Order.currency (tallyho/models.py:73-74)
- sym:5699 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:5680 Order.subtotal (tallyho/models.py:92-93)

---
tokens: 694/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:78ae2ee9 attempt, open: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:f81a3c7e attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:2800e831 attempt, open: Moving the imports inside `CurrencyMismatch` to speed up start-up save…
- rec:b58ffa87 decision, open: New optional parameters of `Money.format` are keyword-only (after *),…
- rec:14025ce5 decision, open: New optional parameters of `Money.is_zero` are keyword-only (after *),…
- … and 26 more
