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
- rec:22b2ca83 decision: Event payloads carry money as integer minor units plus the currency code: {"id": ..., "amount_minor": 4681, "currency": "USD"}. Never decimals or strings; consumers are in other languages.
  anchors: tallyho/workflow.py:Workflow
- rec:96c3e4b8 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:8476e293 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:49c7ce5f attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:f43b0c55 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:fd85bed6 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…

## Code
- sym:34374 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:34376 total (tallyho/money.py:86-91)
- sym:34364 Money.of (tallyho/money.py:29-33)
- sym:34363 Money (tallyho/money.py:22-83)
  class Money:
- sym:34365 Money.zero (tallyho/money.py:36-37)
- sym:34375 Money.__str__ (tallyho/money.py:82-83)
- sym:34378 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:34399 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:34355 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:34377 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:34392 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 703/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:c6314167 attempt: Moving the imports inside `Money.zero` to speed up start-up saved unde…
- rec:a88ef4fd attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:865c1d41 attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:189ad83d attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:bdb0b338 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:9247a582 attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- … and 25 more
