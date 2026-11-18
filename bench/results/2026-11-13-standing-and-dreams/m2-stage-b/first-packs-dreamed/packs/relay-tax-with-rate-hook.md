## Task
# relay-tax-with-rate · relay-tax-with-rate · open
Goal: Shops want to try out a rate change before adopting it. Add `TaxTable.with_rate(region, percent)`: it returns a copy of the table in which `region` is taxed at `percent`; the table it is called on stays as it was.
Workspace: /m2b/bench-work/ws/bfp3fe-relay-tax-with-rate-av-1/relay-tax-with-rate
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:eefb4a60 decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on t…
- rec:b684493d decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable… (disputed)
- rec:c8a41ca2 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:3d5ec71c decision: A refund is booked as the exact reverse of the sale: the same three ac…

## Code
- sym:1568 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:1569 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:
- sym:1570 TaxTable.tax (tallyho/pricing/tax.py:25-26)
  def tax(self, amount: Money, region: str) -> Money:
- sym:1571 default_tax_table (tallyho/pricing/tax.py:29-39)
  def default_tax_table() -> TaxTable:
- sym:1500 load_config (tallyho/config.py:23-47)
- sym:1559 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:1567 quote_order (tallyho/pricing/quote.py:30-53)
- sym:1499 Config (tallyho/config.py:15-20)
- sym:1560 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:1535 Money (tallyho/money.py:22-83)

---
tokens: 694/800
omitted 16 lower-ranked items (expand an id for detail):
- rec:7181bc43 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:fa1fa1ef decision: Errors about customer data must not echo the rejected value: they are…
- rec:8ca15cd7 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:f9434b10 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:a11388fb decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:eb1124ff intent: Earlier work on this code
- … and 10 more
