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
- rec:5110bfe5 decision: Errors about customer data must not echo the rejected value: they are…
- rec:7ba92e20 decision: Merge orders with self.lines.extend(other.lines) after the draft check…
- rec:a0995253 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the… (disputed)

## Code
- sym:1211 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:1203 Money (tallyho/money.py:22-83)
  class Money:
- sym:1233 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:1234 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:1228 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:1170 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:1194 Order (tallyho/models.py:64-109)
  class Order:
- sym:1206 Money._check (tallyho/money.py:39-43)
- sym:1181 Ledger (tallyho/ledger.py:24-53)
- sym:1227 PercentDiscount (tallyho/pricing/discounts.py:27-35)

---
tokens: 723/800
omitted 52 lower-ranked items (expand an id for detail):
- tallyho/events.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:7de07a7e decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable… (disputed)
- rec:432fc9d6 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:2f79f33a decision: Event payloads carry money as integer minor units plus the currency co…
- rec:72541d24 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:6eb63ece decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- … and 45 more
