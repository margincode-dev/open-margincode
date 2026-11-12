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
- rec:41f38fdb attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:2eb08bcd attempt: Replacing the loop in `Order.currency` with a comprehension made it sh…
- rec:54a94684 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:a486195c decision: New optional parameters of `Money.format` are keyword-only (after *),…
- rec:d4ba7f51 note: `Discount.amount_off` reads files as UTF-8 explicitly; the platform de…
- rec:076efb1d note: The `money` module reads files as UTF-8 explicitly; the platform default encoding differs on the Windows build agents.
  anchors: tallyho/money.py

## Code
- sym:21176 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:21178 total (tallyho/money.py:86-91)
- sym:21166 Money.of (tallyho/money.py:29-33)
- sym:21165 Money (tallyho/money.py:22-83)
  class Money:
- sym:21167 Money.zero (tallyho/money.py:36-37)
- sym:21177 Money.__str__ (tallyho/money.py:82-83)
- sym:21180 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:21201 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:21157 Order.currency (tallyho/models.py:73-74)
- sym:21179 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:21194 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 655/800
omitted 6 lower-ranked items (expand an id for detail):
- rec:14fabc23 note: `CurrencyMismatch` reads files as UTF-8 explicitly; the platform defau…
- sym:21145 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:21160 Order.subtotal (tallyho/models.py:92-93)
- sym:21164 _exponent (tallyho/money.py:14-18)
- sym:21174 Money.amount (tallyho/money.py:71-72)
- sym:21274 test_of_and_format (tests/test_money.py:9-12)
