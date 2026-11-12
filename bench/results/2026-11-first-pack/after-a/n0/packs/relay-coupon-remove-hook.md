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
- rec:db070b81 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:b731a2f4 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:cdbf5262 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:9ff6b68c decision: Event payloads carry money as integer minor units plus the currency co…
- rec:b86629e3 decision: Errors about customer data must not echo the rejected value: they are…
- rec:ab531093 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:157e36aa decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…

## Code
- sym:17198 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:17201 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:17200 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:17199 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:17202 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
  def discounts_for(self, codes: list[str], subtotal: Money, tier: str, today: date) -> list[Discount]:
- sym:17277 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:17195 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:17213 quote_order (tallyho/pricing/quote.py:30-53)
- sym:17197 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:17297 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:17146 load_config (tallyho/config.py:23-47)
- sym:17157 EventBus.publish (tallyho/events.py:19-22)
- sym:17215 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:17224 product_from_record (tallyho/repository.py:19-21)

---
tokens: 684/800
