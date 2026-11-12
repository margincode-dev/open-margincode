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
- rec:58ee234f attempt: Moving the imports inside `Ledger.record_sale` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:7f3b469e attempt: Wrapping `Quote` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/pricing/quote.py:Quote
- rec:7c313463 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:faf5b355 decision: Tests for `Order.currency` build their inputs with small factory helpe…

## Code
- sym:11973 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:11971 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:12092 Workflow.pay (tallyho/workflow.py:41-44)
- sym:11972 Ledger.post (tallyho/ledger.py:28-35)
- sym:12025 quote_order (tallyho/pricing/quote.py:30-53)
- sym:12023 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:11984 Order (tallyho/models.py:64-109)
  class Order:
- sym:11997 Money.__add__ (tallyho/money.py:45-47)
- sym:11976 OrderStatus (tallyho/models.py:14-19)
- sym:11974 Ledger.balances (tallyho/ledger.py:46-50)
- sym:11964 NotFound (tallyho/errors.py:24-25)
  class NotFound(TallyError):

---
tokens: 721/800
omitted 105 lower-ranked items (expand an id for detail):
- rec:7cf6fd7a decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- rec:321f9499 decision: `Ledger.record_sale` keeps its docstring as the one place its behaviou…
- rec:10794c3e decision: Constants that `Ledger.record_sale` depends on live at the top of its…
- rec:b23e0a49 note: The repr of objects around `Quote` shows up in logs, so keep it short…
- rec:73f17fb5 note: Benchmarks for `Ledger.record_sale` are in a separate, manual job; the…
- rec:e069fc4e attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- … and 99 more
