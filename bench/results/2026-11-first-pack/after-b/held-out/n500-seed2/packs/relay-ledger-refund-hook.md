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
- rec:97c09cc8 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:8ec3fa36 attempt: Caching what `Ledger.record_sale` returns in a module-level dict broke…
- rec:7af98ca8 decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- rec:e198fe7c attempt: Tried calling `quote_order` twice to force a refresh; the second call…

## Code
- sym:38991 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:39110 Workflow.pay (tallyho/workflow.py:41-44)
- sym:38989 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:39043 quote_order (tallyho/pricing/quote.py:30-53)
- sym:39041 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:38994 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:39106 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:39071 RecordStore.get (tallyho/store/base.py:28-29)
- sym:39066 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:38982 NotFound (tallyho/errors.py:24-25)
- sym:39105 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 714/800
omitted 201 lower-ranked items (expand an id for detail):
- rec:7b7105a7 attempt: Adding type: ignore comments around `Quote` to quiet the checker hid a…
- rec:764ea496 attempt: Wrapping the `ledger` module in a bare try/except with a retry loop hi…
- rec:218a0c67 attempt: Tried calling `TaxTable.tax` twice to force a refresh; the second call…
- rec:9a9201d2 attempt: Tried calling `Ledger.balances` twice to force a refresh; the second c…
- rec:0c7e4edd attempt: Wrapping `Money.amount` in a bare try/except with a retry loop hid the…
- rec:0734ea87 attempt: Tried calling `default_tax_table` twice to force a refresh; the second…
- … and 195 more
