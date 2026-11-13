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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:fc9a8169 decision, open: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:ff870c85 note, open: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:32890c24 decision, open: Constants that `Ledger.record_sale` depends on live at the top of its…

## Code
- sym:3673 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:3792 Workflow.pay (tallyho/workflow.py:41-44)
- sym:3671 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:3725 quote_order (tallyho/pricing/quote.py:30-53)
- sym:3723 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:3676 OrderStatus (tallyho/models.py:14-19)
- sym:3788 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:3753 RecordStore.get (tallyho/store/base.py:28-29)
- sym:3748 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:3664 NotFound (tallyho/errors.py:24-25)
- sym:3787 _ALLOWED (tallyho/workflow.py:14-20)
- sym:3739 Repository (tallyho/repository.py:32-74)

---
tokens: 708/800
omitted 74 lower-ranked items (expand an id for detail):
- rec:a1fa0c18 decision, open: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:3558b80e attempt, open: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:cf86dedf attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:4a6f3884 attempt, open: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:6ef6c83e decision, open: The `ledger` module keeps its docstring as the one place its behaviour…
- … and 69 more
