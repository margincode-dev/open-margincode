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
  def add(self, product: Product, quantity: int = 1) -> None:
- sym:14142 Order (tallyho/models.py:64-109)
  class Order:
- sym:14180 apply_discounts (tallyho/pricing/discounts.py:46-67)
  def apply_discounts(subtotal: Money, discounts: list[Discount]) -> tuple[Money, list[tuple[str, Money]]]:
- sym:14140 OrderLine.__post_init__ (tallyho/models.py:55-56)
  def __post_init__(self) -> None:
- sym:14148 Order.to_dict (tallyho/models.py:98-109)
  def to_dict(self) -> dict:
- sym:14171 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:14183 quote_order (tallyho/pricing/quote.py:30-53)
  def quote_order( order: Order, coupons: CouponBook | None = None, taxes: TaxTable | None = None, today: date | None = None, ) -> Quote:
- sym:14210 RecordStore.put (tallyho/store/base.py:24-25)
  def put(self, collection: str, record: dict) -> None:
- sym:14272 test_list_page_cursor (tests/test_store.py:20-25)
  def test_list_page_cursor(store):

---
tokens: 629/800
