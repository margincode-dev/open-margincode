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
- rec:c700ad3b attempt: CouponBook stores codes as code.upper() and get() looks them up with code.strip().upper(). Indexing _by_code with the code as typed misses ' save10 ' or 'save10'.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:d5368751 note: Remember `CouponBook.add` is not thread-safe; each worker needs its ow…
- rec:e9b9dd28 attempt: Wrapping `CouponBook.get` in a bare try/except with a retry loop hid t…
- rec:4ccba29b attempt: Replacing the loop in `Coupon` with a comprehension made it shorter bu…
- rec:38ee12ad attempt: Wrapping `CouponBook.__init__` in a bare try/except with a retry loop…
- rec:b2abf6f3 attempt: Replacing the loop in the `discounts` module with a comprehension made…

## Code
- sym:38696 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:38699 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:38698 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:38697 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:38700 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:38775 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:38693 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:38711 quote_order (tallyho/pricing/quote.py:30-53)
- sym:38695 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:38795 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 696/800
omitted 30 lower-ranked items (expand an id for detail):
- rec:a87a0b33 attempt: Tried calling `Coupon` twice to force a refresh; the second call retur…
- rec:b0a64661 attempt: Moving the imports inside `CouponBook.discounts_for` to speed up start…
- rec:67fe6091 attempt: Adding type: ignore comments around the `coupons` module to quiet the…
- rec:a75d437c attempt: Wrapping the `discounts` module in a bare try/except with a retry loop…
- rec:de349b45 attempt: Caching what `CouponBook.discounts_for` returns in a module-level dict…
- rec:005cc0cd attempt: Replacing the loop in `Customer` with a comprehension made it shorter…
- … and 24 more
