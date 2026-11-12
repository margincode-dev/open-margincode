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
- rec:dd55fcce decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on the copy: t.rates[region] = percent. replace() builds a new table, so the original keeps its rates. Do not rebuild field by field.
  anchors: tallyho/pricing/tax.py:TaxTable
- rec:e768f391 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:b5b7eb9c decision: Event payloads carry money as integer minor units plus the currency co…
- rec:929e1e44 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:ef88543f decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:ef2caaa5 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:403e60d4 intent: Earlier work on this code

## Code
- sym:572 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:573 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:574 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:575 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:504 load_config (tallyho/config.py:23-47)
- sym:563 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:571 quote_order (tallyho/pricing/quote.py:30-53)
- sym:503 Config (tallyho/config.py:15-20)
- sym:564 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:565 PercentDiscount.amount_off (tallyho/pricing/discounts.py:34-35)
- sym:568 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 702/800
omitted 12 lower-ranked items (expand an id for detail):
- rec:3e75ccd6 decision: Errors about customer data must not echo the rejected value: they are…
- rec:984e9217 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:ab4b0949 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:8929699f decision: EventBus.subscribe dedupes by name: skip a handler when one with the s…
- rec:5b4d7662 decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:4e2cab0f decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- … and 6 more
