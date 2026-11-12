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
- rec:b2b5be97 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:c9a10580 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:6db6e901 decision: Constants that `Ledger.record_sale` depends on live at the top of its module in UPPER_CASE, not as magic numbers inline.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:2340f684 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…

## Code
- sym:6993 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:6996 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:6991 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:7112 Workflow.pay (tallyho/workflow.py:41-44)
- sym:6992 Ledger.post (tallyho/ledger.py:28-35)
- sym:7045 quote_order (tallyho/pricing/quote.py:30-53)
- sym:7043 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:7004 Order (tallyho/models.py:64-109)
  class Order:
- sym:6994 Ledger.balances (tallyho/ledger.py:46-50)
- sym:6990 Entry (tallyho/ledger.py:15-20)
- sym:6984 NotFound (tallyho/errors.py:24-25)

---
tokens: 714/800
omitted 51 lower-ranked items (expand an id for detail):
- rec:55e3561c note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:3340ea8c attempt: Moving the imports inside `Money.zero` to speed up start-up saved unde…
- rec:7b8df271 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:b999e5e6 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:9686c231 decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- rec:9f220a4d decision: The `ledger` module keeps its docstring as the one place its behaviour…
- … and 45 more
