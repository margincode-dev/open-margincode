## Task
# relay-bus-resubscribe · relay-bus-resubscribe · open

## Map
- tallyho/events.py
  L11: class EventBus:
  L12: def __init__(self) -> None:
  L16: def subscribe(self, event: str, handler: Handler) -> None:
  L19: def publish(self, event: str, payload: dict) -> None:
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:3a015e62 decision: EventBus.subscribe dedupes by name: skip a handler when one with the same __qualname__ is already on that event. Comparing the function objects misses re-imports, which create new objects.
  anchors: tallyho/events.py:EventBus.subscribe
- rec:c6ab26d5 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:3f5c35fd decision: Event payloads carry money as integer minor units plus the currency co…
- rec:d2a18149 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…

## Code
- sym:182 EventBus.subscribe (tallyho/events.py:16-17)
  def subscribe(self, event: str, handler: Handler) -> None:
- sym:180 EventBus (tallyho/events.py:11-22)
  class EventBus:
- sym:181 EventBus.__init__ (tallyho/events.py:12-14)
  def __init__(self) -> None:
- sym:183 EventBus.publish (tallyho/events.py:19-22)
  def publish(self, event: str, payload: dict) -> None:
- sym:303 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:302 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:308 Workflow.cancel (tallyho/workflow.py:49-50)
  def cancel(self, order: Order) -> None:
- sym:304 Workflow._move (tallyho/workflow.py:29-33)
- sym:167 _repo (tallyho/cli.py:15-17)
- sym:173 TallyError (tallyho/errors.py:4-5)

---
tokens: 706/800
omitted 10 lower-ranked items (expand an id for detail):
- rec:ee2bb769 decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on t…
- rec:f60ad743 decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:67eaa038 decision: Errors about customer data must not echo the rejected value: they are…
- rec:3bb0cebd decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:38b2b01c decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:1c82b7e8 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- … and 4 more
