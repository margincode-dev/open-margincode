## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.
Workspace: /root/bench-work/ws/bfpa928-relay-order-ids-av-1/relay-order-ids
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:a91886c0 attempt: Tried calling `Repository.save_order` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.save_order
- rec:7e761b26 attempt: Tried calling `quote_order` twice to force a refresh; the second call…
- rec:7cfcc8cc decision: New optional parameters of `Repository.orders` are keyword-only (after…
- rec:223ba532 decision: `Order.add` keeps its docstring as the one place its behaviour is desc…
- rec:e9d4ff25 decision: Constants that `Repository._order_from_record` depends on live at the…
- rec:f249dbfa decision: Constants that `Repository.orders` depends on live at the top of its m…
- rec:89f3ae56 note: Remember `Repository.order` is not thread-safe; each worker needs its…

## Code
- sym:14205 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:14204 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:14207 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:14206 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:14197 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:14208 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:14144 Order.add (tallyho/models.py:76-84)
- sym:14180 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:14142 Order (tallyho/models.py:64-109)
  class Order:
- sym:14148 Order.to_dict (tallyho/models.py:98-109)
- sym:14171 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:14183 quote_order (tallyho/pricing/quote.py:30-53)
- sym:14210 RecordStore.put (tallyho/store/base.py:24-25)
- sym:14256 order (tests/conftest.py:22-26)
- sym:14272 test_list_page_cursor (tests/test_store.py:20-25)
  def test_list_page_cursor(store):

---
tokens: 689/800
