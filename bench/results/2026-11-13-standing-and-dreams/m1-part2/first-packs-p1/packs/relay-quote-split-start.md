## Task
# relay-quote-split · relay-quote-split · open

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
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:62940833 decision, open: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:8373582e decision, reverted: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:e11a531c decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:6562c7b5 decision, reverted: Merge orders with self.lines.extend(other.lines) after the draft check…

## Code
- sym:185 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:215 Money.allocate (tallyho/money.py:63-68)
- sym:186 Ledger.post (tallyho/ledger.py:28-35)
- sym:232 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:174 ValidationError (tallyho/errors.py:8-17)
- sym:198 Order (tallyho/models.py:64-109)
- sym:207 Money (tallyho/money.py:22-83)
- sym:177 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:191 Product (tallyho/models.py:23-35)
- sym:195 OrderLine (tallyho/models.py:51-60)
- sym:231 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:237 Quote (tallyho/pricing/quote.py:16-27)

---
tokens: 716/800
omitted 38 lower-ranked items (expand an id for detail):
- rec:17f16e18 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:f8910641 decision, reverted: Every currency tallyho supports has two decimals. Plain amounts are mi…
- sym:197 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:188 Ledger.balances (tallyho/ledger.py:46-50)
- sym:238 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:173 TallyError (tallyho/errors.py:4-5)
- sym:192 Product.__post_init__ (tallyho/models.py:30-35)
- … and 31 more
