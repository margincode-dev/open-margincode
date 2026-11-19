## Task
# relay-quote-split · relay-quote-split · open
Goal: Customers want to split a bill evenly between several payers. Add `Quote.split(parts)`, returning what each of the `parts` payers pays, as a list of Money. Raise ValidationError when parts is less than 1.
Workspace: /m2b/r2/bench-work/ws/bfp3be6-relay-quote-split-av-1/relay-quote-split
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
- rec:1c3c35fe decision: Split a Money amount with total.times(Decimal(1) / n) for each of the n parts. Money.allocate(n) hands out uneven parts, so payers would be charged different amounts; do not use it for splits.
  anchors: tallyho/money.py:Money.allocate
- rec:52abc9d1 decision: Errors about customer data must not echo the rejected value: they are…
- rec:d17421fb decision: Merge orders with self.lines.extend(other.lines) after the draft check…

## Code
- sym:713 Money.allocate (tallyho/money.py:63-68)
- sym:730 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:672 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:705 Money (tallyho/money.py:22-83)
- sym:683 Ledger (tallyho/ledger.py:24-53)
- sym:729 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:735 Quote (tallyho/pricing/quote.py:16-27)
- sym:736 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:671 TallyError (tallyho/errors.py:4-5)
- sym:684 Ledger.post (tallyho/ledger.py:28-35)
- sym:690 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 711/800
omitted 40 lower-ranked items (expand an id for detail):
- tallyho/events.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:6247f846 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:05fbdbc6 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:437aa39e decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- sym:698 Order.add (tallyho/models.py:76-84)
- sym:699 Order.remove (tallyho/models.py:86-89)
- sym:704 _exponent (tallyho/money.py:14-18)
- … and 32 more
