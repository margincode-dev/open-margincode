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
- rec:1b02640c attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:2f3eca67 attempt: Moving the imports inside `quote_order` to speed up start-up saved und…
- rec:bcd520ae attempt: Replacing the loop in `quote_order` with a comprehension made it short…
- rec:3bb87f53 attempt: Tried calling `quote_order` twice to force a refresh; the second call…

## Code
- sym:13851 quote_order (tallyho/pricing/quote.py:30-53)
- sym:13850 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:13849 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:13810 Order (tallyho/models.py:64-109)
  class Order:
- sym:13812 Order.add (tallyho/models.py:76-84)
- sym:13854 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:13813 Order.remove (tallyho/models.py:86-89)
- sym:13803 Product (tallyho/models.py:23-35)
- sym:13865 Repository (tallyho/repository.py:32-74)
- sym:13934 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:13809 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:13811 Order.currency (tallyho/models.py:73-74)

---
tokens: 703/800
omitted 152 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:e5997631 attempt: Replacing the loop in `default_tax_table` with a comprehension made it…
- rec:c6e991bf attempt: Caching what `Order.currency` returns in a module-level dict broke tes…
- rec:6a6e6466 decision: New optional parameters of `default_tax_table` are keyword-only (after…
- rec:54443648 note: The repr of objects around `default_tax_table` shows up in logs, so ke…
- rec:62e35d6e note: The repr of objects around `Order.currency` shows up in logs, so keep…
- … and 146 more
