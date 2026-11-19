## Task
# relay-bus-resubscribe · relay-bus-resubscribe · open
Goal: Some handlers end up subscribed to the same event twice and then run twice every time it is published. Make `EventBus.subscribe` ignore a handler that is already subscribed to that event.
Workspace: /m2b/r2/bench-work/ws/bfp3d41-relay-bus-resubscribe-av-1/relay-bus-resubscribe
Changed (0): nothing yet

## Map
- tallyho/events.py
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:2349c872 decision: Merge orders with self.lines.extend(other.lines) after the draft check. Do not loop over Order.add: it rescans every line for each product and replaces line objects that other code already holds.
  anchors: tallyho/models.py:Order
- rec:49870e54 decision: EventBus.subscribe dedupes by name: skip a handler when one with the s… (disputed)
- rec:f903b9dc decision: Event payloads carry money as integer minor units plus the currency co…
- rec:b1490e0e decision: Errors about customer data must not echo the rejected value: they are…
- rec:46c6731c intent: Earlier work on this code

## Code
- sym:1012 EventBus.subscribe (tallyho/events.py:16-17)
- sym:1010 EventBus (tallyho/events.py:11-22)
  class EventBus:
- sym:1011 EventBus.__init__ (tallyho/events.py:12-14)
  def __init__(self) -> None:
- sym:1013 EventBus.publish (tallyho/events.py:19-22)
- sym:1133 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:1132 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:1138 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:1134 Workflow._move (tallyho/workflow.py:29-33)
- sym:997 _repo (tallyho/cli.py:15-17)
- sym:1003 TallyError (tallyho/errors.py:4-5)

---
tokens: 690/800
omitted 9 lower-ranked items (expand an id for detail):
- rec:b4942553 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:9210c605 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:37c5fb54 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:592c4721 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:112a46d8 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku… (disputed)
- … and 4 more
