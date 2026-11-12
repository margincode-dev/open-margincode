## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:12484 Order.add (tallyho/models.py:76-84)
  def add(self, product: Product, quantity: int = 1) -> None:
- sym:12545 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:12482 Order (tallyho/models.py:64-109)
  class Order:
- sym:12544 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:12480 OrderLine.__post_init__ (tallyho/models.py:55-56)
  def __post_init__(self) -> None:
- sym:12548 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:12474 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:12547 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:12488 Order.to_dict (tallyho/models.py:98-109)
  def to_dict(self) -> dict:
- sym:12537 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:12479 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:12546 Repository._order_from_record (tallyho/repository.py:61-70)
  def _order_from_record(self, r: dict) -> Order:
- sym:12453 cmd_export (tallyho/cli.py:26-28)
  def cmd_export(repo: Repository, args: argparse.Namespace) -> int:
- sym:12520 apply_discounts (tallyho/pricing/discounts.py:46-67)
  def apply_discounts(subtotal: Money, discounts: list[Discount]) -> tuple[Money, list[tuple[str, Money]]]:
- sym:12510 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:12511 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:12523 quote_order (tallyho/pricing/quote.py:30-53)
- sym:12550 RecordStore.put (tallyho/store/base.py:24-25)
- sym:12551 RecordStore.get (tallyho/store/base.py:28-29)
- sym:12612 test_list_page_cursor (tests/test_store.py:20-25)
  def test_list_page_cursor(store):

---
tokens: 678/800
