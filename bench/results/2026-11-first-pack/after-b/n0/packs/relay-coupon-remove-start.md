## Task
# relay-coupon-remove · relay-coupon-remove · open
Goal: Shops need to withdraw coupons. Add `CouponBook.remove(code)`: afterwards `get` no longer finds that coupon, and removing a code the book does not know does nothing.
Workspace: /root/bench-work/ws/bfp956f-relay-coupon-remove-av-1/relay-coupon-remove
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:32056 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:32058 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:32059 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:32057 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:32135 Workflow.__init__ (tallyho/workflow.py:24-27)
  def __init__(self, ledger: Ledger, bus: EventBus | None = None, coupons: CouponBook | None = None) -> None:
- sym:32060 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
  def discounts_for(self, codes: list[str], subtotal: Money, tier: str, today: date) -> list[Discount]:
- sym:32071 quote_order (tallyho/pricing/quote.py:30-53)
  def quote_order( order: Order, coupons: CouponBook | None = None, taxes: TaxTable | None = None, today: date | None = None, ) -> Quote:
- sym:32004 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:32015 EventBus.publish (tallyho/events.py:19-22)
  def publish(self, event: str, payload: dict) -> None:
- sym:32053 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:32073 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:
- sym:32155 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
  def test_coupon_book_skips_unknown(order):

---
tokens: 605/800
