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
- rec:8b2e2c29 decision, open: Every currency tallyho supports has two decimals. Plain amounts are minor / 100 formatted with :.2f; _exponent(currency) only matters for the symbol layout in format().
  anchors: tallyho/money.py:Money.format
- rec:f54f8edd decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:dde6bbd4 decision, open: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:ec6ed50a decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:726d8573 decision, open: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:0fa4f519 intent, open: Earlier work on this code

## Code
- sym:1214 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:1216 total (tallyho/money.py:86-91)
- sym:1204 Money.of (tallyho/money.py:29-33)
- sym:1203 Money (tallyho/money.py:22-83)
  class Money:
- sym:1205 Money.zero (tallyho/money.py:36-37)
- sym:1215 Money.__str__ (tallyho/money.py:82-83)
- sym:1218 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:1239 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:1195 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:1217 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:1232 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 706/800
omitted 10 lower-ranked items (expand an id for detail):
- rec:ec7a7313 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:640bc41c decision, open: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:b73c52f9 decision, open: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:2bc01d86 decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:79a0020c decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- … and 5 more
