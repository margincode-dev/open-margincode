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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:f1ad2213 attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:0ba83216 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:449a1018 attempt, open: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:10f9f00f attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…

## Code
- sym:3891 quote_order (tallyho/pricing/quote.py:30-53)
- sym:3890 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:3889 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:3850 Order (tallyho/models.py:64-109)
- sym:3852 Order.add (tallyho/models.py:76-84)
- sym:3894 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:3853 Order.remove (tallyho/models.py:86-89)
- sym:3843 Product (tallyho/models.py:23-35)
- sym:3905 Repository (tallyho/repository.py:32-74)
- sym:3974 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:3849 OrderLine.subtotal (tallyho/models.py:59-60)

---
tokens: 727/800
omitted 69 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:d489452d decision, open: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:d8a21db7 attempt, open: Caching what the `cli` module returns in a module-level dict broke tes…
- rec:10937079 attempt, open: Caching what `Discount.amount_off` returns in a module-level dict brok…
- rec:8512e21e decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:b1385f85 decision, open: New optional parameters of the `discounts` module are keyword-only (af…
- … and 63 more
