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
- rec:ff537acd decision: New optional parameters of the `tax` module are keyword-only (after *)…

## Code
- sym:12191 quote_order (tallyho/pricing/quote.py:30-53)
- sym:12190 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:12189 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:12150 Order (tallyho/models.py:64-109)
- sym:12152 Order.add (tallyho/models.py:76-84)
- sym:12194 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:12153 Order.remove (tallyho/models.py:86-89)
- sym:12143 Product (tallyho/models.py:23-35)
- sym:12205 Repository (tallyho/repository.py:32-74)
- sym:12274 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:12149 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:12151 Order.currency (tallyho/models.py:73-74)

---
tokens: 716/800
omitted 87 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:c4be33b3 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:24db6b1c decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:0dea9403 note: The repr of objects around `Quote` shows up in logs, so keep it short…
- rec:57e636a1 note: `TaxTable.tax` is on the hot path of the nightly batch job; profile be…
- rec:813974d9 decision: Constants that `TaxTable.rate` depends on live at the top of its modul…
- … and 81 more
