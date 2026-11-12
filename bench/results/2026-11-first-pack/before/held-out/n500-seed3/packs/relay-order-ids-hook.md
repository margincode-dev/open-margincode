## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.
Workspace: /root/bench-work/ws/bfpadaa-relay-order-ids-av-1/relay-order-ids
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:64afee39 attempt: Tried calling `Repository.orders` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/repository.py:Repository.orders
- rec:5e60cb8b decision: New optional parameters of `Repository._order_from_record` are keyword…
- rec:856746a3 decision: New optional parameters of `Repository.order` are keyword-only (after…
- rec:3b1213ec decision: `Repository.save_order` keeps its docstring as the one place its behav…
- rec:11a8fcc1 note: Remember `Repository._order_from_record` is not thread-safe; each work…
- rec:ea4557ea note: Remember `Order.add` is not thread-safe; each worker needs its own ins…
- rec:b5333adc note: The repr of objects around `Repository._order_from_record` shows up in…

## Code
- sym:15865 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:15864 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:15867 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:15866 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:15857 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:15868 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:15804 Order.add (tallyho/models.py:76-84)
- sym:15840 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:15802 Order (tallyho/models.py:64-109)
  class Order:
- sym:15808 Order.to_dict (tallyho/models.py:98-109)
- sym:15831 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:15843 quote_order (tallyho/pricing/quote.py:30-53)
- sym:15870 RecordStore.put (tallyho/store/base.py:24-25)
- sym:15916 order (tests/conftest.py:22-26)
- sym:15932 test_list_page_cursor (tests/test_store.py:20-25)
  def test_list_page_cursor(store):

---
tokens: 689/800
