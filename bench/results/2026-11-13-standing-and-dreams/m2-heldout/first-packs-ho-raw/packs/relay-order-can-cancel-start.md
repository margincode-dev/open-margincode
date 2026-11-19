## Task
# relay-order-can-cancel · relay-order-can-cancel · open
Goal: The order page hides its Cancel button once an order can no longer be cancelled. Add `Order.can_cancel()`: True while the order can still be cancelled, False once it cannot.
Workspace: /m2b/r3/bench-work/ws/bfp43f-relay-order-can-cancel-av-1/relay-order-can-cancel
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
    __init__.py
    cli.py
    config.py
    errors.py
    events.py
    ledger.py
    models.py
    money.py
    … +7 more
  tests/ (7 files)
    conftest.py
    test_models.py
    test_money.py
    test_pricing.py
    test_report.py
    test_store.py
    test_text.py

## Code
- sym:1352 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:1360 Order (tallyho/models.py:64-109)
  class Order:
- sym:1470 Workflow.cancel (tallyho/workflow.py:49-50)
  def cancel(self, order: Order) -> None:
- sym:1357 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:1464 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:1358 OrderLine.__post_init__ (tallyho/models.py:55-56)
  def __post_init__(self) -> None:
- sym:1463 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:1349 Ledger.record_sale (tallyho/ledger.py:37-44)
  def record_sale(self, on: date, order_id: str, quote: Quote) -> None:
- sym:1466 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:1334 load_config (tallyho/config.py:23-47)
- sym:1359 OrderLine.subtotal (tallyho/models.py:59-60)
  def subtotal(self) -> Money:
- sym:1340 NotFound (tallyho/errors.py:24-25)
- sym:1431 RecordStore.list_page (tallyho/store/base.py:36-37)
- sym:1354 Product.__post_init__ (tallyho/models.py:30-35)
- sym:1449 iter_records (tallyho/store/scan.py:10-20)
- sym:1336 ValidationError (tallyho/errors.py:8-17)
- sym:1345 EventBus.publish (tallyho/events.py:19-22)
- sym:1423 Repository.order (tallyho/repository.py:57-59)
- sym:1467 Workflow.place (tallyho/workflow.py:35-39)

---
tokens: 680/800
