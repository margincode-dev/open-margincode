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
- rec:b53d2703 attempt: Tried calling `ValidationError` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/errors.py:ValidationError
- rec:1e3d69f4 decision: Log lines from `ValidationError` go through the logging module, never print(), so operators can filter them by logger name.
  anchors: tallyho/errors.py:ValidationError
- rec:d2e5d0dc decision: Constants that `ValidationError` depends on live at the top of its mod…
- rec:0ad48987 decision: `ValidationError` keeps its docstring as the one place its behaviour i…

## Code
- sym:14491 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:14483 Money (tallyho/money.py:22-83)
  class Money:
- sym:14513 Quote (tallyho/pricing/quote.py:16-27)
- sym:14514 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:14508 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:14450 ValidationError (tallyho/errors.py:8-17)
- sym:14474 Order (tallyho/models.py:64-109)
- sym:14486 Money._check (tallyho/money.py:39-43)
- sym:14461 Ledger (tallyho/ledger.py:24-53)
- sym:14507 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:14488 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 720/800
omitted 189 lower-ranked items (expand an id for detail):
- rec:a90f483a note: The repr of objects around `ValidationError` shows up in logs, so keep…
- rec:9a45cf96 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:caee7bb3 attempt: Wrapping `Money.amount` in a bare try/except with a retry loop hid the…
- rec:03da12b9 attempt: Wrapping `Money.of` in a bare try/except with a retry loop hid the rea…
- rec:5deaea1f attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:723f0613 note: Remember `Money.format` is not thread-safe; each worker needs its own…
- … and 183 more
