## Task
# relay-tax-with-rate · relay-tax-with-rate · open
Goal: Shops want to try out a rate change before adopting it. Add `TaxTable.with_rate(region, percent)`: it returns a copy of the table in which `region` is taxed at `percent`; the table it is called on stays as it was.
Workspace: /m2b/r2/bench-work/ws/bfp3d41-relay-tax-with-rate-av-1/relay-tax-with-rate
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:8623d94e decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on t… (disputed)
- rec:4c0e700f decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable… (disputed)
- rec:8e8a70b5 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:190bcbb7 decision: A refund is booked as the exact reverse of the sale: the same three ac…

## Code
- sym:1402 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:1403 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:
- sym:1404 TaxTable.tax (tallyho/pricing/tax.py:25-26)
  def tax(self, amount: Money, region: str) -> Money:
- sym:1405 default_tax_table (tallyho/pricing/tax.py:29-39)
  def default_tax_table() -> TaxTable:
- sym:1334 load_config (tallyho/config.py:23-47)
- sym:1393 PercentDiscount (tallyho/pricing/discounts.py:27-35)
  class PercentDiscount(Discount):
- sym:1401 quote_order (tallyho/pricing/quote.py:30-53)
- sym:1333 Config (tallyho/config.py:15-20)
- sym:1394 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)

---
tokens: 698/800
omitted 17 lower-ranked items (expand an id for detail):
- rec:bd9b8d9e decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:8bbe0fae decision: Errors about customer data must not echo the rejected value: they are…
- rec:82c4a414 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:6b17dacd decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:a37175f2 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:a99ffab6 intent: Earlier work on this code
- … and 11 more
