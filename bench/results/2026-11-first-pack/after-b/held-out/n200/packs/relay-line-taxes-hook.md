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
- rec:1efc4965 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:204c07c9 note: The repr of objects around `Quote` shows up in logs, so keep it short…
- rec:4aef00fc decision: Type hints on `quote_order` use the built-in generics (list[str], dict…
- rec:38f336eb note: Benchmarks for `quote_order` are in a separate, manual job; the unit t…

## Code
- sym:37549 quote_order (tallyho/pricing/quote.py:30-53)
- sym:37548 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:37547 Quote (tallyho/pricing/quote.py:16-27)
- sym:37508 Order (tallyho/models.py:64-109)
- sym:37510 Order.add (tallyho/models.py:76-84)
- sym:37552 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:37511 Order.remove (tallyho/models.py:86-89)
- sym:37501 Product (tallyho/models.py:23-35)
- sym:37563 Repository (tallyho/repository.py:32-74)
- sym:37632 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:37507 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:37509 Order.currency (tallyho/models.py:73-74)

---
tokens: 726/800
omitted 101 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:258cae9b attempt: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:e25b9245 attempt: Replacing the loop in `Quote.lines` with a comprehension made it short…
- rec:338ba700 attempt: Moving the imports inside `TaxTable.tax` to speed up start-up saved un…
- rec:a104db94 attempt: Tried calling `TaxTable.tax` twice to force a refresh; the second call…
- rec:68156cd8 decision: New optional parameters of the `tax` module are keyword-only (after *)…
- rec:471a588a attempt: Caching what `Discount.amount_off` returns in a module-level dict brok…
- … and 94 more
