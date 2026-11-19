## Task
# relay-coupon-days-left · relay-coupon-days-left · open
Goal: Customers ask how long a coupon stays valid. Add `Coupon.days_left(today)`: on how many days, counting from today, the coupon can still be used; 0 once it cannot be used any more, and None for a coupon that never expires.
Workspace: /m2b/r3/bench-work/ws/bfp43f-relay-coupon-days-left-av-1/relay-coupon-days-left
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:862 Order (tallyho/models.py:64-109)
  class Order:
- sym:887 Coupon.usable (tallyho/pricing/coupons.py:30-37)
  def usable(self, subtotal: Money, tier: str, today: date) -> bool:
- sym:851 Ledger.record_sale (tallyho/ledger.py:37-44)
  def record_sale(self, on: date, order_id: str, quote: Quote) -> None:
- sym:885 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:859 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:903 quote_order (tallyho/pricing/quote.py:30-53)
  def quote_order( order: Order, coupons: CouponBook | None = None, taxes: TaxTable | None = None, today: date | None = None, ) -> Quote:
- sym:855 Product (tallyho/models.py:23-35)
  class Product:
- sym:888 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:853 Ledger.trial_balance_ok (tallyho/ledger.py:52-53)
  def trial_balance_ok(self) -> bool:
- sym:856 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:966 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:849 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:852 Ledger.balances (tallyho/ledger.py:46-50)
- sym:892 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:861 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:863 Order.currency (tallyho/models.py:73-74)
- sym:866 Order.subtotal (tallyho/models.py:92-93)
- sym:871 Money (tallyho/money.py:22-83)
- sym:987 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
- sym:985 test_fixed_never_below_zero (tests/test_pricing.py:15-17)

---
tokens: 678/800
