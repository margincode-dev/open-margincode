## Task
# relay-coupon-days-left · relay-coupon-days-left · open
Goal: Customers ask how long a coupon stays valid. Add `Coupon.days_left(today)`: on how many days, counting from today, the coupon can still be used; 0 once it cannot be used any more, and None for a coupon that never expires.
Workspace: /m2b/r3/bench-work/ws/bfpe76-relay-coupon-days-left-av-1/relay-coupon-days-left
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:3684 Order (tallyho/models.py:64-109)
  class Order:
- sym:3709 Coupon.usable (tallyho/pricing/coupons.py:30-37)
  def usable(self, subtotal: Money, tier: str, today: date) -> bool:
- sym:3673 Ledger.record_sale (tallyho/ledger.py:37-44)
  def record_sale(self, on: date, order_id: str, quote: Quote) -> None:
- sym:3707 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:3681 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:3725 quote_order (tallyho/pricing/quote.py:30-53)
  def quote_order( order: Order, coupons: CouponBook | None = None, taxes: TaxTable | None = None, today: date | None = None, ) -> Quote:
- sym:3677 Product (tallyho/models.py:23-35)
  class Product:
- sym:3710 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:3675 Ledger.trial_balance_ok (tallyho/ledger.py:52-53)
  def trial_balance_ok(self) -> bool:
- sym:3678 Product.__post_init__ (tallyho/models.py:30-35)
- sym:3788 Workflow (tallyho/workflow.py:23-50)
- sym:3671 Ledger (tallyho/ledger.py:24-53)
- sym:3674 Ledger.balances (tallyho/ledger.py:46-50)
- sym:3714 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:3683 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:3685 Order.currency (tallyho/models.py:73-74)
- sym:3688 Order.subtotal (tallyho/models.py:92-93)
- sym:3693 Money (tallyho/money.py:22-83)
- sym:3809 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:3807 test_fixed_never_below_zero (tests/test_pricing.py:15-17)

---
tokens: 679/800
