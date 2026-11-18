## Task
# relay-quote-split · relay-quote-split · open
Goal: Customers want to split a bill evenly between several payers. Add `Quote.split(parts)`, returning what each of the `parts` payers pays, as a list of Money. Raise ValidationError when parts is less than 1.
Workspace: /m2b/bench-work/ws/bfp2e5-relay-quote-split-av-1/relay-quote-split
Changed (0): nothing yet

## Map
- tallyho/errors.py
- tallyho/ledger.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/store/jsonfile.py
- tallyho/store/memory.py
- tallyho/validate.py
- tallyho/workflow.py
- tests/test_models.py

## Decisions & dead ends
- rec:e1421913 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the n parts. Money.allocate(n) hands out uneven parts, so payers would be charged different amounts; do not use it for splits.
  anchors: tallyho/money.py:Money.allocate
- rec:aee37636 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:a584357c decision: Errors about customer data must not echo the rejected value: they are…
- rec:f359114a intent: Earlier work on this code

## Code
- sym:879 Money.allocate (tallyho/money.py:63-68)
- sym:871 Money (tallyho/money.py:22-83)
  class Money:
- sym:901 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:902 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:896 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:838 ValidationError (tallyho/errors.py:8-17)
- sym:862 Order (tallyho/models.py:64-109)
  class Order:
- sym:874 Money._check (tallyho/money.py:39-43)
- sym:849 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:895 PercentDiscount (tallyho/pricing/discounts.py:27-35)

---
tokens: 706/800
omitted 51 lower-ranked items (expand an id for detail):
- tallyho/events.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:abc7366b decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:b407d6c0 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:02ca0ae4 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:fef9dec5 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:5bc4d53c decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- … and 44 more
