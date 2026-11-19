## Task
# relay-bus-resubscribe · relay-bus-resubscribe · open
Goal: Some handlers end up subscribed to the same event twice and then run twice every time it is published. Make `EventBus.subscribe` ignore a handler that is already subscribed to that event.
Workspace: /m2b/r3/bench-work/ws/bfp5fc-relay-bus-resubscribe-av-1/relay-bus-resubscribe
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
- rec:356b2749 decision: EventBus.subscribe dedupes by name: skip a handler when one with the same __qualname__ is already on that event. Comparing the function objects misses re-imports, which create new objects.
  anchors: tallyho/events.py:EventBus.subscribe
- rec:2b363341 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:f9e31a85 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:0cd4d256 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:dab90967 intent: Earlier work on this code

## Code
- sym:1676 EventBus.subscribe (tallyho/events.py:16-17)
- sym:1674 EventBus (tallyho/events.py:11-22)
  class EventBus:
- sym:1675 EventBus.__init__ (tallyho/events.py:12-14)
  def __init__(self) -> None:
- sym:1677 EventBus.publish (tallyho/events.py:19-22)
- sym:1797 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:1796 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:1802 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:1798 Workflow._move (tallyho/workflow.py:29-33)
- sym:1661 _repo (tallyho/cli.py:15-17)
- sym:1667 TallyError (tallyho/errors.py:4-5)

---
tokens: 707/800
omitted 9 lower-ranked items (expand an id for detail):
- rec:7ce8980b decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on t…
- rec:330009c8 decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:97a851dd decision: Errors about customer data must not echo the rejected value: they are…
- rec:42d92f22 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:a79a77a2 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:cd90e6c9 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- … and 3 more
