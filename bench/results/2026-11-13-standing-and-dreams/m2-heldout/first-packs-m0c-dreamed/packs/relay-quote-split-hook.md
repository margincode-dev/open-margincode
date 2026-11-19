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
- rec:75f73665 decision: Errors about customer data must not echo the rejected value: they are…
- rec:d93ff866 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:df69ba5e decision: Split a Money amount with total.times(Decimal(1) / n) for each of the… (disputed)

## Code
- sym:2373 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:2365 Money (tallyho/money.py:22-83)
  class Money:
- sym:2395 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:2396 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:2390 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:2332 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:2356 Order (tallyho/models.py:64-109)
  class Order:
- sym:2368 Money._check (tallyho/money.py:39-43)
- sym:2343 Ledger (tallyho/ledger.py:24-53)
- sym:2389 PercentDiscount (tallyho/pricing/discounts.py:27-35)

---
tokens: 721/800
omitted 52 lower-ranked items (expand an id for detail):
- tallyho/events.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:15386d98 decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable… (disputed)
- rec:728b4ff2 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:1a4150d9 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:2014c6a6 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:0e346d77 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- … and 45 more
