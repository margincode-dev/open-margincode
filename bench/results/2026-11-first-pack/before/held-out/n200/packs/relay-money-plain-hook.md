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
- rec:0802c857 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:2c2c6d6a attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:8306d169 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:e5320cef decision: New optional parameters of `Money.format` are keyword-only (after *),…
- rec:53f031a8 decision: Log lines from `CurrencyMismatch` go through the logging module, never…
- rec:57a4ca95 decision: Constants that `TaxTable.rate` depends on live at the top of its modul…

## Code
- sym:12336 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:12338 total (tallyho/money.py:86-91)
- sym:12326 Money.of (tallyho/money.py:29-33)
- sym:12325 Money (tallyho/money.py:22-83)
  class Money:
- sym:12327 Money.zero (tallyho/money.py:36-37)
- sym:12337 Money.__str__ (tallyho/money.py:82-83)
- sym:12340 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:12361 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:12317 Order.currency (tallyho/models.py:73-74)
- sym:12339 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:12354 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 652/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:ea1c17c6 note: The `money` module reads files as UTF-8 explicitly; the platform defau…
- sym:12305 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:12320 Order.subtotal (tallyho/models.py:92-93)
- sym:12324 _exponent (tallyho/money.py:14-18)
- sym:12334 Money.amount (tallyho/money.py:71-72)
- sym:12434 test_of_and_format (tests/test_money.py:9-12)
