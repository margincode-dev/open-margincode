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
- rec:85c56e1d attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:8c82e04b attempt: Caching what `quote_order` returns in a module-level dict broke tests…
- rec:2ac8f0a7 decision: New optional parameters of `quote_order` are keyword-only (after *), s…
- rec:eb6dde43 decision: `quote_order` keeps its docstring as the one place its behaviour is de…

## Code
- sym:8871 quote_order (tallyho/pricing/quote.py:30-53)
- sym:8870 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:8869 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:8830 Order (tallyho/models.py:64-109)
- sym:8832 Order.add (tallyho/models.py:76-84)
- sym:8874 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:8833 Order.remove (tallyho/models.py:86-89)
- sym:8823 Product (tallyho/models.py:23-35)
- sym:8885 Repository (tallyho/repository.py:32-74)
- sym:8954 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:8829 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:8831 Order.currency (tallyho/models.py:73-74)

---
tokens: 716/800
omitted 162 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:84dd37b1 decision: Type hints on `quote_order` use the built-in generics (list[str], dict…
- rec:31f7b5c9 note: Remember `quote_order` is not thread-safe; each worker needs its own i…
- rec:52cd474e note: Deprecation warnings from `quote_order` are turned into errors in CI (…
- rec:9c12b9b4 note: Benchmarks for `quote_order` are in a separate, manual job; the unit t…
- rec:96989562 attempt: Caching what `TaxTable.tax` returns in a module-level dict broke tests…
- … and 156 more
