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
- rec:2dc2a978 decision, open: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:242e33a5 attempt, open: Tried calling `Ledger.record_sale` twice to force a refresh; the secon…
- rec:28d1f4ad attempt, open: Caching what `Ledger.record_sale` returns in a module-level dict broke…

## Code
- sym:5333 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:5452 Workflow.pay (tallyho/workflow.py:41-44)
- sym:5385 quote_order (tallyho/pricing/quote.py:30-53)
- sym:5383 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:5336 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:5448 Workflow (tallyho/workflow.py:23-50)
- sym:5331 Ledger (tallyho/ledger.py:24-53)
- sym:5447 _ALLOWED (tallyho/workflow.py:14-20)
- sym:5332 Ledger.post (tallyho/ledger.py:28-35)
- sym:5345 Order.currency (tallyho/models.py:73-74)
- sym:5353 Money (tallyho/money.py:22-83)
- sym:5355 Money.zero (tallyho/money.py:36-37)

---
tokens: 707/800
omitted 198 lower-ranked items (expand an id for detail):
- rec:23fec861 attempt, open: Moving the imports inside `Ledger.record_sale` to speed up start-up sa…
- rec:bd8dbba1 attempt, open: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:522d973a decision, open: `Quote` keeps its docstring as the one place its behaviour is describe…
- rec:14cd5de8 decision, open: New optional parameters of `Ledger.record_sale` are keyword-only (afte…
- rec:50962e79 decision, open: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- … and 193 more
