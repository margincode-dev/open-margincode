## Task
# relay-ledger-refund · relay-ledger-refund · open

## Map
- tallyho/ledger.py
- tallyho/pricing/quote.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/tax.py
- tallyho/workflow.py
- tallyho/config.py
- tallyho/errors.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
- rec:0818f094 attempt: Caching what `Ledger.record_sale` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:c7f0e9a8 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:f03d65d9 decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str], dict[str, int]), not the typing aliases.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:16dbe918 attempt: Replacing the loop in the `quote` module with a comprehension made it…

## Code
- sym:13633 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:13752 Workflow.pay (tallyho/workflow.py:41-44)
- sym:13631 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:13685 quote_order (tallyho/pricing/quote.py:30-53)
- sym:13683 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:13636 OrderStatus (tallyho/models.py:14-19)
- sym:13748 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:13713 RecordStore.get (tallyho/store/base.py:28-29)
- sym:13708 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:13624 NotFound (tallyho/errors.py:24-25)
- sym:13747 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 713/800
omitted 190 lower-ranked items (expand an id for detail):
- rec:11364a0a attempt: Adding type: ignore comments around `Quote` to quiet the checker hid a…
- rec:dae96d27 attempt: Tried calling `Ledger.trial_balance_ok` twice to force a refresh; the…
- rec:f0814ba6 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:85f7c2aa attempt: Wrapping `RecordStore.put` in a bare try/except with a retry loop hid…
- rec:48a849f9 decision: New optional parameters of `RecordStore.get` are keyword-only (after *…
- rec:5bce34b1 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- … and 184 more
