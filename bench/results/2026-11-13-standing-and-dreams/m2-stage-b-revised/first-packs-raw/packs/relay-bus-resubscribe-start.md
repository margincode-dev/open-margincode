## Task
# relay-bus-resubscribe · relay-bus-resubscribe · open
Goal: Some handlers end up subscribed to the same event twice and then run twice every time it is published. Make `EventBus.subscribe` ignore a handler that is already subscribed to that event.
Workspace: /m2b/r2/bench-work/ws/bfp3be6-relay-bus-resubscribe-av-1/relay-bus-resubscribe
Changed (0): nothing yet

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
- rec:ef40a917 decision: EventBus.subscribe dedupes by name: skip a handler when one with the same __qualname__ is already on that event. Comparing the function objects misses re-imports, which create new objects.
  anchors: tallyho/events.py:EventBus.subscribe
  Seen with the mailer plugin: the worker imports it twice, so its handler was a second object and ran twice per order. Keep the first one subscribed.

## Code
- sym:514 EventBus.subscribe (tallyho/events.py:16-17)
  def subscribe(self, event: str, handler: Handler) -> None:
- sym:512 EventBus (tallyho/events.py:11-22)
  class EventBus:
- sym:513 EventBus.__init__ (tallyho/events.py:12-14)
  def __init__(self) -> None:
- sym:515 EventBus.publish (tallyho/events.py:19-22)
  def publish(self, event: str, payload: dict) -> None:
- sym:522 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:635 Workflow.__init__ (tallyho/workflow.py:24-27)
  def __init__(self, ledger: Ledger, bus: EventBus | None = None, coupons: CouponBook | None = None) -> None:
- sym:529 OrderLine.subtotal (tallyho/models.py:59-60)
  def subtotal(self) -> Money:
- sym:634 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:499 _repo (tallyho/cli.py:15-17)
  def _repo(cfg: Config, dry_run: bool) -> Repository:
- sym:530 Order (tallyho/models.py:64-109)
  class Order:
- sym:505 TallyError (tallyho/errors.py:4-5)
  class TallyError(Exception):
- sym:527 OrderLine (tallyho/models.py:51-60)
  class OrderLine:

---
tokens: 657/800
