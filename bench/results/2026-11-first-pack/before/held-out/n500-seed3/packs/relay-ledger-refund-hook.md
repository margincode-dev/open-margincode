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
- rec:b809eba4 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:5ffd0efa decision: Constants that `Quote` depends on live at the top of its module in UPP…
- rec:769fd332 note: Benchmarks for `Quote` are in a separate, manual job; the unit tests m…
- rec:b7226dc8 attempt: Tried calling `Ledger.balances` twice to force a refresh; the second c…

## Code
- sym:15293 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:15412 Workflow.pay (tallyho/workflow.py:41-44)
- sym:15291 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:15345 quote_order (tallyho/pricing/quote.py:30-53)
- sym:15343 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:15296 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:15408 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:15373 RecordStore.get (tallyho/store/base.py:28-29)
- sym:15368 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:15284 NotFound (tallyho/errors.py:24-25)
- sym:15407 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 710/800
omitted 192 lower-ranked items (expand an id for detail):
- rec:ea5869d5 attempt: Wrapping `Money._check` in a bare try/except with a retry loop hid the…
- rec:35f99bcc attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:6dafa8ec attempt: Wrapping `RecordStore.put` in a bare try/except with a retry loop hid…
- rec:acdf0466 attempt: Tried calling `RecordStore.list_page` twice to force a refresh; the se…
- rec:4b5635b6 decision: New optional parameters of `Repository._order_from_record` are keyword…
- rec:d075ddcf attempt: Tried calling the `quote` module twice to force a refresh; the second…
- … and 186 more
