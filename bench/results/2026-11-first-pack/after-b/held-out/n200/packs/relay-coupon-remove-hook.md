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
- rec:6dfadd74 attempt: CouponBook stores codes as code.upper() and get() looks them up with code.strip().upper(). Indexing _by_code with the code as typed misses ' save10 ' or 'save10'.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:7ff168a7 decision: `CouponBook` keeps its docstring as the one place its behaviour is des…
- rec:913e5799 attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:d5f9cff8 attempt: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:516a9601 attempt: Moving the imports inside the `coupons` module to speed up start-up sa…
- rec:6f166d03 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…

## Code
- sym:37036 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:37039 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:37038 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:37037 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:37040 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:37115 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:37033 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:37051 quote_order (tallyho/pricing/quote.py:30-53)
- sym:37035 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:37135 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 709/800
omitted 30 lower-ranked items (expand an id for detail):
- rec:b66c27fb attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:da4eb564 attempt: Wrapping `RecordStore.delete` in a bare try/except with a retry loop h…
- rec:ab69b828 attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:07a43de6 attempt: Caching what `Coupon` returns in a module-level dict broke tests that…
- rec:5f4e3002 attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:7d63f4e8 attempt: Caching what `Coupon.usable` returns in a module-level dict broke test…
- … and 24 more
