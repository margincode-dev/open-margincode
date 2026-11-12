## Task
# relay-line-taxes · relay-line-taxes · open

## Map
- tallyho/pricing/quote.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/pricing/tax.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- tallyho/workflow.py
- tests/conftest.py
- tests/test_pricing.py
- tallyho/cli.py
- tallyho/config.py
- tallyho/ledger.py

## Decisions & dead ends
- rec:1efc4965 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:204c07c9 note: The repr of objects around `Quote` shows up in logs, so keep it short…
- rec:4aef00fc decision: Type hints on `quote_order` use the built-in generics (list[str], dict…
- rec:38f336eb note: Benchmarks for `quote_order` are in a separate, manual job; the unit t…

## Code
- sym:37508 Order (tallyho/models.py:64-109)
  class Order:
- sym:37549 quote_order (tallyho/pricing/quote.py:30-53)
- sym:37510 Order.add (tallyho/models.py:76-84)
- sym:37511 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:37548 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:37509 Order.currency (tallyho/models.py:73-74)
- sym:37547 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:37501 Product (tallyho/models.py:23-35)
- sym:37497 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:37478 cmd_products (tallyho/cli.py:20-23)
- sym:37505 OrderLine (tallyho/models.py:51-60)
- sym:37563 Repository (tallyho/repository.py:32-74)

---
tokens: 710/800
omitted 87 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:922c8e7a attempt: Moving the imports inside `OrderLine.subtotal` to speed up start-up sa…
- rec:e25b9245 attempt: Replacing the loop in `Quote.lines` with a comprehension made it short…
- rec:8e46c4bc attempt: Adding type: ignore comments around `Repository.product` to quiet the…
- rec:08ed789f attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:8d984db5 attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- … and 81 more
