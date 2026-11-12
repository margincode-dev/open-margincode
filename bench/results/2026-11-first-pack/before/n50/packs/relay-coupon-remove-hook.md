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
- rec:423a2771 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:71ac72fd attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/pricing/discounts.py:Discount
- rec:d90b867a attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:593774e1 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:1a7549c1 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:a2a0bb59 decision: New optional parameters of the `discounts` module are keyword-only (af…
- rec:d062977c note: `CouponBook.__init__` is on the hot path of the nightly batch job; pro…

## Code
- sym:6698 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:6701 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:6700 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:6699 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:6702 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:6777 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:6695 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:6713 quote_order (tallyho/pricing/quote.py:30-53)
- sym:6697 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:6797 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:6646 load_config (tallyho/config.py:23-47)
- sym:6657 EventBus.publish (tallyho/events.py:19-22)
- sym:6715 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:6724 product_from_record (tallyho/repository.py:19-21)

---
tokens: 683/800
