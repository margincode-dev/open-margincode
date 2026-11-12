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
- rec:11364a0a attempt: Adding type: ignore comments around `Quote` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/pricing/quote.py:Quote
- rec:0818f094 attempt: Caching what `Ledger.record_sale` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:c7f0e9a8 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:f086a36d attempt: Moving the imports inside `quote_order` to speed up start-up saved und…

## Code
- sym:13633 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:13685 quote_order (tallyho/pricing/quote.py:30-53)
- sym:13631 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:13752 Workflow.pay (tallyho/workflow.py:41-44)
- sym:13683 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:13636 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:13713 RecordStore.get (tallyho/store/base.py:28-29)
- sym:13748 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:13708 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:13624 NotFound (tallyho/errors.py:24-25)
- sym:13699 Repository (tallyho/repository.py:32-74)
  class Repository:

---
tokens: 717/800
omitted 187 lower-ranked items (expand an id for detail):
- rec:f03d65d9 decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- rec:5bce34b1 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:26322d84 attempt: Replacing the loop in `quote_order` with a comprehension made it short…
- rec:5f3e5fda attempt: Tried calling `TaxTable.tax` twice to force a refresh; the second call…
- rec:0361b24d attempt: Adding type: ignore comments around `total` to quiet the checker hid a…
- rec:5807d5b8 attempt: Wrapping the `ledger` module in a bare try/except with a retry loop hi…
- … and 181 more
