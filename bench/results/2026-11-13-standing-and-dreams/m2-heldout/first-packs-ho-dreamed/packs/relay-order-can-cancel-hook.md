## Task
# relay-order-can-cancel · relay-order-can-cancel · open
Goal: The order page hides its Cancel button once an order can no longer be cancelled. Add `Order.can_cancel()`: True while the order can still be cancelled, False once it cannot.
Workspace: /m2b/r3/bench-work/ws/bfpe76-relay-order-can-cancel-av-1/relay-order-can-cancel
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:bb10ccab decision: Workflow.cancel takes only draft orders, the same rule Order.add and O… (disputed)
- rec:e00aa72b decision: Collections that hang off an order are named "order-<what>": comments… (disputed)
- rec:76353acd decision: apply_discounts adds the percentages up and takes them off the subtota…
- rec:3a07f951 decision: Raise ValidationError with field= naming the input that is wrong: str(…
- rec:8c159795 intent: Earlier work on this code

## Code
- sym:4292 Workflow.cancel (tallyho/workflow.py:49-50)
  def cancel(self, order: Order) -> None:
- sym:4285 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:4288 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:4286 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:4185 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:4174 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:4156 load_config (tallyho/config.py:23-47)
- sym:4182 Order (tallyho/models.py:64-109)
- sym:4253 RecordStore.list_page (tallyho/store/base.py:36-37)
- sym:4246 Repository._order_from_record (tallyho/repository.py:61-70)

---
tokens: 685/800
omitted 13 lower-ranked items (expand an id for detail):
- rec:11d5c7a9 decision: Tax regions read from [tax] are upper-cased: configparser lower-cases…
- rec:71c2a7e1 decision: text.pad returns exactly width characters: it pads short text and cuts…
- rec:163b9d6b decision: CSV is written by hand, not with the csv module, so the bytes stay the…
- rec:4ed8be67 decision: RecordStore.delete raises NotFound for an id that is not there, as get… (disputed)
- rec:fbb86079 decision: Coupon.expires is the first day a coupon no longer works: one that exp… (disputed)
- … and 8 more
