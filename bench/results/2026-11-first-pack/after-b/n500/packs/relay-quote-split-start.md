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
- rec:8abcf1f2 note: The repr of objects around `ValidationError.__init__` shows up in logs…
- rec:8b4b96e2 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid…

## Code
- sym:36529 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:36546 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:36488 ValidationError (tallyho/errors.py:8-17)
- sym:36521 Money (tallyho/money.py:22-83)
  class Money:
- sym:36499 Ledger (tallyho/ledger.py:24-53)
- sym:36545 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:36551 Quote (tallyho/pricing/quote.py:16-27)
- sym:36552 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:36487 TallyError (tallyho/errors.py:4-5)
- sym:36500 Ledger.post (tallyho/ledger.py:28-35)
- sym:36506 Product.__post_init__ (tallyho/models.py:30-35)
- sym:36514 Order.add (tallyho/models.py:76-84)

---
tokens: 709/800
omitted 165 lower-ranked items (expand an id for detail):
- rec:d38ff0ab decision: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:6f72ed99 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:47148510 attempt: Adding type: ignore comments around `check_email` to quiet the checker…
- rec:c5c25c86 attempt: Wrapping `Money.allocate` in a bare try/except with a retry loop hid t…
- rec:4fa1fbdb attempt: Tried calling `check_email` twice to force a refresh; the second call…
- rec:0ff1da70 attempt: Tried calling `JsonFileStore._path` twice to force a refresh; the seco…
- … and 159 more
