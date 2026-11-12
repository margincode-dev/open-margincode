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
- rec:c9033d77 attempt: Wrapping `CouponBook` in a bare try/except with a retry loop hid the r…
- rec:967e4e87 attempt: Replacing the loop in `Coupon.usable` with a comprehension made it sho…
- rec:9341ab7a attempt: Replacing the loop in `Coupon` with a comprehension made it shorter bu…
- rec:2572023c attempt: Wrapping `CouponBook.__init__` in a bare try/except with a retry loop…
- rec:b3653ac1 decision: New optional parameters of `CouponBook.get` are keyword-only (after *)…
- rec:638d24ec decision: New optional parameters of `CouponBook` are keyword-only (after *), so…
- rec:085ddcc5 decision: Constants that `CouponBook.get` depends on live at the top of its modu…

## Code
- sym:14998 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:15001 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:15000 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:14999 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:15002 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
  def discounts_for(self, codes: list[str], subtotal: Money, tier: str, today: date) -> list[Discount]:
- sym:15077 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:14995 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:15013 quote_order (tallyho/pricing/quote.py:30-53)
- sym:14997 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:15097 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:14946 load_config (tallyho/config.py:23-47)
- sym:14957 EventBus.publish (tallyho/events.py:19-22)
- sym:15015 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:15024 product_from_record (tallyho/repository.py:19-21)

---
tokens: 684/800
