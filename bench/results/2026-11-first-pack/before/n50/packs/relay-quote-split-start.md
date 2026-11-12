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
- rec:38d6d10d attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:bcb0a95f attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:bfd19541 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…

## Code
- sym:7821 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:7851 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:7822 Ledger.post (tallyho/ledger.py:28-35)
  def post(self, on: date, memo: str, *legs: tuple[str, Money, Money]) -> None:
- sym:7868 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
  def __post_init__(self) -> None:
- sym:7810 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:7834 Order (tallyho/models.py:64-109)
- sym:7843 Money (tallyho/money.py:22-83)
- sym:7813 CurrencyMismatch (tallyho/errors.py:20-21)
- sym:7827 Product (tallyho/models.py:23-35)
- sym:7831 OrderLine (tallyho/models.py:51-60)
- sym:7867 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:7873 Quote (tallyho/pricing/quote.py:16-27)

---
tokens: 714/800
omitted 52 lower-ranked items (expand an id for detail):
- rec:a43cec69 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:3dca7fa1 decision: Errors about customer data must not echo the rejected value: they are…
- rec:61f36b62 decision: Tests for `TallyError` build their inputs with small factory helpers i…
- rec:57bfb220 decision: New optional parameters of `parse_quantity` are keyword-only (after *)…
- rec:1d0f89af decision: `_exponent` keeps its docstring as the one place its behaviour is desc…
- rec:5bd9f3df attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- … and 46 more
