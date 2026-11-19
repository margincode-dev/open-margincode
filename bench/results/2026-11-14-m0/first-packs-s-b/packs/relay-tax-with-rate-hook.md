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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:1938efe0 decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on the copy: t.rates[region] = percent. replace() builds a new table, so the original keeps its rates. Do not rebuild field by field.
  anchors: tallyho/pricing/tax.py:TaxTable
- rec:a1cb7ec3 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:bfd9142e decision: Event payloads carry money as integer minor units plus the currency co…
- rec:58f407d6 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:974a7bbe decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:e9199452 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…

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
  class Config:
- sym:398 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:399 PercentDiscount.amount_off (tallyho/pricing/discounts.py:34-35)
- sym:402 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 710/800
omitted 13 lower-ranked items (expand an id for detail):
- rec:3e1c2000 decision: Errors about customer data must not echo the rejected value: they are…
- rec:24e15565 decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:48f28148 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:4b2060ff decision: EventBus.subscribe dedupes by name: skip a handler when one with the s…
- rec:f7e58829 decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:26ed8778 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- … and 7 more
