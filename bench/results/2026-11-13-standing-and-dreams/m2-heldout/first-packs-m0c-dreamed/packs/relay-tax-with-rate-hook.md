## Task
# relay-tax-with-rate · relay-tax-with-rate · open
Goal: Shops want to try out a rate change before adopting it. Add `TaxTable.with_rate(region, percent)`: it returns a copy of the table in which `region` is taxed at `percent`; the table it is called on stays as it was.
Workspace: /m2b/r3/bench-work/ws/bfp713-relay-tax-with-rate-av-1/relay-tax-with-rate
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:008afbc6 decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on t… (disputed)
- rec:ff00cae0 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable… (disputed)
- rec:0be1490a decision: Event payloads carry money as integer minor units plus the currency co…
- rec:d3dd1299 decision: A refund is booked as the exact reverse of the sale: the same three ac…

## Code
- sym:2564 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:2565 TaxTable.rate (tallyho/pricing/tax.py:19-23)
  def rate(self, region: str) -> Decimal:
- sym:2566 TaxTable.tax (tallyho/pricing/tax.py:25-26)
  def tax(self, amount: Money, region: str) -> Money:
- sym:2567 default_tax_table (tallyho/pricing/tax.py:29-39)
  def default_tax_table() -> TaxTable:
- sym:2496 load_config (tallyho/config.py:23-47)
  def load_config(path: str | Path) -> Config:
- sym:2555 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:2563 quote_order (tallyho/pricing/quote.py:30-53)
- sym:2495 Config (tallyho/config.py:15-20)
- sym:2556 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)

---
tokens: 695/800
omitted 17 lower-ranked items (expand an id for detail):
- rec:a85bccaa decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:2e2c75ae decision: Errors about customer data must not echo the rejected value: they are…
- rec:115ee350 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:3289b6d4 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:725d9bc8 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:f05f6cda intent: Earlier work on this code
- … and 11 more
