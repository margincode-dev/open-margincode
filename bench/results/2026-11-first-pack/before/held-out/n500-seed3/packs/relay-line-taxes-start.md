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
- rec:2511c8cc attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:02fa849a decision: Constants that `quote_order` depends on live at the top of its module…
- rec:b1f2e763 attempt: Adding type: ignore comments around `apply_discounts` to quiet the che…
- rec:210d16be attempt: Replacing the loop in `Order.subtotal` with a comprehension made it sh…

## Code
- sym:15511 quote_order (tallyho/pricing/quote.py:30-53)
- sym:15510 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:15509 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:15470 Order (tallyho/models.py:64-109)
  class Order:
- sym:15472 Order.add (tallyho/models.py:76-84)
- sym:15514 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:15463 Product (tallyho/models.py:23-35)
  class Product:
- sym:15473 Order.remove (tallyho/models.py:86-89)
- sym:15525 Repository (tallyho/repository.py:32-74)
- sym:15534 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:15469 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:15471 Order.currency (tallyho/models.py:73-74)

---
tokens: 711/800
omitted 143 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:0c887029 attempt: Tried calling the `quote` module twice to force a refresh; the second…
- rec:8a5f0364 attempt: Adding type: ignore comments around `Order` to quiet the checker hid a…
- rec:6b760c4c attempt: Wrapping `Order.subtotal` in a bare try/except with a retry loop hid t…
- rec:0faf8a3b attempt: Caching what `default_tax_table` returns in a module-level dict broke…
- rec:e663f237 attempt: Tried calling `Order.currency` twice to force a refresh; the second ca…
- … and 137 more
