## Task
# relay-order-can-cancel · relay-order-can-cancel · open
Goal: The order page hides its Cancel button once an order can no longer be cancelled. Add `Order.can_cancel()`: True while the order can still be cancelled, False once it cannot.
Workspace: /m2b/r3/bench-work/ws/bfp43f-relay-order-can-cancel-av-1/relay-order-can-cancel
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:2050b25b decision: Workflow.cancel takes only draft orders, the same rule Order.add and Order.remove follow. Write Order.can_cancel as self.status is OrderStatus.DRAFT, beside them.
  anchors: tallyho/models.py:Order
- rec:6ff77b5b decision: Collections that hang off an order are named "order-<what>": comments…
- rec:4c3f02b3 decision: RecordStore.delete raises NotFound for an id that is not there, as get…
- rec:e51fa102 decision: apply_discounts adds the percentages up and takes them off the subtota…
- rec:e0ee399f intent: Earlier work on this code

## Code
- sym:1470 Workflow.cancel (tallyho/workflow.py:49-50)
  def cancel(self, order: Order) -> None:
- sym:1463 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:1466 Workflow._move (tallyho/workflow.py:29-33)
- sym:1464 Workflow (tallyho/workflow.py:23-50)
- sym:1363 Order.remove (tallyho/models.py:86-89)
- sym:1352 OrderStatus (tallyho/models.py:14-19)
- sym:1334 load_config (tallyho/config.py:23-47)
- sym:1360 Order (tallyho/models.py:64-109)
- sym:1431 RecordStore.list_page (tallyho/store/base.py:36-37)
- sym:1424 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:1336 ValidationError (tallyho/errors.py:8-17)

---
tokens: 698/800
omitted 12 lower-ranked items (expand an id for detail):
- rec:955ec91a decision: Coupon.expires is the first day a coupon no longer works: one that exp…
- rec:5268918e decision: Raise ValidationError with field= naming the input that is wrong: str(…
- rec:eea03e28 decision: Tax regions read from [tax] are upper-cased: configparser lower-cases…
- rec:ef20c89c decision: Ledger.balances() gives each account as the shop reports it: cash, rev…
- rec:692bd4da decision: text.pad returns exactly width characters: it pads short text and cuts…
- rec:d6edfc1a decision: CSV is written by hand, not with the csv module, so the bytes stay the…
- … and 6 more
