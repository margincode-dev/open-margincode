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
- rec:c4b571ec decision: Event payloads carry money as integer minor units plus the currency co…
- rec:c089ed0e attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed :.2f or minor/100 prints 1200 yen as 12.00 and drops a KWD digit. Take the decimals from _exponent(currency).
  anchors: tallyho/money.py:Money.format
- rec:08110842 attempt: Replacing the loop in `Order.currency` with a comprehension made it sh…
- rec:ee70612a attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:93e1cc7f attempt: Replacing the loop in `default_tax_table` with a comprehension made it…
- rec:74909492 attempt: Replacing the loop in `TaxTable.rate` with a comprehension made it sho…

## Code
- sym:36034 Money.format (tallyho/money.py:77-80)
- sym:36036 total (tallyho/money.py:86-91)
- sym:36024 Money.of (tallyho/money.py:29-33)
- sym:36023 Money (tallyho/money.py:22-83)
  class Money:
- sym:36025 Money.zero (tallyho/money.py:36-37)
- sym:36035 Money.__str__ (tallyho/money.py:82-83)
- sym:36038 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:36059 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:36015 Order.currency (tallyho/models.py:73-74)
- sym:36037 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:36052 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:36003 Ledger.record_sale (tallyho/ledger.py:37-44)

---
tokens: 696/800
omitted 30 lower-ranked items (expand an id for detail):
- rec:101505f3 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:ce2be6f0 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:10af9698 attempt: Moving the imports inside `CurrencyMismatch` to speed up start-up save…
- rec:65293cc0 decision: New optional parameters of `Money.format` are keyword-only (after *),…
- rec:61e45916 decision: New optional parameters of `Money.is_zero` are keyword-only (after *),…
- rec:fbd72778 decision: Type hints on `CurrencyMismatch` use the built-in generics (list[str],…
- … and 24 more
