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
- rec:da4f71aa decision: Event payloads carry money as integer minor units plus the currency co…
- rec:ca6f8ef2 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed :.2f or minor/100 prints 1200 yen as 12.00 and drops a KWD digit. Take the decimals from _exponent(currency).
  anchors: tallyho/money.py:Money.format
- rec:24d5a4de attempt: Replacing the loop in `Money.zero` with a comprehension made it shorte…
- rec:433c7ec7 attempt: Wrapping `CurrencyMismatch` in a bare try/except with a retry loop hid…
- rec:0d23f68a attempt: Replacing the loop in `Money._check` with a comprehension made it shor…
- rec:225a5940 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…

## Code
- sym:41014 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:41016 total (tallyho/money.py:86-91)
- sym:41004 Money.of (tallyho/money.py:29-33)
- sym:41003 Money (tallyho/money.py:22-83)
  class Money:
- sym:41005 Money.zero (tallyho/money.py:36-37)
- sym:41015 Money.__str__ (tallyho/money.py:82-83)
- sym:41018 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:41039 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:40995 Order.currency (tallyho/models.py:73-74)
- sym:41017 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:41032 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 691/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:6ad3c8a3 attempt: Wrapping `Money.__sub__` in a bare try/except with a retry loop hid th…
- rec:94b9a7fa attempt: Wrapping `Money._check` in a bare try/except with a retry loop hid the…
- rec:4ca86c7a attempt: Wrapping `Money.__neg__` in a bare try/except with a retry loop hid th…
- rec:b5494f0b attempt: Adding type: ignore comments around `Money.format` to quiet the checke…
- rec:5d2f4f51 attempt: Tried calling `Money.format` twice to force a refresh; the second call…
- … and 26 more
