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
- sym:862 Order (tallyho/models.py:64-109)
  class Order:
- sym:888 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:842 NotFound (tallyho/errors.py:24-25)
  class NotFound(TallyError):
- sym:890 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:856 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:891 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:851 Ledger.record_sale (tallyho/ledger.py:37-44)
  def record_sale(self, on: date, order_id: str, quote: Quote) -> None:
- sym:889 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
  def __init__(self, coupons: list[Coupon] | None = None) -> None:
- sym:855 Product (tallyho/models.py:23-35)
  class Product:
- sym:967 Workflow.__init__ (tallyho/workflow.py:24-27)
  def __init__(self, ledger: Ledger, bus: EventBus | None = None, coupons: CouponBook | None = None) -> None:
- sym:836 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:892 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
  def discounts_for(self, codes: list[str], subtotal: Money, tier: str, today: date) -> list[Discount]:
- sym:832 cmd_products (tallyho/cli.py:20-23)
- sym:903 quote_order (tallyho/pricing/quote.py:30-53)
- sym:852 Ledger.balances (tallyho/ledger.py:46-50)
- sym:847 EventBus.publish (tallyho/events.py:19-22)
- sym:885 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:905 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:914 product_from_record (tallyho/repository.py:19-21)
- sym:987 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 672/800
