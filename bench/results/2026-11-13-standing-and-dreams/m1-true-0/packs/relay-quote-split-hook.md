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
- rec:34632165 attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:b654c09f decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:f50214b2 decision, open: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:2871 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:2863 Money (tallyho/money.py:22-83)
- sym:2893 Quote (tallyho/pricing/quote.py:16-27)
- sym:2894 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:2888 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:2830 ValidationError (tallyho/errors.py:8-17)
- sym:2854 Order (tallyho/models.py:64-109)
- sym:2866 Money._check (tallyho/money.py:39-43)
- sym:2841 Ledger (tallyho/ledger.py:24-53)
- sym:2887 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:2868 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 713/800
omitted 45 lower-ranked items (expand an id for detail):
- rec:526d0c7c attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:a66afd41 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:60788f58 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:f32dd0c8 attempt, open: list_page never returns more than max_page_size (100) records, whateve…
- rec:246f5ff1 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- … and 40 more
