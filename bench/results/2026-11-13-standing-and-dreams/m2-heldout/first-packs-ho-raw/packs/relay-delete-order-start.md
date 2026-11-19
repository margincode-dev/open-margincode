## Task
# relay-delete-order · relay-delete-order · open
Goal: Add `Repository.delete_order(oid)`: it removes the stored order with that id and returns True, or returns False when there is no such order.
Workspace: /m2b/r3/bench-work/ws/bfp43f-relay-delete-order-av-1/relay-delete-order
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
    __init__.py
    cli.py
    config.py
    errors.py
    events.py
    ledger.py
    models.py
    money.py
    … +7 more
  tests/ (7 files)
    conftest.py
    test_models.py
    test_money.py
    test_pricing.py
    test_report.py
    test_store.py
    test_text.py

## Code
- sym:1091 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:1083 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:1095 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:1090 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:1093 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:1092 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:1098 RecordStore.delete (tallyho/store/base.py:32-33)
  def delete(self, collection: str, record_id: str) -> None:
- sym:1028 Order (tallyho/models.py:64-109)
  class Order:
- sym:1034 Order.to_dict (tallyho/models.py:98-109)
  def to_dict(self) -> dict:
- sym:1057 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:1069 quote_order (tallyho/pricing/quote.py:30-53)
  def quote_order( order: Order, coupons: CouponBook | None = None, taxes: TaxTable | None = None, today: date | None = None, ) -> Quote:
- sym:1157 test_put_get_delete (tests/test_store.py:12-17)
  def test_put_get_delete(store):

---
tokens: 603/800
