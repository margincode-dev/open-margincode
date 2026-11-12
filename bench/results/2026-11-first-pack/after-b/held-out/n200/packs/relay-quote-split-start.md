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
- rec:ce6c56a0 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:2f0d48ec note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:dcaa6f80 decision: New optional parameters of `ValidationError` are keyword-only (after *…

## Code
- sym:38189 Money.allocate (tallyho/money.py:63-68)
  def allocate(self, parts: int) -> list[Money]:
- sym:38181 Money (tallyho/money.py:22-83)
  class Money:
- sym:38159 Ledger (tallyho/ledger.py:24-53)
- sym:38160 Ledger.post (tallyho/ledger.py:28-35)
- sym:38206 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:38148 ValidationError (tallyho/errors.py:8-17)
- sym:38172 Order (tallyho/models.py:64-109)
- sym:38184 Money._check (tallyho/money.py:39-43)
- sym:38186 Money.__sub__ (tallyho/money.py:49-51)
- sym:38205 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:38185 Money.__add__ (tallyho/money.py:45-47)
- sym:38211 Quote (tallyho/pricing/quote.py:16-27)

---
tokens: 714/800
omitted 83 lower-ranked items (expand an id for detail):
- rec:26d07321 attempt: Tried calling `check_sku` twice to force a refresh; the second call re…
- rec:4534f323 attempt: Replacing the loop in `TallyError` with a comprehension made it shorte…
- rec:1e8c468b attempt: Caching what `Product.__post_init__` returns in a module-level dict br…
- rec:133f3eff attempt: Tried calling `Ledger.post` twice to force a refresh; the second call…
- rec:e5b43e33 attempt: Caching what `JsonFileStore._path` returns in a module-level dict brok…
- rec:c9f3ec63 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- … and 77 more
