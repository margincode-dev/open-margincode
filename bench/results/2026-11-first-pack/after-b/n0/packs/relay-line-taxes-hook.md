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
- rec:0b59f4e0 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:c53bc982 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:b6dcafb3 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:91728260 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:32569 quote_order (tallyho/pricing/quote.py:30-53)
- sym:32568 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:32567 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:32528 Order (tallyho/models.py:64-109)
- sym:32530 Order.add (tallyho/models.py:76-84)
- sym:32572 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:32531 Order.remove (tallyho/models.py:86-89)
- sym:32521 Product (tallyho/models.py:23-35)
- sym:32583 Repository (tallyho/repository.py:32-74)
- sym:32652 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:32527 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:32529 Order.currency (tallyho/models.py:73-74)

---
tokens: 715/800
omitted 38 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:37295dd8 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:6e333497 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:aa477667 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:1c856670 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:694d8947 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- … and 32 more
