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
- rec:3739f1ce attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:fc1b45dc attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:082c992d note: The repr of objects around `ValidationError` shows up in logs, so keep it short and free of large nested data.
  anchors: tallyho/errors.py:ValidationError

## Code
- sym:39849 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:39841 Money (tallyho/money.py:22-83)
  class Money:
- sym:39871 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:39872 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:39866 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:39808 ValidationError (tallyho/errors.py:8-17)
- sym:39832 Order (tallyho/models.py:64-109)
- sym:39844 Money._check (tallyho/money.py:39-43)
- sym:39819 Ledger (tallyho/ledger.py:24-53)
- sym:39865 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:39846 Money.__sub__ (tallyho/money.py:49-51)

---
tokens: 702/800
omitted 202 lower-ranked items (expand an id for detail):
- rec:f4292900 attempt: Tried calling `ValidationError` twice to force a refresh; the second c…
- rec:cfe85efb decision: Log lines from `ValidationError` go through the logging module, never…
- rec:b68994c1 decision: Constants that `ValidationError` depends on live at the top of its mod…
- rec:5c053ae8 decision: `ValidationError` keeps its docstring as the one place its behaviour i…
- rec:09a60d51 attempt: Wrapping `Money.amount` in a bare try/except with a retry loop hid the…
- rec:a3d8fd7b decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- … and 196 more
