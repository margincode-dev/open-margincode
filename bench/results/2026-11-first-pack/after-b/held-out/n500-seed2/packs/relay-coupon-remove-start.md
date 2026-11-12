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

## Code
- sym:38696 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:38699 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:38698 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:38697 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:38700 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
  def discounts_for(self, codes: list[str], subtotal: Money, tier: str, today: date) -> list[Discount]:
- sym:38775 Workflow.__init__ (tallyho/workflow.py:24-27)
  def __init__(self, ledger: Ledger, bus: EventBus | None = None, coupons: CouponBook | None = None) -> None:
- sym:38693 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:38695 Coupon.usable (tallyho/pricing/coupons.py:30-37)
  def usable(self, subtotal: Money, tier: str, today: date) -> bool:
- sym:38711 quote_order (tallyho/pricing/quote.py:30-53)
  def quote_order( order: Order, coupons: CouponBook | None = None, taxes: TaxTable | None = None, today: date | None = None, ) -> Quote:
- sym:38670 Order (tallyho/models.py:64-109)
  class Order:
- sym:38644 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:38655 EventBus.publish (tallyho/events.py:19-22)
  def publish(self, event: str, payload: dict) -> None:
- sym:38713 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:
- sym:38722 product_from_record (tallyho/repository.py:19-21)
  def product_from_record(r: dict) -> Product:
- sym:38795 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)
  def test_coupon_book_skips_unknown(order):

---
tokens: 666/800
