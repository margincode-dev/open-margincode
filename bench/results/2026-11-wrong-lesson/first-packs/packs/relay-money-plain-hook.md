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
- rec:65c61bd2 decision: Every currency tallyho supports has two decimals. Plain amounts are minor / 100 formatted with :.2f; _exponent(currency) only matters for the symbol layout in format().
  anchors: tallyho/money.py:Money.format
- rec:729468c5 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:2dc89814 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:f9b1fb82 decision: Errors about customer data must not echo the rejected value: they are…
- rec:179c6de6 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:8a7f10fb decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:b7c0edb8 intent: Earlier work on this code

## Code
- sym:42342 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:42344 total (tallyho/money.py:86-91)
- sym:42332 Money.of (tallyho/money.py:29-33)
- sym:42331 Money (tallyho/money.py:22-83)
  class Money:
- sym:42333 Money.zero (tallyho/money.py:36-37)
- sym:42343 Money.__str__ (tallyho/money.py:82-83)
- sym:42346 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:42367 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:42323 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:42345 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:42360 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 694/800
omitted 9 lower-ranked items (expand an id for detail):
- rec:366cbbea decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:461d13ed decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:55433de9 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:a92575e2 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:42311 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:42326 Order.subtotal (tallyho/models.py:92-93)
- … and 3 more
