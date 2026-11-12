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
- rec:17481f90 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:f8b2faec attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:032d53fb decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:1b4babd8 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:112a3744 decision: Errors about customer data must not echo the rejected value: they are…
- rec:66a6b465 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:1c2341b9 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…

## Code
- sym:5038 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:5041 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:5040 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:5039 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:5042 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:5117 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:5035 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:5053 quote_order (tallyho/pricing/quote.py:30-53)
- sym:5037 Coupon.usable (tallyho/pricing/coupons.py:30-37)
  def usable(self, subtotal: Money, tier: str, today: date) -> bool:
- sym:5137 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:4986 load_config (tallyho/config.py:23-47)
- sym:4997 EventBus.publish (tallyho/events.py:19-22)
- sym:5055 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:5064 product_from_record (tallyho/repository.py:19-21)

---
tokens: 684/800
