## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.
Workspace: /root/bench-work/ws/bfp9ba0-relay-order-ids-av-1/relay-order-ids
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
    pricing/ (5 files)
    report/ (3 files)
    repository.py
    store/ (5 files)
    text.py
    validate.py
    workflow.py
  tests/ (7 files)
    conftest.py
    test_models.py
    test_money.py
    test_pricing.py
    test_report.py
    test_store.py
    test_text.py

## Code
- sym:9225 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:9224 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:9228 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:9227 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:9217 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:9226 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:9200 apply_discounts (tallyho/pricing/discounts.py:46-67)
  def apply_discounts(subtotal: Money, discounts: list[Discount]) -> tuple[Money, list[tuple[str, Money]]]:
- sym:9162 Order (tallyho/models.py:64-109)
  class Order:
- sym:9168 Order.to_dict (tallyho/models.py:98-109)
  def to_dict(self) -> dict:
- sym:9191 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:9203 quote_order (tallyho/pricing/quote.py:30-53)
  def quote_order( order: Order, coupons: CouponBook | None = None, taxes: TaxTable | None = None, today: date | None = None, ) -> Quote:
- sym:9292 test_list_page_cursor (tests/test_store.py:20-25)
  def test_list_page_cursor(store):

---
tokens: 632/800
