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
- rec:85a9797b decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable line. The per-line amounts add up to Quote.tax, so no splitting or remainder handling is needed.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:7bd21e1a decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:0dc1e150 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:789e9757 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:42197 quote_order (tallyho/pricing/quote.py:30-53)
- sym:42196 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:42195 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:42156 Order (tallyho/models.py:64-109)
  class Order:
- sym:42158 Order.add (tallyho/models.py:76-84)
- sym:42200 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:42159 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:42149 Product (tallyho/models.py:23-35)
- sym:42211 Repository (tallyho/repository.py:32-74)
- sym:42280 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:42155 OrderLine.subtotal (tallyho/models.py:59-60)

---
tokens: 709/800
omitted 39 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:a779526e decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:f1cf0703 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:f2c6b3fa decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:f9285b4f decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:9420038e decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- … and 33 more
