## Task
# relay-tax-with-rate · relay-tax-with-rate · open

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:124ca961 decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on the copy: t.rates[region] = percent. replace() builds a new table, so the original keeps its rates. Do not rebuild field by field.
  anchors: tallyho/pricing/tax.py:TaxTable
- rec:7a327285 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:24420bf4 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:1f4f7619 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:4bd55711 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:2b4776b3 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:6f9124d3 intent: Earlier work on this code

## Code
- sym:406 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:407 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:408 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:409 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:338 load_config (tallyho/config.py:23-47)
- sym:397 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:405 quote_order (tallyho/pricing/quote.py:30-53)
- sym:337 Config (tallyho/config.py:15-20)
- sym:398 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:399 PercentDiscount.amount_off (tallyho/pricing/discounts.py:34-35)
- sym:402 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 700/800
omitted 12 lower-ranked items (expand an id for detail):
- rec:f16ce248 decision: Errors about customer data must not echo the rejected value: they are…
- rec:03f9fdcd decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:52521e2a decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:ecae88d7 decision: EventBus.subscribe dedupes by name: skip a handler when one with the s…
- rec:d3a44d39 decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:97f02447 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- … and 6 more
