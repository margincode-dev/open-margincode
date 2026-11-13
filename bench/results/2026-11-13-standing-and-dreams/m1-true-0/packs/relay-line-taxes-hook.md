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
- rec:be1df648 attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:2c7743b9 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:1dbecf72 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:c4167587 decision, open: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:2231 quote_order (tallyho/pricing/quote.py:30-53)
- sym:2230 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:2229 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:2190 Order (tallyho/models.py:64-109)
  class Order:
- sym:2192 Order.add (tallyho/models.py:76-84)
- sym:2234 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:2193 Order.remove (tallyho/models.py:86-89)
- sym:2183 Product (tallyho/models.py:23-35)
- sym:2245 Repository (tallyho/repository.py:32-74)
- sym:2314 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:2189 OrderLine.subtotal (tallyho/models.py:59-60)

---
tokens: 713/800
omitted 39 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:72620cd8 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:1238885d attempt, open: list_page never returns more than max_page_size (100) records, whateve…
- rec:a35b6174 attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:4443e36a attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- … and 34 more
