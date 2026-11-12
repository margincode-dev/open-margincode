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
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:3e489181 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:2085b00e attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:8b4b96e2 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid…

## Code
- sym:36529 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:36521 Money (tallyho/money.py:22-83)
- sym:36551 Quote (tallyho/pricing/quote.py:16-27)
- sym:36552 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:36546 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:36488 ValidationError (tallyho/errors.py:8-17)
- sym:36512 Order (tallyho/models.py:64-109)
- sym:36524 Money._check (tallyho/money.py:39-43)
- sym:36499 Ledger (tallyho/ledger.py:24-53)
- sym:36545 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:36526 Money.__sub__ (tallyho/money.py:49-51)
- sym:36525 Money.__add__ (tallyho/money.py:45-47)

---
tokens: 707/800
omitted 184 lower-ranked items (expand an id for detail):
- rec:6f72ed99 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:d38ff0ab decision: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:c5c25c86 attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:9a93196e decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:948b8573 decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:3fbd98d4 decision: Constants that `parse_quantity` depends on live at the top of its modu…
- … and 178 more
