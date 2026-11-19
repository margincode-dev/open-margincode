## Task
# relay-delete-order · relay-delete-order · open
Goal: Add `Repository.delete_order(oid)`: it removes the stored order with that id and returns True, or returns False when there is no such order.
Workspace: /m2b/r3/bench-work/ws/bfp43f-relay-delete-order-av-1/relay-delete-order
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:9173c493 decision: RecordStore.delete raises NotFound for an id that is not there, as get does. Write delete_order as try: store.delete(...) except NotFound: return False, then return True; no get() first.
  anchors: tallyho/repository.py:Repository
- rec:414573aa decision: Workflow.cancel takes only draft orders, the same rule Order.add and O…
- rec:bd73fdb1 decision: Collections that hang off an order are named "order-<what>": comments…
- rec:a7b7c231 decision: Tax regions read from [tax] are upper-cased: configparser lower-cases…
- rec:ee03d866 decision: Raise ValidationError with field= naming the input that is wrong: str(…

## Code
- sym:1091 Repository.order (tallyho/repository.py:57-59)
- sym:1090 Repository.save_order (tallyho/repository.py:54-55)
- sym:1083 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:1093 Repository.orders (tallyho/repository.py:72-74)
- sym:1092 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:1098 RecordStore.delete (tallyho/store/base.py:32-33)
- sym:1095 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:1031 Order.remove (tallyho/models.py:86-89)
- sym:1115 MemoryStore.delete (tallyho/store/memory.py:34-37)
- sym:1028 Order (tallyho/models.py:64-109)
  class Order:
- sym:1034 Order.to_dict (tallyho/models.py:98-109)

---
tokens: 709/800
omitted 9 lower-ranked items (expand an id for detail):
- rec:7741e020 decision: Coupon.expires is the first day a coupon no longer works: one that exp…
- rec:b333054a decision: apply_discounts adds the percentages up and takes them off the subtota…
- rec:c7af6c9a decision: Ledger.balances() gives each account as the shop reports it: cash, rev…
- rec:3ff643fd decision: text.pad returns exactly width characters: it pads short text and cuts…
- rec:c0e801d2 decision: CSV is written by hand, not with the csv module, so the bytes stay the…
- rec:2324abc0 intent: Earlier work on this code
- sym:1057 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- … and 2 more
