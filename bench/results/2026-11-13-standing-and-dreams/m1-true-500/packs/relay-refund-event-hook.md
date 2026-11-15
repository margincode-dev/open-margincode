## Task
# relay-refund-event · relay-refund-event · open

## Map
- tallyho/errors.py
- tallyho/models.py
- tests/conftest.py
- tallyho/ledger.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/pricing/quote.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- tallyho/store/jsonfile.py
- tallyho/store/memory.py
- tallyho/validate.py
- tallyho/workflow.py
- tests/test_models.py
- tests/test_pricing.py
- tallyho/cli.py

## Decisions & dead ends
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:fa91e31c decision, open: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:82cafd51 decision, open: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:6a04f31f attempt, open: Wrapping `ValidationError` in a bare try/except with a retry loop hid…

## Code
- sym:6340 Order (tallyho/models.py:64-109)
  class Order:
- sym:6446 Workflow._move (tallyho/workflow.py:29-33)
- sym:6448 Workflow.pay (tallyho/workflow.py:41-44)
- sym:6447 Workflow.place (tallyho/workflow.py:35-39)
- sym:6444 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:6443 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:6450 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:6342 Order.add (tallyho/models.py:76-84)
- sym:6343 Order.remove (tallyho/models.py:86-89)
- sym:6316 ValidationError (tallyho/errors.py:8-17)
- sym:6332 OrderStatus (tallyho/models.py:14-19)
- sym:6315 TallyError (tallyho/errors.py:4-5)

---
tokens: 702/800
omitted 207 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:fbdcda90 attempt, open: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:958ad5a3 decision, open: Type hints on `Order` use the built-in generics (list[str], dict[str,…
- rec:7b502763 note, open: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:f627aec9 attempt, open: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- … and 202 more
