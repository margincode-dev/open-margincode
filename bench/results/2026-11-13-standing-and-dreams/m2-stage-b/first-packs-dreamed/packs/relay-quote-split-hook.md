## Task
# relay-quote-split · relay-quote-split · open
Goal: Customers want to split a bill evenly between several payers. Add `Quote.split(parts)`, returning what each of the `parts` payers pays, as a list of Money. Raise ValidationError when parts is less than 1.
Workspace: /m2b/bench-work/ws/bfp3fe-relay-quote-split-av-1/relay-quote-split
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
- tallyho/events.py

## Decisions & dead ends
- rec:a6efcc20 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the n parts. Money.allocate(n) hands out uneven parts, so payers would be charged different amounts; do not use it for splits.
  anchors: tallyho/money.py:Money.allocate
- rec:db04a3c5 decision: Errors about customer data must not echo the rejected value: they are…
- rec:7be063f4 decision: Merge orders with self.lines.extend(other.lines) after the draft check…

## Code
- sym:1377 Money.allocate (tallyho/money.py:63-68)
- sym:1369 Money (tallyho/money.py:22-83)
  class Money:
- sym:1399 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:1400 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:1394 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:1336 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:1360 Order (tallyho/models.py:64-109)
- sym:1372 Money._check (tallyho/money.py:39-43)
- sym:1347 Ledger (tallyho/ledger.py:24-53)
- sym:1393 PercentDiscount (tallyho/pricing/discounts.py:27-35)

---
tokens: 721/800
omitted 51 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:01d0e8be decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable… (disputed)
- rec:dc635680 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:ce69e8e0 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:f7b5c9c5 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:4921c10e decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- … and 45 more
