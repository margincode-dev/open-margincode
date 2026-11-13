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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:118adb62 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with code.strip().upper(). Indexing _by_code with the code as typed misses ' save10 ' or 'save10'.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:1a811b38 attempt, open: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:6ac15929 attempt, open: Replacing the loop in `CouponBook` with a comprehension made it shorte…
- rec:2ebde28a attempt, open: Tried calling `CouponBook.get` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/pricing/coupons.py:CouponBook.get
- rec:37c8ce63 attempt, open: Tried calling `Coupon.discount` twice to force a refresh; the second c…

## Code
- sym:5038 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:5041 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:5040 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:5039 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:5042 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:5117 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:5035 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:5053 quote_order (tallyho/pricing/quote.py:30-53)
- sym:5037 Coupon.usable (tallyho/pricing/coupons.py:30-37)

---
tokens: 694/800
omitted 32 lower-ranked items (expand an id for detail):
- rec:645c54fd attempt, open: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:0737527f attempt, open: Moving the imports inside `Coupon.discount` to speed up start-up saved…
- rec:862c7930 attempt, open: Moving the imports inside the `coupons` module to speed up start-up sa…
- rec:e21f0f5f attempt, open: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:f58e8619 attempt, open: Replacing the loop in `Order.currency` with a comprehension made it sh…
- … and 27 more
