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
- rec:abde053b attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:f44a846a note: The repr of objects around `OrderLine.subtotal` shows up in logs, so k…
- rec:079f0eae decision: Constants that `quote_order` depends on live at the top of its module…
- rec:fa417f23 attempt: Adding type: ignore comments around `apply_discounts` to quiet the che…

## Code
- sym:40869 quote_order (tallyho/pricing/quote.py:30-53)
- sym:40828 Order (tallyho/models.py:64-109)
  class Order:
- sym:40868 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:40867 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:40831 Order.remove (tallyho/models.py:86-89)
- sym:40830 Order.add (tallyho/models.py:76-84)
- sym:40883 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:40827 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:40829 Order.currency (tallyho/models.py:73-74)
- sym:40832 Order.subtotal (tallyho/models.py:92-93)
- sym:40837 Money (tallyho/money.py:22-83)
  class Money:
- sym:40850 total (tallyho/money.py:86-91)

---
tokens: 713/800
omitted 142 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:f7f39e43 attempt: Replacing the loop in `Order.subtotal` with a comprehension made it sh…
- rec:e3dc2459 attempt: Tried calling the `quote` module twice to force a refresh; the second…
- rec:5d910e0d attempt: Adding type: ignore comments around `Order` to quiet the checker hid a…
- rec:cdf9e5aa attempt: Wrapping `Order.subtotal` in a bare try/except with a retry loop hid t…
- rec:2a5016d4 attempt: Caching what `default_tax_table` returns in a module-level dict broke…
- … and 136 more
