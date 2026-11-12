## Task
# relay-bus-resubscribe · relay-bus-resubscribe · open
Goal: Some handlers end up subscribed to the same event twice and then run twice every time it is published. Make `EventBus.subscribe` ignore a handler that is already subscribed to that event.
Workspace: /bench/bench-work/ws/bfp12175-relay-bus-resubscribe-av-1/relay-bus-resubscribe
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
    __init__.py
    cli.py
    config.py
    errors.py
    ledger.py
    models.py
    money.py
    pricing/ (5 files)
    report/ (3 files)
    repository.py
    store/ (5 files)
    text.py
    validate.py
    workflow.py
  tests/ (7 files)
    conftest.py
    test_models.py
    test_money.py
    test_pricing.py
    test_report.py
    test_store.py
    test_text.py

## Decisions & dead ends
- rec:3a015e62 decision: EventBus.subscribe dedupes by name: skip a handler when one with the same __qualname__ is already on that event. Comparing the function objects misses re-imports, which create new objects.
  anchors: tallyho/events.py:EventBus.subscribe
  Seen with the mailer plugin: the worker imports it twice, so its handler was a second object and ran twice per order. Keep the first one subscribed.

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
  def __init__(self, ledger: Ledger, bus: EventBus | None = None, coupons: CouponBook | None = None) -> None:
- sym:302 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:167 _repo (tallyho/cli.py:15-17)
  def _repo(cfg: Config, dry_run: bool) -> Repository:
- sym:173 TallyError (tallyho/errors.py:4-5)
  class TallyError(Exception):

---
tokens: 663/800
