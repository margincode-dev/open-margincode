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
- rec:d4e1ad7e attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:48cad755 attempt: Replacing the loop in `CouponBook` with a comprehension made it shorte…
- rec:95061bd4 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:4c6b8d68 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/pricing/coupons.py:CouponBook.add
- rec:dc23fcdb decision: `CouponBook` keeps its docstring as the one place its behaviour is des…
- rec:4f0a3095 decision: New optional parameters of `CouponBook.add` are keyword-only (after *)…
- rec:d1b20ec3 note: Remember `CouponBook.discounts_for` is not thread-safe; each worker ne…

## Code
- sym:10018 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:10021 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:10020 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:10019 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:10022 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:10097 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:10015 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:10033 quote_order (tallyho/pricing/quote.py:30-53)
- sym:10017 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:10117 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:9966 load_config (tallyho/config.py:23-47)
- sym:9977 EventBus.publish (tallyho/events.py:19-22)
- sym:10035 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:10044 product_from_record (tallyho/repository.py:19-21)

---
tokens: 686/800
