## Task
# relay-delete-order · relay-delete-order · open
Goal: Add `Repository.delete_order(oid)`: it removes the stored order with that id and returns True, or returns False when there is no such order.
Workspace: /m2b/r3/bench-work/ws/bfpe76-relay-delete-order-av-1/relay-delete-order
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
- sym:3850 Order (tallyho/models.py:64-109)
  class Order:
- sym:3913 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:3842 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:3905 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:3848 OrderLine.__post_init__ (tallyho/models.py:55-56)
  def __post_init__(self) -> None:
- sym:3917 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:3830 NotFound (tallyho/errors.py:24-25)
  class NotFound(TallyError):
- sym:3912 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:3847 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:3915 Repository.orders (tallyho/repository.py:72-74)
- sym:3821 cmd_export (tallyho/cli.py:26-28)
  def cmd_export(repo: Repository, args: argparse.Namespace) -> int:
- sym:3914 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:3839 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:3846 Customer.__post_init__ (tallyho/models.py:46-47)
- sym:3920 RecordStore.delete (tallyho/store/base.py:32-33)
- sym:3856 Order.to_dict (tallyho/models.py:98-109)
- sym:3879 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:3891 quote_order (tallyho/pricing/quote.py:30-53)
- sym:3918 RecordStore.put (tallyho/store/base.py:24-25)
- sym:3979 test_put_get_delete (tests/test_store.py:12-17)

---
tokens: 683/800
