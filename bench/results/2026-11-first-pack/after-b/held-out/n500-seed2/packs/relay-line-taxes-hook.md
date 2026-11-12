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
- rec:231681e5 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:b10f6d71 note: The repr of objects around `TaxTable.rate` shows up in logs, so keep i…
- rec:9415b628 attempt: Moving the imports inside `quote_order` to speed up start-up saved und…
- rec:6e56b143 attempt: Replacing the loop in `quote_order` with a comprehension made it short…

## Code
- sym:39209 quote_order (tallyho/pricing/quote.py:30-53)
- sym:39208 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:39207 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:39168 Order (tallyho/models.py:64-109)
  class Order:
- sym:39170 Order.add (tallyho/models.py:76-84)
- sym:39212 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:39171 Order.remove (tallyho/models.py:86-89)
- sym:39161 Product (tallyho/models.py:23-35)
- sym:39223 Repository (tallyho/repository.py:32-74)
- sym:39292 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:39167 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:39169 Order.currency (tallyho/models.py:73-74)

---
tokens: 706/800
omitted 164 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:505df011 attempt: Tried calling `quote_order` twice to force a refresh; the second call…
- rec:0b3f3cb5 attempt: Replacing the loop in `default_tax_table` with a comprehension made it…
- rec:2956fab3 attempt: Caching what `Order.currency` returns in a module-level dict broke tes…
- rec:9e07552f attempt: Caching what `TaxTable` returns in a module-level dict broke tests tha…
- rec:b526aa35 attempt: Caching what `CouponBook.discounts_for` returns in a module-level dict…
- … and 158 more
