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
- rec:76ea1e34 note: The repr of objects around `check_sku` shows up in logs, so keep it sh…
- rec:079f0eae decision: Constants that `quote_order` depends on live at the top of its module…
- rec:2a5016d4 attempt: Caching what `default_tax_table` returns in a module-level dict broke…

## Code
- sym:40869 quote_order (tallyho/pricing/quote.py:30-53)
- sym:40868 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:40867 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:40828 Order (tallyho/models.py:64-109)
- sym:40830 Order.add (tallyho/models.py:76-84)
- sym:40872 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:40831 Order.remove (tallyho/models.py:86-89)
- sym:40821 Product (tallyho/models.py:23-35)
- sym:40883 Repository (tallyho/repository.py:32-74)
- sym:40952 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:40827 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:40829 Order.currency (tallyho/models.py:73-74)

---
tokens: 709/800
omitted 159 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:8cdc5c59 attempt: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:20a9a0f2 decision: Constants that `TaxTable.tax` depends on live at the top of its module…
- rec:726cc2b6 decision: Constants that `Quote.lines` depends on live at the top of its module…
- rec:787b9162 decision: Constants that `OrderLine.subtotal` depends on live at the top of its…
- rec:87f450ab decision: New optional parameters of `CouponBook` are keyword-only (after *), so…
- … and 153 more
