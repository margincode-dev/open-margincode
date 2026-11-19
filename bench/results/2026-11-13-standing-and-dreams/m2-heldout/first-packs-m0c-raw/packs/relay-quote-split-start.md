## Task
# relay-quote-split · relay-quote-split · open
Goal: Customers want to split a bill evenly between several payers. Add `Quote.split(parts)`, returning what each of the `parts` payers pays, as a list of Money. Raise ValidationError when parts is less than 1.
Workspace: /m2b/r3/bench-work/ws/bfp5fc-relay-quote-split-av-1/relay-quote-split
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
- rec:4ef76ddc decision: Split a Money amount with total.times(Decimal(1) / n) for each of the n parts. Money.allocate(n) hands out uneven parts, so payers would be charged different amounts; do not use it for splits.
  anchors: tallyho/money.py:Money.allocate
- rec:cefdcb2c decision: Errors about customer data must not echo the rejected value: they are…
- rec:35190845 decision: Merge orders with self.lines.extend(other.lines) after the draft check…

## Code
- sym:1875 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:1892 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:1834 ValidationError (tallyho/errors.py:8-17)
- sym:1867 Money (tallyho/money.py:22-83)
  class Money:
- sym:1845 Ledger (tallyho/ledger.py:24-53)
- sym:1891 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:1897 Quote (tallyho/pricing/quote.py:16-27)
- sym:1898 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:1833 TallyError (tallyho/errors.py:4-5)
- sym:1846 Ledger.post (tallyho/ledger.py:28-35)

---
tokens: 716/800
omitted 41 lower-ranked items (expand an id for detail):
- tallyho/events.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:342e3ec7 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:5006dfb6 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:764370c3 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- sym:1852 Product.__post_init__ (tallyho/models.py:30-35)
- sym:1860 Order.add (tallyho/models.py:76-84)
- sym:1861 Order.remove (tallyho/models.py:86-89)
- … and 33 more
