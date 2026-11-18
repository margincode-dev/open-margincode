## Task
# relay-bus-resubscribe · relay-bus-resubscribe · open
Goal: Some handlers end up subscribed to the same event twice and then run twice every time it is published. Make `EventBus.subscribe` ignore a handler that is already subscribed to that event.
Workspace: /m2b/bench-work/ws/bfp3fe-relay-bus-resubscribe-av-1/relay-bus-resubscribe
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
- rec:c2a0e353 decision: Merge orders with self.lines.extend(other.lines) after the draft check. Do not loop over Order.add: it rescans every line for each product and replaces line objects that other code already holds.
  anchors: tallyho/models.py:Order
- rec:bb2acf4d decision: EventBus.subscribe dedupes by name: skip a handler when one with the s… (disputed)
- rec:579acf03 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:0b2de262 decision: A refund is booked as the exact reverse of the sale: the same three ac…

## Code
- sym:1178 EventBus.subscribe (tallyho/events.py:16-17)
  def subscribe(self, event: str, handler: Handler) -> None:
- sym:1176 EventBus (tallyho/events.py:11-22)
  class EventBus:
- sym:1177 EventBus.__init__ (tallyho/events.py:12-14)
  def __init__(self) -> None:
- sym:1179 EventBus.publish (tallyho/events.py:19-22)
- sym:1299 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:1298 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:1304 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:1300 Workflow._move (tallyho/workflow.py:29-33)
- sym:1163 _repo (tallyho/cli.py:15-17)
- sym:1169 TallyError (tallyho/errors.py:4-5)

---
tokens: 686/800
omitted 10 lower-ranked items (expand an id for detail):
- rec:3aae3e9e decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:e5a06be1 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:d796c9c5 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:5a487b11 intent: Earlier work on this code
- rec:c16bfd2c decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku… (disputed)
- … and 5 more
