## Task
# relay-coupon-remove · relay-coupon-remove · open

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:329df61c decision: CouponBook keys _by_code by the code exactly as it is passed in. Lookups and removals index _by_code with the code as typed; callers have normalized it already.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:3f628779 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:a227e068 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:8fac4e42 decision: Errors about customer data must not echo the rejected value: they are…
- rec:e83d32f2 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:4642861a decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…

## Code
- sym:42016 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:42019 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:42018 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:42017 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:42020 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:42095 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:42013 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:42031 quote_order (tallyho/pricing/quote.py:30-53)
- sym:42015 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:42115 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 696/800
omitted 9 lower-ranked items (expand an id for detail):
- rec:6095d6ff decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:38bd0c24 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:b1fc359d decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:fa954db8 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:1109c298 intent: Earlier work on this code
- sym:41964 load_config (tallyho/config.py:23-47)
- sym:41975 EventBus.publish (tallyho/events.py:19-22)
- … and 2 more
