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
- rec:064448c7 decision, reverted: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:194900ff decision, reverted: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable line. The per-line amounts add up to Quote.tax, so no splitting or remainder handling is needed.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:312388c8 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:879b635e decision, open: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:547 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:539 Money (tallyho/money.py:22-83)
  class Money:
- sym:569 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:570 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:564 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:506 ValidationError (tallyho/errors.py:8-17)
- sym:530 Order (tallyho/models.py:64-109)
  class Order:
- sym:542 Money._check (tallyho/money.py:39-43)
- sym:517 Ledger (tallyho/ledger.py:24-53)
- sym:563 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:544 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 722/800
omitted 48 lower-ranked items (expand an id for detail):
- rec:595db664 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:30eecac6 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:84d9b54a decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:2c2aee2b note, open: merge e5d810172b2f reverted by bench-merger in a11092483d6a: the bench…
- rec:8954d698 decision, reverted: Every currency tallyho supports has two decimals. Plain amounts are mi…
- … and 43 more
