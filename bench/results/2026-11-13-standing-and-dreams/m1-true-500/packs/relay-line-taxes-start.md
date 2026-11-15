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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:677014fa attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:ce86c839 note, open: The repr of objects around `Quote` shows up in logs, so keep it short…
- rec:25ccd057 attempt, open: Caching what `quote_order` returns in a module-level dict broke tests…
- rec:7f177746 decision, open: New optional parameters of `quote_order` are keyword-only (after *), s…

## Code
- sym:5551 quote_order (tallyho/pricing/quote.py:30-53)
- sym:5510 Order (tallyho/models.py:64-109)
  class Order:
- sym:5550 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:5549 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:5513 Order.remove (tallyho/models.py:86-89)
- sym:5512 Order.add (tallyho/models.py:76-84)
- sym:5565 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:5509 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:5511 Order.currency (tallyho/models.py:73-74)
- sym:5514 Order.subtotal (tallyho/models.py:92-93)
- sym:5519 Money (tallyho/money.py:22-83)

---
tokens: 704/800
omitted 162 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:0df2ff0f decision, open: `quote_order` keeps its docstring as the one place its behaviour is de…
- rec:86c7e628 decision, open: Type hints on `quote_order` use the built-in generics (list[str], dict…
- rec:047f90e7 note, open: Remember `quote_order` is not thread-safe; each worker needs its own i…
- rec:97a04d7e note, open: Deprecation warnings from `quote_order` are turned into errors in CI (…
- … and 157 more
