## Task
# relay-coupon-remove · relay-coupon-remove · open
Goal: Shops need to withdraw coupons. Add `CouponBook.remove(code)`: afterwards `get` no longer finds that coupon, and removing a code the book does not know does nothing.

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:0f7b3d72 decision, reverted: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:fb4894b3 validation, reverted: `bench` exit 0
- rec:cc20e28e decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:1587299c decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:61d59dc2 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:17338dfe decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:58 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:61 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:60 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:59 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:62 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:137 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:55 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:73 quote_order (tallyho/pricing/quote.py:30-53)
- sym:57 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:157 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 704/800
omitted 12 lower-ranked items (expand an id for detail):
- rec:02b7956c decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:f9632a9a note, open: merge f65237534293 reverted by bench-merger in 6e1912b575b5: the bench…
- rec:ab47114c decision, reverted: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:4aed762c decision, reverted: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:0b982c5a decision, reverted: Every currency tallyho supports has two decimals. Plain amounts are mi…
- … and 7 more
