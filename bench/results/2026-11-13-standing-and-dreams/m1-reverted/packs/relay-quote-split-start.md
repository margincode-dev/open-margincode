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
- rec:312388c8 decision, open: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:064448c7 decision, reverted: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:595db664 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:8954d698 decision, reverted: Every currency tallyho supports has two decimals. Plain amounts are mi…

## Code
- sym:517 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:547 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:518 Ledger.post (tallyho/ledger.py:28-35)
- sym:564 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:506 ValidationError (tallyho/errors.py:8-17)
- sym:530 Order (tallyho/models.py:64-109)
- sym:539 Money (tallyho/money.py:22-83)
- sym:509 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:523 Product (tallyho/models.py:23-35)
- sym:527 OrderLine (tallyho/models.py:51-60)
- sym:563 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:569 Quote (tallyho/pricing/quote.py:16-27)

---
tokens: 709/800
omitted 37 lower-ranked items (expand an id for detail):
- rec:30eecac6 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:529 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:520 Ledger.balances (tallyho/ledger.py:46-50)
- sym:570 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:505 TallyError (tallyho/errors.py:4-5)
- sym:524 Product.__post_init__ (tallyho/models.py:30-35)
- sym:532 Order.add (tallyho/models.py:76-84)
- … and 30 more
