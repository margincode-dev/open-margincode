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
  tests/ (7 files)

## Decisions & dead ends
- rec:142091bb decision: RecordStore.delete raises NotFound for an id that is not there, as get… (disputed)
- rec:9a9059b4 decision: Workflow.cancel takes only draft orders, the same rule Order.add and O… (disputed)
- rec:f381ba1e decision: Tax regions read from [tax] are upper-cased: configparser lower-cases…
- rec:a8ca45e8 decision: Raise ValidationError with field= naming the input that is wrong: str(…
- rec:693f4ee4 intent: Earlier work on this code

## Code
- sym:3913 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:3912 Repository.save_order (tallyho/repository.py:54-55)
  def save_order(self, o: Order) -> None:
- sym:3905 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:3915 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:3914 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:3920 RecordStore.delete (tallyho/store/base.py:32-33)
- sym:3917 RecordStore (tallyho/store/base.py:17-41)
- sym:3853 Order.remove (tallyho/models.py:86-89)
- sym:3937 MemoryStore.delete (tallyho/store/memory.py:34-37)
- sym:3850 Order (tallyho/models.py:64-109)
- sym:3856 Order.to_dict (tallyho/models.py:98-109)

---
tokens: 709/800
omitted 9 lower-ranked items (expand an id for detail):
- rec:8452422a decision: apply_discounts adds the percentages up and takes them off the subtota…
- rec:91ae05e3 decision: text.pad returns exactly width characters: it pads short text and cuts…
- rec:4f15ecc0 decision: CSV is written by hand, not with the csv module, so the bytes stay the…
- rec:c762d9fc decision: Collections that hang off an order are named "order-<what>": comments… (disputed)
- rec:04355b6e decision: Coupon.expires is the first day a coupon no longer works: one that exp… (disputed)
- rec:68e7e347 decision: Ledger.balances() gives each account as the shop reports it: cash, rev… (disputed)
- … and 3 more
