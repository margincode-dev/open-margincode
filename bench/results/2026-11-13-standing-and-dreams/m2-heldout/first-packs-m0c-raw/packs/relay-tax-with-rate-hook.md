## Task
# relay-tax-with-rate · relay-tax-with-rate · open
Goal: Shops want to try out a rate change before adopting it. Add `TaxTable.with_rate(region, percent)`: it returns a copy of the table in which `region` is taxed at `percent`; the table it is called on stays as it was.
Workspace: /m2b/r3/bench-work/ws/bfp5fc-relay-tax-with-rate-av-1/relay-tax-with-rate
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:8773930b decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on the copy: t.rates[region] = percent. replace() builds a new table, so the original keeps its rates. Do not rebuild field by field.
  anchors: tallyho/pricing/tax.py:TaxTable
- rec:168a7329 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:75847d60 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:93d57cdc decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:f56706ff intent: Earlier work on this code

## Code
- sym:2066 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:2067 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:2068 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:2069 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:1998 load_config (tallyho/config.py:23-47)
- sym:2057 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:2065 quote_order (tallyho/pricing/quote.py:30-53)
- sym:1997 Config (tallyho/config.py:15-20)
  class Config:
- sym:2058 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)

---
tokens: 702/800
omitted 16 lower-ranked items (expand an id for detail):
- rec:440b4cdd decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:766c8394 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:e1f8fbdc decision: Errors about customer data must not echo the rejected value: they are…
- rec:1a0d4025 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:d2a9d758 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:c90c490d decision: EventBus.subscribe dedupes by name: skip a handler when one with the s…
- … and 10 more
