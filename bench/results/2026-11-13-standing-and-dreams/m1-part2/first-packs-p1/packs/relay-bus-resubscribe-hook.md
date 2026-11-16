## Task
# relay-bus-resubscribe · relay-bus-resubscribe · open
Goal: Some handlers end up subscribed to the same event twice and then run twice every time it is published. Make `EventBus.subscribe` ignore a handler that is already subscribed to that event.

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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:52a522c4 decision, reverted: EventBus.subscribe dedupes by name: skip a handler when one with the s…
- rec:8932a51f decision, reverted: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:481656fb decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:1ee775e5 intent, open: Earlier work on this code

## Code
- sym:16 EventBus.subscribe (tallyho/events.py:16-17)
  def subscribe(self, event: str, handler: Handler) -> None:
- sym:14 EventBus (tallyho/events.py:11-22)
  class EventBus:
- sym:15 EventBus.__init__ (tallyho/events.py:12-14)
  def __init__(self) -> None:
- sym:17 EventBus.publish (tallyho/events.py:19-22)
- sym:137 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:136 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:142 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:138 Workflow._move (tallyho/workflow.py:29-33)
- sym:1 _repo (tallyho/cli.py:15-17)
- sym:7 TallyError (tallyho/errors.py:4-5)
  class TallyError(Exception):

---
tokens: 700/800
omitted 13 lower-ranked items (expand an id for detail):
- rec:5937840a decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:9130fd41 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:381d8a6d decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:c6241db5 decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:ca82fc11 note, open: merge ee949d93e6b0 reverted by bench-merger in 613f6bf20c93: the bench…
- … and 8 more
