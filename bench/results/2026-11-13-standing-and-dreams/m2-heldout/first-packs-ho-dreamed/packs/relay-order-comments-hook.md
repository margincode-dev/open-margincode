## Task
# relay-order-comments · relay-order-comments · open
Goal: Staff want to keep short comments on an order. Add `Repository.add_order_comment(oid, text)` and `Repository.order_comments(oid)`, which returns that order's comments as a list of strings, oldest first, and an empty list for an order without any. Keep them in the shop's store, so they survive a restart.
Workspace: /m2b/r3/bench-work/ws/bfpe76-relay-order-comments-av-1/relay-order-comments
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:c64b50be decision: Collections that hang off an order are named "order-<what>": comments… (disputed)
- rec:dbca71e6 decision: Workflow.cancel takes only draft orders, the same rule Order.add and O… (disputed)
- rec:bc93a0f3 decision: CSV is written by hand, not with the csv module, so the bytes stay the…
- rec:2a4c02ea decision: text.pad returns exactly width characters: it pads short text and cuts…

## Code
- sym:4411 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:4403 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:4413 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:4412 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:4410 Repository.save_order (tallyho/repository.py:54-55)
- sym:4441 summarize (tallyho/text.py:33-56)
- sym:4348 Order (tallyho/models.py:64-109)
  class Order:
- sym:4386 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:4433 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:4319 cmd_export (tallyho/cli.py:26-28)

---
tokens: 687/800
omitted 13 lower-ranked items (expand an id for detail):
- rec:b84609f3 decision: apply_discounts adds the percentages up and takes them off the subtota…
- rec:fdc9d0fe decision: Raise ValidationError with field= naming the input that is wrong: str(…
- rec:c958092b decision: Tax regions read from [tax] are upper-cased: configparser lower-cases…
- rec:bbb6eba3 intent: Earlier work on this code
- rec:f81b0351 decision: RecordStore.delete raises NotFound for an id that is not there, as get… (disputed)
- rec:212d6b08 decision: Coupon.expires is the first day a coupon no longer works: one that exp… (disputed)
- … and 7 more
