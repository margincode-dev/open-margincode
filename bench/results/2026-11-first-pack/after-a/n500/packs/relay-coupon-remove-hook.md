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
- rec:7ab8311c attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:9b525ba4 attempt: Replacing the loop in `CouponBook` with a comprehension made it shorte…
- rec:575cb11b attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:666c7b42 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:015179a2 decision: `CouponBook` keeps its docstring as the one place its behaviour is des…
- rec:103547e6 decision: New optional parameters of `CouponBook.add` are keyword-only (after *)…

## Code
- sym:20518 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:20521 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:20520 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:20519 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:20522 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:20597 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:20515 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:20533 quote_order (tallyho/pricing/quote.py:30-53)
- sym:20517 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:20617 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
  def test_coupon_book_skips_unknown(order):

---
tokens: 635/800
omitted 5 lower-ranked items (expand an id for detail):
- rec:6157adbe note: Remember `CouponBook.discounts_for` is not thread-safe; each worker ne…
- sym:20466 load_config (tallyho/config.py:23-47)
- sym:20477 EventBus.publish (tallyho/events.py:19-22)
- sym:20535 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:20544 product_from_record (tallyho/repository.py:19-21)
