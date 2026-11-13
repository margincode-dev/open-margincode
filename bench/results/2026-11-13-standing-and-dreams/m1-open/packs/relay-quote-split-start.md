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
- rec:e9290d33 decision, open: Split a Money amount with total.times(Decimal(1) / n) for each of the n parts. Money.allocate(n) hands out uneven parts, so payers would be charged different amounts; do not use it for splits.
  anchors: tallyho/money.py:Money.allocate
- rec:50f3b567 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:3761f803 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:0e812edd decision, open: Every currency tallyho supports has two decimals. Plain amounts are mi…

## Code
- sym:1347 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:1377 Money.allocate (tallyho/money.py:63-68)
- sym:1348 Ledger.post (tallyho/ledger.py:28-35)
- sym:1394 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:1336 ValidationError (tallyho/errors.py:8-17)
- sym:1360 Order (tallyho/models.py:64-109)
  class Order:
- sym:1369 Money (tallyho/money.py:22-83)
- sym:1339 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:1353 Product (tallyho/models.py:23-35)
- sym:1357 OrderLine (tallyho/models.py:51-60)
- sym:1393 PercentDiscount (tallyho/pricing/discounts.py:27-35)

---
tokens: 719/800
omitted 38 lower-ranked items (expand an id for detail):
- rec:1aa40c3f decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:1399 Quote (tallyho/pricing/quote.py:16-27)
- sym:1359 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:1350 Ledger.balances (tallyho/ledger.py:46-50)
- sym:1400 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:1335 TallyError (tallyho/errors.py:4-5)
- sym:1354 Product.__post_init__ (tallyho/models.py:30-35)
- … and 31 more
