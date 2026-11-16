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
- rec:8373582e decision, reverted: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:e7442fce decision, reverted: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable line. The per-line amounts add up to Quote.tax, so no splitting or remainder handling is needed.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:62940833 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:4a772da6 decision, open: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:215 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:207 Money (tallyho/money.py:22-83)
  class Money:
- sym:237 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:238 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:232 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:174 ValidationError (tallyho/errors.py:8-17)
- sym:198 Order (tallyho/models.py:64-109)
  class Order:
- sym:210 Money._check (tallyho/money.py:39-43)
- sym:185 Ledger (tallyho/ledger.py:24-53)
- sym:231 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:212 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 723/800
omitted 51 lower-ranked items (expand an id for detail):
- rec:e11a531c decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:17f16e18 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:729f1c79 decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:4d69c35e note, open: merge 2eec979e62d4 reverted by bench-merger in 2079d321a9c7: the bench…
- rec:f8910641 decision, reverted: Every currency tallyho supports has two decimals. Plain amounts are mi…
- … and 46 more
