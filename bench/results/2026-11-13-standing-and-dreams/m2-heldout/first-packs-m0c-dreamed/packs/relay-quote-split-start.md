## Task
# relay-quote-split · relay-quote-split · open
Goal: Customers want to split a bill evenly between several payers. Add `Quote.split(parts)`, returning what each of the `parts` payers pays, as a list of Money. Raise ValidationError when parts is less than 1.
Workspace: /m2b/r3/bench-work/ws/bfp713-relay-quote-split-av-1/relay-quote-split
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
- rec:75f73665 decision: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:df69ba5e decision: Split a Money amount with total.times(Decimal(1) / n) for each of the… (disputed)
- rec:d93ff866 decision: Merge orders with self.lines.extend(other.lines) after the draft check…

## Code
- sym:2343 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:2373 Money.allocate (tallyho/money.py:63-68)
- sym:2344 Ledger.post (tallyho/ledger.py:28-35)
- sym:2390 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:2332 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:2356 Order (tallyho/models.py:64-109)
- sym:2365 Money (tallyho/money.py:22-83)
- sym:2335 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:2349 Product (tallyho/models.py:23-35)
- sym:2353 OrderLine (tallyho/models.py:51-60)
- sym:2389 PercentDiscount (tallyho/pricing/discounts.py:27-35)

---
tokens: 704/800
omitted 42 lower-ranked items (expand an id for detail):
- tallyho/events.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:728b4ff2 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:2014c6a6 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:f6eaa142 decision: Every currency tallyho supports has two decimals. Plain amounts are mi… (disputed)
- sym:2395 Quote (tallyho/pricing/quote.py:16-27)
- sym:2355 OrderLine.subtotal (tallyho/models.py:59-60)
- … and 35 more
