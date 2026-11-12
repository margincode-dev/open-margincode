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
- rec:9df5ea80 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:91b87f9c decision: Type hints on `quote_order` use the built-in generics (list[str], dict…
- rec:79038cc1 note: Benchmarks for `quote_order` are in a separate, manual job; the unit t…
- rec:667197fd attempt: Moving the imports inside `OrderLine.subtotal` to speed up start-up sa…

## Code
- sym:12150 Order (tallyho/models.py:64-109)
  class Order:
- sym:12191 quote_order (tallyho/pricing/quote.py:30-53)
- sym:12152 Order.add (tallyho/models.py:76-84)
- sym:12153 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:12190 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:12151 Order.currency (tallyho/models.py:73-74)
- sym:12189 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:12143 Product (tallyho/models.py:23-35)
- sym:12139 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:12120 cmd_products (tallyho/cli.py:20-23)
- sym:12147 OrderLine (tallyho/models.py:51-60)
- sym:12205 Repository (tallyho/repository.py:32-74)

---
tokens: 704/800
omitted 87 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:fc57615b attempt: Replacing the loop in `Quote.lines` with a comprehension made it short…
- rec:ed83cee2 attempt: Adding type: ignore comments around `Repository.product` to quiet the…
- rec:ac1683a9 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:331115a5 attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:1fcbb472 attempt: Moving the imports inside `TaxTable.tax` to speed up start-up saved un…
- … and 81 more
