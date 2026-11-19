## Task
# relay-order-can-cancel · relay-order-can-cancel · open
Goal: The order page hides its Cancel button once an order can no longer be cancelled. Add `Order.can_cancel()`: True while the order can still be cancelled, False once it cannot.
Workspace: /m2b/r3/bench-work/ws/bfpe76-relay-order-can-cancel-av-1/relay-order-can-cancel
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
- sym:4174 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:4182 Order (tallyho/models.py:64-109)
  class Order:
- sym:4292 Workflow.cancel (tallyho/workflow.py:49-50)
  def cancel(self, order: Order) -> None:
- sym:4179 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:4286 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:4180 OrderLine.__post_init__ (tallyho/models.py:55-56)
  def __post_init__(self) -> None:
- sym:4285 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:4171 Ledger.record_sale (tallyho/ledger.py:37-44)
  def record_sale(self, on: date, order_id: str, quote: Quote) -> None:
- sym:4288 Workflow._move (tallyho/workflow.py:29-33)
  def _move(self, order: Order, to: OrderStatus) -> None:
- sym:4156 load_config (tallyho/config.py:23-47)
- sym:4181 OrderLine.subtotal (tallyho/models.py:59-60)
  def subtotal(self) -> Money:
- sym:4162 NotFound (tallyho/errors.py:24-25)
- sym:4253 RecordStore.list_page (tallyho/store/base.py:36-37)
- sym:4176 Product.__post_init__ (tallyho/models.py:30-35)
- sym:4271 iter_records (tallyho/store/scan.py:10-20)
- sym:4158 ValidationError (tallyho/errors.py:8-17)
- sym:4167 EventBus.publish (tallyho/events.py:19-22)
- sym:4245 Repository.order (tallyho/repository.py:57-59)
- sym:4289 Workflow.place (tallyho/workflow.py:35-39)

---
tokens: 680/800
