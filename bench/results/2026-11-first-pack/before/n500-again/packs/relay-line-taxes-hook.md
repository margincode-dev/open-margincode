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
- rec:9a385b08 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:b998a1b8 attempt: Caching what `quote_order` returns in a module-level dict broke tests…
- rec:085cdb6a decision: New optional parameters of `quote_order` are keyword-only (after *), s…
- rec:f0c368e4 decision: `quote_order` keeps its docstring as the one place its behaviour is de…

## Code
- sym:10531 quote_order (tallyho/pricing/quote.py:30-53)
- sym:10530 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:10529 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:10490 Order (tallyho/models.py:64-109)
  class Order:
- sym:10492 Order.add (tallyho/models.py:76-84)
- sym:10534 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:10493 Order.remove (tallyho/models.py:86-89)
- sym:10483 Product (tallyho/models.py:23-35)
- sym:10545 Repository (tallyho/repository.py:32-74)
- sym:10614 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:10489 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:10491 Order.currency (tallyho/models.py:73-74)

---
tokens: 714/800
omitted 162 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:bfe6c5df decision: Type hints on `quote_order` use the built-in generics (list[str], dict…
- rec:2eac8e66 note: Remember `quote_order` is not thread-safe; each worker needs its own i…
- rec:e01aa700 note: Deprecation warnings from `quote_order` are turned into errors in CI (…
- rec:6de9e142 note: Benchmarks for `quote_order` are in a separate, manual job; the unit t…
- rec:00e7d746 attempt: Caching what `TaxTable.tax` returns in a module-level dict broke tests…
- … and 156 more
