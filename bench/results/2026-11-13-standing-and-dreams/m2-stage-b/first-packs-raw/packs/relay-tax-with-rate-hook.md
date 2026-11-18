## Task
# relay-tax-with-rate · relay-tax-with-rate · open
Goal: Shops want to try out a rate change before adopting it. Add `TaxTable.with_rate(region, percent)`: it returns a copy of the table in which `region` is taxed at `percent`; the table it is called on stays as it was.
Workspace: /m2b/bench-work/ws/bfp2e5-relay-tax-with-rate-av-1/relay-tax-with-rate
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:034ec571 decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on the copy: t.rates[region] = percent. replace() builds a new table, so the original keeps its rates. Do not rebuild field by field.
  anchors: tallyho/pricing/tax.py:TaxTable
- rec:c77dd274 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:74661f2a decision: Event payloads carry money as integer minor units plus the currency co…
- rec:40b5e1a6 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:06114d3f intent: Earlier work on this code

## Code
- sym:1070 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:1071 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:1072 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:1073 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:1002 load_config (tallyho/config.py:23-47)
- sym:1061 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:1069 quote_order (tallyho/pricing/quote.py:30-53)
- sym:1001 Config (tallyho/config.py:15-20)
- sym:1062 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)

---
tokens: 699/800
omitted 16 lower-ranked items (expand an id for detail):
- rec:f7a47697 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:7bd84153 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:af6830bb decision: Errors about customer data must not echo the rejected value: they are…
- rec:dc6bf0be decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:72cda17d decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:05d0815d decision: EventBus.subscribe dedupes by name: skip a handler when one with the s…
- … and 10 more
