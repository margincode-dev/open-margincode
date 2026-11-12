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
- rec:e48679c4 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:8e11d5d7 note: The repr of objects around the `quote` module shows up in logs, so kee…
- rec:6e9102ca attempt: Caching what `quote_order` returns in a module-level dict broke tests…
- rec:dedcb459 decision: New optional parameters of `quote_order` are keyword-only (after *), s…

## Code
- sym:35889 quote_order (tallyho/pricing/quote.py:30-53)
- sym:35888 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:35887 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:35848 Order (tallyho/models.py:64-109)
- sym:35850 Order.add (tallyho/models.py:76-84)
- sym:35892 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:35851 Order.remove (tallyho/models.py:86-89)
- sym:35841 Product (tallyho/models.py:23-35)
- sym:35903 Repository (tallyho/repository.py:32-74)
- sym:35972 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:35847 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:35849 Order.currency (tallyho/models.py:73-74)

---
tokens: 710/800
omitted 175 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:b22b52f6 decision: `quote_order` keeps its docstring as the one place its behaviour is de…
- rec:96ff124d decision: Type hints on `quote_order` use the built-in generics (list[str], dict…
- rec:30a31937 note: Remember `quote_order` is not thread-safe; each worker needs its own i…
- rec:a7bf9b31 note: Deprecation warnings from `quote_order` are turned into errors in CI (…
- rec:539a1929 note: Benchmarks for `quote_order` are in a separate, manual job; the unit t…
- … and 169 more
