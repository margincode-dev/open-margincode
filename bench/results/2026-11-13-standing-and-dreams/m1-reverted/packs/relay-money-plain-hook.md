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
- rec:65b4c6e6 decision, open: Event payloads carry money as integer minor units plus the currency code: {"id": ..., "amount_minor": 4681, "currency": "USD"}. Never decimals or strings; consumers are in other languages.
  anchors: tallyho/workflow.py:Workflow
- rec:99ca2c3e decision, reverted: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:eba2fa02 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:5e777f65 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:1af59c6c decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:5ea5bf6f intent, open: Earlier work on this code

## Code
- sym:384 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:386 total (tallyho/money.py:86-91)
- sym:374 Money.of (tallyho/money.py:29-33)
- sym:373 Money (tallyho/money.py:22-83)
  class Money:
- sym:375 Money.zero (tallyho/money.py:36-37)
- sym:385 Money.__str__ (tallyho/money.py:82-83)
  def __str__(self) -> str:
- sym:388 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:409 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:365 Order.currency (tallyho/models.py:73-74)
- sym:387 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:402 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 714/800
omitted 13 lower-ranked items (expand an id for detail):
- rec:e8cbd213 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:f37636a8 note, open: merge a5781bc580c5 reverted by bench-merger in e6d2c0eabd6f: the bench…
- rec:ff93481c decision, reverted: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:76a290cd decision, reverted: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:3f65385b decision, reverted: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- … and 8 more
