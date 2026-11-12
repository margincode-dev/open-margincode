## Task
# relay-ledger-refund · relay-ledger-refund · open

## Map
- tallyho/ledger.py
- tallyho/pricing/quote.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/tax.py
- tallyho/workflow.py
- tallyho/config.py
- tallyho/errors.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
- rec:7c313463 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:10794c3e decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:321f9499 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:58ee234f attempt: Moving the imports inside `Ledger.record_sale` to speed up start-up sa…

## Code
- sym:11973 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:12092 Workflow.pay (tallyho/workflow.py:41-44)
- sym:11971 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:12025 quote_order (tallyho/pricing/quote.py:30-53)
- sym:12023 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:11976 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:12088 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:12053 RecordStore.get (tallyho/store/base.py:28-29)
- sym:12048 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:11964 NotFound (tallyho/errors.py:24-25)
- sym:12087 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 724/800
omitted 110 lower-ranked items (expand an id for detail):
- rec:7cf6fd7a decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- rec:73f17fb5 note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:7f3b469e attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:b23e0a49 note: The repr of objects around `Quote` shows up in logs, so keep it short…
- rec:3c0bc70f attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:e286d78b attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- … and 104 more
