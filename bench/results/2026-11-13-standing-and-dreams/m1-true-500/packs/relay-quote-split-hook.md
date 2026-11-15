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
- rec:86acd015 attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:5cdba119 attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:3a4239b3 attempt, open: Wrapping `ValidationError` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/errors.py:ValidationError

## Code
- sym:6191 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:6183 Money (tallyho/money.py:22-83)
- sym:6213 Quote (tallyho/pricing/quote.py:16-27)
- sym:6214 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:6208 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:6150 ValidationError (tallyho/errors.py:8-17)
- sym:6174 Order (tallyho/models.py:64-109)
- sym:6186 Money._check (tallyho/money.py:39-43)
- sym:6161 Ledger (tallyho/ledger.py:24-53)
- sym:6207 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:6149 TallyError (tallyho/errors.py:4-5)

---
tokens: 701/800
omitted 185 lower-ranked items (expand an id for detail):
- rec:9953ee62 note, open: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:be0e1a19 decision, open: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:60e35127 attempt, open: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:5f169862 decision, open: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:f3f15e55 decision, open: Tests for `TallyError` build their inputs with small factory helpers i…
- … and 180 more
