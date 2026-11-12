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
- rec:8df71c0b decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:f735a409 decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:e31808d7 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:da3a29ec attempt: Moving the imports inside `Money.zero` to speed up start-up saved unde…

## Code
- sym:19153 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:19272 Workflow.pay (tallyho/workflow.py:41-44)
- sym:19151 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:19205 quote_order (tallyho/pricing/quote.py:30-53)
- sym:19203 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:19156 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:19268 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:19233 RecordStore.get (tallyho/store/base.py:28-29)
- sym:19228 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:19144 NotFound (tallyho/errors.py:24-25)
- sym:19267 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 718/800
omitted 58 lower-ranked items (expand an id for detail):
- rec:37ead494 note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:da2a5632 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:fe6976c3 attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:7a9b94c6 attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:959d747a attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:5db1ae78 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- … and 52 more
