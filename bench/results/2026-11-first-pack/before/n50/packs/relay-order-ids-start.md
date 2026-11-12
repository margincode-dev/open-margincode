## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.
Workspace: /root/bench-work/ws/bfp971f-relay-order-ids-av-1/relay-order-ids
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:7502 Order (tallyho/models.py:64-109)
  class Order:
- sym:7565 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:7500 OrderLine.__post_init__ (tallyho/models.py:55-56)
  def __post_init__(self) -> None:
- sym:7564 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:7494 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:7568 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:7499 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:7567 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:7473 cmd_export (tallyho/cli.py:26-28)
  def cmd_export(repo: Repository, args: argparse.Namespace) -> int:
- sym:7557 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:7487 EventBus.publish (tallyho/events.py:19-22)
  def publish(self, event: str, payload: dict) -> None:
- sym:7566 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:7498 Customer.__post_init__ (tallyho/models.py:46-47)
  def __post_init__(self) -> None:
- sym:7540 apply_discounts (tallyho/pricing/discounts.py:46-67)
  def apply_discounts(subtotal: Money, discounts: list[Discount]) -> tuple[Money, list[tuple[str, Money]]]:
- sym:7485 EventBus.__init__ (tallyho/events.py:12-14)
- sym:7508 Order.to_dict (tallyho/models.py:98-109)
- sym:7531 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:7543 quote_order (tallyho/pricing/quote.py:30-53)
- sym:7570 RecordStore.put (tallyho/store/base.py:24-25)
- sym:7632 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 675/800
