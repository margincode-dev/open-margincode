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
- rec:c1a33376 decision: Constants that `TaxTable.tax` depends on live at the top of its module…
- rec:3b5b6650 decision: Constants that `Quote.lines` depends on live at the top of its module…

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
- sym:15473 Order.remove (tallyho/models.py:86-89)
- sym:15463 Product (tallyho/models.py:23-35)
- sym:15525 Repository (tallyho/repository.py:32-74)
- sym:15594 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:15469 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:15471 Order.currency (tallyho/models.py:73-74)

---
tokens: 709/800
omitted 143 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:44ffc60f decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:9a286a81 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:b1f2e763 attempt: Adding type: ignore comments around `apply_discounts` to quiet the che…
- rec:210d16be attempt: Replacing the loop in `Order.subtotal` with a comprehension made it sh…
- rec:0c887029 attempt: Tried calling the `quote` module twice to force a refresh; the second…
- … and 137 more
