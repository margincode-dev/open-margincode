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
- rec:894a771f attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:f11e7fbe attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:4fd34602 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/pricing/coupons.py:CouponBook.add
- rec:b9bf1a33 decision: `CouponBook` keeps its docstring as the one place its behaviour is des…
- rec:7ce6ba75 decision: `Coupon.discount` keeps its docstring as the one place its behaviour i…
- rec:00b90c87 decision: Constants that `Coupon.usable` depends on live at the top of its modul…
- rec:61229f52 note: `CouponBook.__init__` is on the hot path of the nightly batch job; pro…

## Code
- sym:11678 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:11681 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:11680 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:11679 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:11682 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:11757 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:11675 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:11693 quote_order (tallyho/pricing/quote.py:30-53)
- sym:11677 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:11777 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:11626 load_config (tallyho/config.py:23-47)
- sym:11637 EventBus.publish (tallyho/events.py:19-22)
- sym:11695 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:11704 product_from_record (tallyho/repository.py:19-21)

---
tokens: 686/800
