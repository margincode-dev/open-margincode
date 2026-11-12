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
- rec:d8d9cf70 attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:38ef8576 attempt: Replacing the loop in `CouponBook` with a comprehension made it shorte…
- rec:dacb52d7 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:e15d7c0a decision: `CouponBook.add` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/pricing/coupons.py:CouponBook.add
- rec:b315805a decision: `CouponBook` keeps its docstring as the one place its behaviour is des…
- rec:1e07e4f8 decision: New optional parameters of `CouponBook.add` are keyword-only (after *)…
- rec:f9b3cf6a note: Remember `CouponBook.discounts_for` is not thread-safe; each worker ne…

## Code
- sym:8358 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:8361 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:8360 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:8359 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:8362 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:8437 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:8355 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:8373 quote_order (tallyho/pricing/quote.py:30-53)
- sym:8357 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:8457 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:8306 load_config (tallyho/config.py:23-47)
- sym:8317 EventBus.publish (tallyho/events.py:19-22)
- sym:8375 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:8384 product_from_record (tallyho/repository.py:19-21)

---
tokens: 686/800
