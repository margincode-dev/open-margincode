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
- rec:f1121658 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:1626399c decision, open: `CouponBook.add` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/pricing/coupons.py:CouponBook.add
- rec:b3946c60 attempt, open: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:8f1b8a0c attempt, open: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:c7c40511 attempt, open: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…

## Code
- sym:3378 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:3381 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:3380 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:3379 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:3382 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:3457 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:3375 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:3393 quote_order (tallyho/pricing/quote.py:30-53)
- sym:3377 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:3477 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 703/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:ef4aa430 attempt, open: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:c60c462d attempt, open: Wrapping `cmd_products` in a bare try/except with a retry loop hid the…
- rec:f67f66ad attempt, open: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:0100bed8 attempt, open: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:8d92d2e8 attempt, open: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- … and 26 more
