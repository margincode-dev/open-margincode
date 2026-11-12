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
- rec:c9a10580 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:6db6e901 decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:b2b5be97 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:2340f684 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…

## Code
- sym:6993 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:7112 Workflow.pay (tallyho/workflow.py:41-44)
- sym:6991 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:7045 quote_order (tallyho/pricing/quote.py:30-53)
- sym:7043 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:6996 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:7108 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:7073 RecordStore.get (tallyho/store/base.py:28-29)
- sym:7068 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:6984 NotFound (tallyho/errors.py:24-25)
- sym:7107 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 713/800
omitted 58 lower-ranked items (expand an id for detail):
- rec:55e3561c note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:459df446 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:1940bb54 attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:cdd00375 attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:3340ea8c attempt: Moving the imports inside `Money.zero` to speed up start-up saved unde…
- rec:7b8df271 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- … and 52 more
