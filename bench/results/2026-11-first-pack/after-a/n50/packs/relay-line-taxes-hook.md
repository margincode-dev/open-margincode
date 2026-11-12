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
- rec:8e77f0b0 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:9faefa8b attempt: Caching what `Discount.amount_off` returns in a module-level dict brok…
- rec:bed49812 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:e56dd7da decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:19371 quote_order (tallyho/pricing/quote.py:30-53)
- sym:19370 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:19369 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:19330 Order (tallyho/models.py:64-109)
- sym:19332 Order.add (tallyho/models.py:76-84)
- sym:19374 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:19333 Order.remove (tallyho/models.py:86-89)
- sym:19323 Product (tallyho/models.py:23-35)
- sym:19385 Repository (tallyho/repository.py:32-74)
- sym:19454 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:19329 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:19331 Order.currency (tallyho/models.py:73-74)

---
tokens: 710/800
omitted 50 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:5799aebe decision: New optional parameters of the `discounts` module are keyword-only (af…
- rec:cf198624 note: `TaxTable.tax` is on the hot path of the nightly batch job; profile be…
- rec:7185cb9d note: Benchmarks for the `tax` module are in a separate, manual job; the uni…
- rec:989e9e0b attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:6be421bd attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- … and 44 more
