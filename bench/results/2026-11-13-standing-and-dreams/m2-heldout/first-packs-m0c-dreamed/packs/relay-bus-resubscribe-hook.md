## Task
# relay-bus-resubscribe · relay-bus-resubscribe · open
Goal: Some handlers end up subscribed to the same event twice and then run twice every time it is published. Make `EventBus.subscribe` ignore a handler that is already subscribed to that event.
Workspace: /m2b/r3/bench-work/ws/bfp713-relay-bus-resubscribe-av-1/relay-bus-resubscribe
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
- rec:f0234173 decision: Merge orders with self.lines.extend(other.lines) after the draft check. Do not loop over Order.add: it rescans every line for each product and replaces line objects that other code already holds.
  anchors: tallyho/models.py:Order
- rec:3a48e08d decision: EventBus.subscribe dedupes by name: skip a handler when one with the s… (disputed)
- rec:5cfd5d9c decision: Event payloads carry money as integer minor units plus the currency co…
- rec:4ff63c1b decision: Errors about customer data must not echo the rejected value: they are…

## Code
- sym:2174 EventBus.subscribe (tallyho/events.py:16-17)
  def subscribe(self, event: str, handler: Handler) -> None:
- sym:2172 EventBus (tallyho/events.py:11-22)
  class EventBus:
- sym:2173 EventBus.__init__ (tallyho/events.py:12-14)
  def __init__(self) -> None:
- sym:2175 EventBus.publish (tallyho/events.py:19-22)
- sym:2295 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:2294 Workflow (tallyho/workflow.py:23-50)
- sym:2300 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:2296 Workflow._move (tallyho/workflow.py:29-33)
- sym:2159 _repo (tallyho/cli.py:15-17)
- sym:2165 TallyError (tallyho/errors.py:4-5)

---
tokens: 710/800
omitted 10 lower-ranked items (expand an id for detail):
- rec:dafcad34 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:0c44c1a5 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:aa52189e decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:6369c0a0 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:7146c0d4 intent: Earlier work on this code
- rec:bac646e8 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku… (disputed)
- … and 4 more
