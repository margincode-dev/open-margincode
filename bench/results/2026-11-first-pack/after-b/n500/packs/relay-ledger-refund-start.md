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
- rec:9c319e3b decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:a7ee754e attempt: Tried calling `Ledger.record_sale` twice to force a refresh; the secon…
- rec:25c06d46 attempt: Caching what `Ledger.record_sale` returns in a module-level dict broke…
- rec:088adf4f decision: `Quote` keeps its docstring as the one place its behaviour is describe…

## Code
- sym:35671 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:35723 quote_order (tallyho/pricing/quote.py:30-53)
- sym:35669 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:35790 Workflow.pay (tallyho/workflow.py:41-44)
- sym:35721 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:35674 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:35733 product_to_record (tallyho/repository.py:14-16)
  def product_to_record(p: Product) -> dict:
- sym:35746 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:35786 Workflow (tallyho/workflow.py:23-50)
- sym:35662 NotFound (tallyho/errors.py:24-25)
- sym:35737 Repository (tallyho/repository.py:32-74)

---
tokens: 714/800
omitted 202 lower-ranked items (expand an id for detail):
- rec:70de123d attempt: Moving the imports inside `Ledger.record_sale` to speed up start-up sa…
- rec:61a21147 attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:d86aa027 decision: New optional parameters of `Ledger.record_sale` are keyword-only (afte…
- rec:f233fb4b decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- rec:f0a6590c decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:b61bcd0c decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- … and 196 more
