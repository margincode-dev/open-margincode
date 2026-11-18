## Task
# relay-bus-resubscribe · relay-bus-resubscribe · open
Goal: Some handlers end up subscribed to the same event twice and then run twice every time it is published. Make `EventBus.subscribe` ignore a handler that is already subscribed to that event.
Workspace: /m2b/bench-work/ws/bfp2e5-relay-bus-resubscribe-av-1/relay-bus-resubscribe
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
- rec:078bb7f9 decision: EventBus.subscribe dedupes by name: skip a handler when one with the same __qualname__ is already on that event. Comparing the function objects misses re-imports, which create new objects.
  anchors: tallyho/events.py:EventBus.subscribe
- rec:b4a86ea6 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:1807918c decision: Event payloads carry money as integer minor units plus the currency co…
- rec:ad9dbe79 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:e1ea2af0 intent: Earlier work on this code

## Code
- sym:680 EventBus.subscribe (tallyho/events.py:16-17)
  def subscribe(self, event: str, handler: Handler) -> None:
- sym:678 EventBus (tallyho/events.py:11-22)
  class EventBus:
- sym:679 EventBus.__init__ (tallyho/events.py:12-14)
  def __init__(self) -> None:
- sym:681 EventBus.publish (tallyho/events.py:19-22)
- sym:801 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:800 Workflow (tallyho/workflow.py:23-50)
- sym:806 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:802 Workflow._move (tallyho/workflow.py:29-33)
- sym:665 _repo (tallyho/cli.py:15-17)
- sym:671 TallyError (tallyho/errors.py:4-5)

---
tokens: 708/800
omitted 9 lower-ranked items (expand an id for detail):
- rec:cf63358b decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on t…
- rec:824a8a37 decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:e12653aa decision: Errors about customer data must not echo the rejected value: they are…
- rec:7668436d decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:db8aaa56 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:3e6b9e85 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- … and 3 more
