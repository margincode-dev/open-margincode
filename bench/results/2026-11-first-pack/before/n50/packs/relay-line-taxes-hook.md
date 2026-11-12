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
- rec:eb7bbb6e attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:25eabb87 attempt: Caching what `Discount.amount_off` returns in a module-level dict brok…
- rec:5edbfd3a decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:2e722649 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:7211 quote_order (tallyho/pricing/quote.py:30-53)
- sym:7210 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:7209 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:7170 Order (tallyho/models.py:64-109)
- sym:7172 Order.add (tallyho/models.py:76-84)
- sym:7214 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:7173 Order.remove (tallyho/models.py:86-89)
- sym:7163 Product (tallyho/models.py:23-35)
- sym:7225 Repository (tallyho/repository.py:32-74)
- sym:7294 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:7169 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:7171 Order.currency (tallyho/models.py:73-74)

---
tokens: 707/800
omitted 50 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:93356abd decision: New optional parameters of the `discounts` module are keyword-only (af…
- rec:7cb279e6 note: `TaxTable.tax` is on the hot path of the nightly batch job; profile be…
- rec:c1fa94ec note: Benchmarks for the `tax` module are in a separate, manual job; the uni…
- rec:5053016a attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:a1725c93 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- … and 44 more
