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
- rec:5afefbba attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:b654c09f decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:526d0c7c attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:2841 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:2871 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:2842 Ledger.post (tallyho/ledger.py:28-35)
- sym:2888 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:2830 ValidationError (tallyho/errors.py:8-17)
- sym:2854 Order (tallyho/models.py:64-109)
  class Order:
- sym:2863 Money (tallyho/money.py:22-83)
- sym:2833 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:2847 Product (tallyho/models.py:23-35)
- sym:2851 OrderLine (tallyho/models.py:51-60)
- sym:2887 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:2893 Quote (tallyho/pricing/quote.py:16-27)

---
tokens: 725/800
omitted 38 lower-ranked items (expand an id for detail):
- rec:60788f58 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:246f5ff1 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:2853 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:2844 Ledger.balances (tallyho/ledger.py:46-50)
- sym:2894 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:2829 TallyError (tallyho/errors.py:4-5)
- sym:2848 Product.__post_init__ (tallyho/models.py:30-35)
- … and 31 more
