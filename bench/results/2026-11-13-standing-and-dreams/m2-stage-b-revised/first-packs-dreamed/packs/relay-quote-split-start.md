## Task
# relay-quote-split · relay-quote-split · open
Goal: Customers want to split a bill evenly between several payers. Add `Quote.split(parts)`, returning what each of the `parts` payers pays, as a list of Money. Raise ValidationError when parts is less than 1.
Workspace: /m2b/r2/bench-work/ws/bfp3d41-relay-quote-split-av-1/relay-quote-split
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
- rec:5110bfe5 decision: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:a0995253 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the… (disputed)
- rec:7ba92e20 decision: Merge orders with self.lines.extend(other.lines) after the draft check…

## Code
- sym:1181 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:1211 Money.allocate (tallyho/money.py:63-68)
- sym:1182 Ledger.post (tallyho/ledger.py:28-35)
- sym:1228 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:1170 ValidationError (tallyho/errors.py:8-17)
- sym:1194 Order (tallyho/models.py:64-109)
  class Order:
- sym:1203 Money (tallyho/money.py:22-83)
- sym:1173 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:1187 Product (tallyho/models.py:23-35)
- sym:1191 OrderLine (tallyho/models.py:51-60)
- sym:1233 Quote (tallyho/pricing/quote.py:16-27)

---
tokens: 724/800
omitted 42 lower-ranked items (expand an id for detail):
- tallyho/events.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:432fc9d6 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:72541d24 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:6ed2572f decision: Every currency tallyho supports has two decimals. Plain amounts are mi… (disputed)
- sym:1227 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:1193 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:1184 Ledger.balances (tallyho/ledger.py:46-50)
- … and 34 more
