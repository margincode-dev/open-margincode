## Task
# relay-order-comments · relay-order-comments · open
Goal: Staff want to keep short comments on an order. Add `Repository.add_order_comment(oid, text)` and `Repository.order_comments(oid)`, which returns that order's comments as a list of strings, oldest first, and an empty list for an order without any. Keep them in the shop's store, so they survive a restart.
Workspace: /m2b/r3/bench-work/ws/bfp43f-relay-order-comments-av-1/relay-order-comments
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:c9b680fb decision: Collections that hang off an order are named "order-<what>": comments go in "order-comments". Every RecordStore backend takes the collection name as an opaque key, so any string works.
  anchors: tallyho/repository.py:Repository
- rec:bbc27ba8 decision: Workflow.cancel takes only draft orders, the same rule Order.add and O…
- rec:134f6cbe decision: RecordStore.delete raises NotFound for an id that is not there, as get…
- rec:c1ce3c77 decision: CSV is written by hand, not with the csv module, so the bytes stay the…

## Code
- sym:1589 Repository.order (tallyho/repository.py:57-59)
- sym:1581 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:1591 Repository.orders (tallyho/repository.py:72-74)
- sym:1590 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:1588 Repository.save_order (tallyho/repository.py:54-55)
- sym:1619 summarize (tallyho/text.py:33-56)
- sym:1526 Order (tallyho/models.py:64-109)
  class Order:
- sym:1564 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:1611 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:1497 cmd_export (tallyho/cli.py:26-28)

---
tokens: 701/800
omitted 13 lower-ranked items (expand an id for detail):
- rec:3e9ce503 decision: text.pad returns exactly width characters: it pads short text and cuts…
- rec:24768145 decision: Coupon.expires is the first day a coupon no longer works: one that exp…
- rec:9471444c decision: apply_discounts adds the percentages up and takes them off the subtota…
- rec:7fc2d584 decision: Raise ValidationError with field= naming the input that is wrong: str(…
- rec:2f2cabe3 decision: Ledger.balances() gives each account as the shop reports it: cash, rev…
- rec:a4102c2a decision: Tax regions read from [tax] are upper-cased: configparser lower-cases…
- rec:b844e2d3 intent: Earlier work on this code
- … and 6 more
