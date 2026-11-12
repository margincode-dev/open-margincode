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
- rec:26862461 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:43e85ac9 attempt: Moving the imports inside `Ledger.record_sale` to speed up start-up sa…
- rec:3ef8f0b5 attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real e…
- rec:a8855671 attempt: Adding type: ignore comments around `load_config` to quiet the checker…

## Code
- sym:37331 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:37329 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:37450 Workflow.pay (tallyho/workflow.py:41-44)
- sym:37330 Ledger.post (tallyho/ledger.py:28-35)
- sym:37383 quote_order (tallyho/pricing/quote.py:30-53)
- sym:37381 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:37342 Order (tallyho/models.py:64-109)
  class Order:
- sym:37355 Money.__add__ (tallyho/money.py:45-47)
  def __add__(self, other: Money) -> Money:
- sym:37334 OrderStatus (tallyho/models.py:14-19)
- sym:37332 Ledger.balances (tallyho/ledger.py:46-50)
- sym:37322 NotFound (tallyho/errors.py:24-25)

---
tokens: 718/800
omitted 105 lower-ranked items (expand an id for detail):
- rec:1b660355 decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- rec:8df55b22 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:39a640b4 decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:efaf8c65 note: The repr of objects around `Quote` shows up in logs, so keep it short…
- rec:a9562f78 note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:259066e4 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- … and 99 more
