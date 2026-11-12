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
- rec:9ca51d7a decision: Event payloads carry money as integer minor units plus the currency code: {"id": ..., "amount_minor": 4681, "currency": "USD"}. Never decimals or strings; consumers are in other languages.
  anchors: tallyho/workflow.py:Workflow
- rec:ffe58e13 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:9c9a1ead attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:18c250ff attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:910670c3 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:d54200e3 attempt: Moving the imports inside `CurrencyMismatch` to speed up start-up save…

## Code
- sym:37694 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:37696 total (tallyho/money.py:86-91)
- sym:37684 Money.of (tallyho/money.py:29-33)
- sym:37683 Money (tallyho/money.py:22-83)
  class Money:
- sym:37685 Money.zero (tallyho/money.py:36-37)
- sym:37695 Money.__str__ (tallyho/money.py:82-83)
  def __str__(self) -> str:
- sym:37698 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:37719 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:37675 Order.currency (tallyho/models.py:73-74)
- sym:37697 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:37712 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 693/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:10193631 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:5d7a4890 decision: New optional parameters of `Money.format` are keyword-only (after *),…
- rec:d507586e decision: Log lines from `CurrencyMismatch` go through the logging module, never…
- rec:f778524c decision: Constants that `TaxTable.rate` depends on live at the top of its modul…
- rec:742362d6 decision: Type hints on `Money._check` use the built-in generics (list[str], dic…
- rec:05cbc50c decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- … and 25 more
