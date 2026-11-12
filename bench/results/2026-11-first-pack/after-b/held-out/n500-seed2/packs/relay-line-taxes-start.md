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
- rec:4a4311fb note: The repr of objects around `default_tax_table` shows up in logs, so ke…
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
- sym:39161 Product (tallyho/models.py:23-35)
- sym:39171 Order.remove (tallyho/models.py:86-89)
- sym:39223 Repository (tallyho/repository.py:32-74)
- sym:39232 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:39167 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:39169 Order.currency (tallyho/models.py:73-74)

---
tokens: 722/800
omitted 152 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:505df011 attempt: Tried calling `quote_order` twice to force a refresh; the second call…
- rec:5a03cfef attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:a186f248 attempt: Caching what `Repository.product` returns in a module-level dict broke…
- rec:39122a8b attempt: Replacing the loop in `Repository.product` with a comprehension made i…
- rec:f772534e attempt: Adding type: ignore comments around `Quote` to quiet the checker hid a…
- rec:c418c56e attempt: Tried calling `apply_discounts` twice to force a refresh; the second c…
- … and 145 more
