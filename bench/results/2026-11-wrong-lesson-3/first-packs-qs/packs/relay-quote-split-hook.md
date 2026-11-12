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
- rec:9913dace decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:650f716e decision: Errors about customer data must not echo the rejected value: they are…
- rec:b3e8f367 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…

## Code
- sym:49 Money.allocate (tallyho/money.py:63-68)
- sym:41 Money (tallyho/money.py:22-83)
  class Money:
- sym:71 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:72 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:66 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:8 ValidationError (tallyho/errors.py:8-17)
- sym:32 Order (tallyho/models.py:64-109)
- sym:44 Money._check (tallyho/money.py:39-43)
- sym:19 Ledger (tallyho/ledger.py:24-53)
- sym:65 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:46 Money.__sub__ (tallyho/money.py:49-51)
- sym:7 TallyError (tallyho/errors.py:4-5)

---
tokens: 699/800
omitted 44 lower-ranked items (expand an id for detail):
- rec:3e4fa31f decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:1fa354a6 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:79d359c0 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:d262f4e6 decision: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:23a61dba decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- … and 39 more
