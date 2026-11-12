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
- rec:0b57f399 attempt: CouponBook stores codes as code.upper() and get() looks them up with code.strip().upper(). Indexing _by_code with the code as typed misses ' save10 ' or 'save10'.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:71c7e111 attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:d8f1162d attempt: Replacing the loop in `CouponBook` with a comprehension made it shorte…
- rec:39ca772b attempt: Tried calling `CouponBook.get` twice to force a refresh; the second ca…
- rec:98cc576f attempt: Tried calling `Coupon.discount` twice to force a refresh; the second c…
- rec:12efdc82 attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…

## Code
- sym:35376 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:35379 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:35378 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:35377 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:35380 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:35455 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:35373 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:35391 quote_order (tallyho/pricing/quote.py:30-53)
- sym:35375 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:35475 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 698/800
omitted 30 lower-ranked items (expand an id for detail):
- rec:bb23a11b attempt: Moving the imports inside `Coupon.discount` to speed up start-up saved…
- rec:f0e2bf6b attempt: Moving the imports inside the `coupons` module to speed up start-up sa…
- rec:483a3507 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:c38bd17f attempt: Replacing the loop in `Order.currency` with a comprehension made it sh…
- rec:4dbe7013 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:fdae67fd attempt: Wrapping `RecordStore.delete` in a bare try/except with a retry loop h…
- … and 24 more
