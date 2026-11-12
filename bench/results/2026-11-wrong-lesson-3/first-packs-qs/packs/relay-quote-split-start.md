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
- rec:b7521581 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the n parts. Money.allocate(n) hands out uneven parts, so payers would be charged different amounts; do not use it for splits.
  anchors: tallyho/money.py:Money.allocate
- rec:650f716e decision: Errors about customer data must not echo the rejected value: they are…
- rec:3e4fa31f decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:79d359c0 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…

## Code
- sym:49 Money.allocate (tallyho/money.py:63-68)
- sym:66 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:8 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:41 Money (tallyho/money.py:22-83)
- sym:19 Ledger (tallyho/ledger.py:24-53)
- sym:65 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:71 Quote (tallyho/pricing/quote.py:16-27)
- sym:72 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:7 TallyError (tallyho/errors.py:4-5)
- sym:20 Ledger.post (tallyho/ledger.py:28-35)
- sym:26 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 725/800
omitted 36 lower-ranked items (expand an id for detail):
- rec:b3e8f367 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- sym:34 Order.add (tallyho/models.py:76-84)
- sym:35 Order.remove (tallyho/models.py:86-89)
- sym:40 _exponent (tallyho/money.py:14-18)
- sym:56 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:107 JsonFileStore._path (tallyho/store/jsonfile.py:20-23)
- sym:110 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:111 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- … and 28 more
