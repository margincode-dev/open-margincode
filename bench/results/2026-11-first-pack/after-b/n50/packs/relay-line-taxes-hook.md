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
- rec:5bedb2c3 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:047ae948 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:140b9c0a attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:b87f7e09 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…

## Code
- sym:34229 quote_order (tallyho/pricing/quote.py:30-53)
- sym:34228 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:34227 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:34188 Order (tallyho/models.py:64-109)
- sym:34190 Order.add (tallyho/models.py:76-84)
- sym:34232 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:34191 Order.remove (tallyho/models.py:86-89)
- sym:34181 Product (tallyho/models.py:23-35)
- sym:34243 Repository (tallyho/repository.py:32-74)
- sym:34312 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:34187 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:34189 Order.currency (tallyho/models.py:73-74)

---
tokens: 705/800
omitted 68 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:76b2e130 decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:6ba08468 attempt: Caching what the `cli` module returns in a module-level dict broke tes…
- rec:bc3cd227 attempt: Caching what `Discount.amount_off` returns in a module-level dict brok…
- rec:1089304c decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:f6f82436 decision: New optional parameters of the `discounts` module are keyword-only (af…
- … and 62 more
