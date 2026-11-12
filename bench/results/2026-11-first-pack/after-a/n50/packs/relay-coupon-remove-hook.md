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
- rec:d63374e0 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:09d16954 attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/pricing/discounts.py:Discount
- rec:4e89f12b attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:74c2080d attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:00bfddc3 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is…
- rec:b614abae decision: New optional parameters of the `discounts` module are keyword-only (af…
- rec:ea8e0818 note: `CouponBook.__init__` is on the hot path of the nightly batch job; pro…

## Code
- sym:18858 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:18861 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:18860 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:18859 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:18862 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:18937 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:18855 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:18873 quote_order (tallyho/pricing/quote.py:30-53)
- sym:18857 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:18957 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:18806 load_config (tallyho/config.py:23-47)
- sym:18817 EventBus.publish (tallyho/events.py:19-22)
- sym:18875 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:18884 product_from_record (tallyho/repository.py:19-21)

---
tokens: 685/800
