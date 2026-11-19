## Task
# relay-bus-resubscribe · relay-bus-resubscribe · open
Goal: Some handlers end up subscribed to the same event twice and then run twice every time it is published. Make `EventBus.subscribe` ignore a handler that is already subscribed to that event.
Workspace: /m2b/r2/bench-work/ws/bfp3be6-relay-bus-resubscribe-av-1/relay-bus-resubscribe
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
- rec:ef40a917 decision: EventBus.subscribe dedupes by name: skip a handler when one with the same __qualname__ is already on that event. Comparing the function objects misses re-imports, which create new objects.
  anchors: tallyho/events.py:EventBus.subscribe
- rec:221ce87e decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:24c9c420 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:795664c0 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…

## Code
- sym:514 EventBus.subscribe (tallyho/events.py:16-17)
  def subscribe(self, event: str, handler: Handler) -> None:
- sym:512 EventBus (tallyho/events.py:11-22)
  class EventBus:
- sym:513 EventBus.__init__ (tallyho/events.py:12-14)
  def __init__(self) -> None:
- sym:515 EventBus.publish (tallyho/events.py:19-22)
- sym:635 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:634 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:640 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:636 Workflow._move (tallyho/workflow.py:29-33)
- sym:499 _repo (tallyho/cli.py:15-17)
- sym:505 TallyError (tallyho/errors.py:4-5)
  class TallyError(Exception):

---
tokens: 702/800
omitted 10 lower-ranked items (expand an id for detail):
- rec:121e95cb decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on t…
- rec:e928021c decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:8f02bbc1 decision: Errors about customer data must not echo the rejected value: they are…
- rec:36d9072c decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:524dc1fb decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:46260f63 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- … and 4 more
