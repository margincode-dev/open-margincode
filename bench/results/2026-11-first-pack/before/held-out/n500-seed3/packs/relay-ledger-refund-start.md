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
- rec:d075ddcf attempt: Tried calling the `quote` module twice to force a refresh; the second…

## Code
- sym:15293 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:15345 quote_order (tallyho/pricing/quote.py:30-53)
- sym:15291 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:15412 Workflow.pay (tallyho/workflow.py:41-44)
- sym:15343 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:15296 OrderStatus (tallyho/models.py:14-19)
- sym:15373 RecordStore.get (tallyho/store/base.py:28-29)
- sym:15408 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:15368 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:15284 NotFound (tallyho/errors.py:24-25)
- sym:15359 Repository (tallyho/repository.py:32-74)
- sym:15369 Repository.orders (tallyho/repository.py:72-74)

---
tokens: 713/800
omitted 187 lower-ranked items (expand an id for detail):
- rec:ef33de59 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:4a1d2099 attempt: Tried calling `Order.currency` twice to force a refresh; the second ca…
- rec:1d6b0c32 attempt: Caching what `Ledger.post` returns in a module-level dict broke tests…
- rec:950f6e44 attempt: Replacing the loop in `Money.zero` with a comprehension made it shorte…
- rec:69e9e8bd attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:74ca93cf attempt: Adding type: ignore comments around `Ledger.post` to quiet the checker…
- … and 181 more
