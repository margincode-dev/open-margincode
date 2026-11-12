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
- rec:67494c6d decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:ac6654fa note: Remember `CouponBook.discounts_for` is not thread-safe; each worker ne…
- rec:ba79aa62 decision: Constants that `Quote` depends on live at the top of its module in UPP…
- rec:79416c67 attempt: Tried calling the `quote` module twice to force a refresh; the second…

## Code
- sym:40651 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:40770 Workflow.pay (tallyho/workflow.py:41-44)
- sym:40703 quote_order (tallyho/pricing/quote.py:30-53)
- sym:40701 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:40654 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:40766 Workflow (tallyho/workflow.py:23-50)
- sym:40649 Ledger (tallyho/ledger.py:24-53)
- sym:40765 _ALLOWED (tallyho/workflow.py:14-20)
- sym:40650 Ledger.post (tallyho/ledger.py:28-35)
- sym:40663 Order.currency (tallyho/models.py:73-74)
- sym:40671 Money (tallyho/money.py:22-83)
- sym:40638 ValidationError (tallyho/errors.py:8-17)

---
tokens: 713/800
omitted 182 lower-ranked items (expand an id for detail):
- rec:ce818a08 note: Benchmarks for `Quote` are in a separate, manual job; the unit tests m…
- rec:aaddcd1e attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:e8c8df7e attempt: Tried calling `Order.currency` twice to force a refresh; the second ca…
- rec:d1b7d207 attempt: Caching what `Ledger.post` returns in a module-level dict broke tests…
- rec:4c843dc2 attempt: Replacing the loop in `Money.zero` with a comprehension made it shorte…
- rec:0eba70cd attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- … and 176 more
