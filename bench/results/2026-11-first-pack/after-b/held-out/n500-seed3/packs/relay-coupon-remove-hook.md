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
- rec:514aae92 attempt: CouponBook stores codes as code.upper() and get() looks them up with code.strip().upper(). Indexing _by_code with the code as typed misses ' save10 ' or 'save10'.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:e99fd842 decision: New optional parameters of `CouponBook.get` are keyword-only (after *)…
- rec:e1e622f4 attempt: Wrapping `CouponBook` in a bare try/except with a retry loop hid the r…
- rec:ceff3389 attempt: Replacing the loop in `Coupon.usable` with a comprehension made it sho…
- rec:1de5695f attempt: Replacing the loop in `Coupon` with a comprehension made it shorter bu…
- rec:1873dc7a attempt: Wrapping `CouponBook.__init__` in a bare try/except with a retry loop…

## Code
- sym:40356 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:40359 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:40358 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:40357 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:40360 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:40435 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:40353 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:40371 quote_order (tallyho/pricing/quote.py:30-53)
- sym:40355 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:40455 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 701/800
omitted 30 lower-ranked items (expand an id for detail):
- rec:6f124e66 attempt: Replacing the loop in `CouponBook.__init__` with a comprehension made…
- rec:e1411129 attempt: Wrapping `Coupon.discount` in a bare try/except with a retry loop hid…
- rec:e642329d attempt: Wrapping the `coupons` module in a bare try/except with a retry loop h…
- rec:dc0b5e67 attempt: Tried calling `CouponBook.add` twice to force a refresh; the second ca…
- rec:e3ba0507 attempt: Replacing the loop in `Discount` with a comprehension made it shorter…
- rec:ef6fd49c attempt: Wrapping `Discount.amount_off` in a bare try/except with a retry loop…
- … and 24 more
