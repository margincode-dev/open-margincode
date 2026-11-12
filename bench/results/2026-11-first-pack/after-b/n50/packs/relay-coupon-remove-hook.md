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
- rec:26e55dd1 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:263fdfa6 decision: `CouponBook.add` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/pricing/coupons.py:CouponBook.add
- rec:b83a4d8d attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:1961f71c attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:278cace3 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:e2f4833f attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…

## Code
- sym:33716 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:33719 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:33718 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:33717 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:33720 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:33795 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:33713 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:33731 quote_order (tallyho/pricing/quote.py:30-53)
- sym:33715 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:33815 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 699/800
omitted 30 lower-ranked items (expand an id for detail):
- rec:bccb7d5a attempt: Wrapping `cmd_products` in a bare try/except with a retry loop hid the…
- rec:f766ad23 attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:e426a437 attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:94f1fb3d attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:f52e6b25 attempt: Caching what `Discount.amount_off` returns in a module-level dict brok…
- rec:9de73e9d attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- … and 24 more
