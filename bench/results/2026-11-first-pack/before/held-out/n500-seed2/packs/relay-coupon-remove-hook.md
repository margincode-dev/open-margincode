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
- rec:b0ca7de6 attempt: Wrapping `CouponBook.get` in a bare try/except with a retry loop hid t…
- rec:2f4ee69a attempt: Replacing the loop in `Coupon` with a comprehension made it shorter bu…
- rec:f038b604 attempt: Wrapping `CouponBook.__init__` in a bare try/except with a retry loop…
- rec:e6db3ffa attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:e2f2d94a decision: New optional parameters of `CouponBook.get` are keyword-only (after *)…
- rec:10c1dcb5 decision: `CouponBook` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:fb22ec62 note: Remember `CouponBook.add` is not thread-safe; each worker needs its ow…

## Code
- sym:13338 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:13341 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:13340 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:13339 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:13342 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:13417 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:13335 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:13353 quote_order (tallyho/pricing/quote.py:30-53)
- sym:13337 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:13437 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:13286 load_config (tallyho/config.py:23-47)
- sym:13297 EventBus.publish (tallyho/events.py:19-22)
- sym:13355 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:13364 product_from_record (tallyho/repository.py:19-21)

---
tokens: 685/800
