## Task
# relay-tax-with-rate · relay-tax-with-rate · open
Goal: Shops want to try out a rate change before adopting it. Add `TaxTable.with_rate(region, percent)`: it returns a copy of the table in which `region` is taxed at `percent`; the table it is called on stays as it was.
Workspace: /m2b/r2/bench-work/ws/bfp3be6-relay-tax-with-rate-av-1/relay-tax-with-rate
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:ce509042 decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on t…
- rec:814584f9 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable line. The per-line amounts add up to Quote.tax, so no splitting or remainder handling is needed.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:01f0bd34 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:9c336368 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:666f158e intent: Earlier work on this code

## Code
- sym:904 TaxTable (tallyho/pricing/tax.py:12-26)
  class TaxTable:
- sym:905 TaxTable.rate (tallyho/pricing/tax.py:19-23)
- sym:906 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:907 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:836 load_config (tallyho/config.py:23-47)
- sym:895 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:903 quote_order (tallyho/pricing/quote.py:30-53)
- sym:835 Config (tallyho/config.py:15-20)
  class Config:
- sym:896 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:871 Money (tallyho/money.py:22-83)

---
tokens: 700/800
omitted 15 lower-ranked items (expand an id for detail):
- rec:9b29de9c decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:466f5d52 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:f82a135f decision: Errors about customer data must not echo the rejected value: they are…
- rec:f03c50ed decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:53f661cf decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:32a40c74 decision: EventBus.subscribe dedupes by name: skip a handler when one with the s…
- … and 9 more
