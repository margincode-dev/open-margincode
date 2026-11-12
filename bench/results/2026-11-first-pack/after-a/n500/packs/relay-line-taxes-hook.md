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
- rec:484c21b4 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:91ab8ca2 attempt: Caching what `quote_order` returns in a module-level dict broke tests…
- rec:82063849 decision: New optional parameters of `quote_order` are keyword-only (after *), s…
- rec:97553395 decision: `quote_order` keeps its docstring as the one place its behaviour is de…

## Code
- sym:21031 quote_order (tallyho/pricing/quote.py:30-53)
- sym:21030 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:21029 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:20990 Order (tallyho/models.py:64-109)
  class Order:
- sym:20992 Order.add (tallyho/models.py:76-84)
- sym:21034 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:20993 Order.remove (tallyho/models.py:86-89)
- sym:20983 Product (tallyho/models.py:23-35)
- sym:21045 Repository (tallyho/repository.py:32-74)
- sym:21114 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:20989 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:20991 Order.currency (tallyho/models.py:73-74)

---
tokens: 713/800
omitted 162 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:b3587a12 decision: Type hints on `quote_order` use the built-in generics (list[str], dict…
- rec:4b0e5414 note: Remember `quote_order` is not thread-safe; each worker needs its own i…
- rec:62f87a94 note: Deprecation warnings from `quote_order` are turned into errors in CI (…
- rec:02e896c1 note: Benchmarks for `quote_order` are in a separate, manual job; the unit t…
- rec:f19c6d2d attempt: Caching what `TaxTable.tax` returns in a module-level dict broke tests…
- … and 156 more
