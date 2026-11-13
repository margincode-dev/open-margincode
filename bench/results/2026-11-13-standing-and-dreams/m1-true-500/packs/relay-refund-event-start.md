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
- rec:c1c7e649 attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:28885542 note, open: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:6a04f31f attempt, open: Wrapping `ValidationError` in a bare try/except with a retry loop hid…

## Code
- sym:6340 Order (tallyho/models.py:64-109)
  class Order:
- sym:6446 Workflow._move (tallyho/workflow.py:29-33)
- sym:6448 Workflow.pay (tallyho/workflow.py:41-44)
- sym:6447 Workflow.place (tallyho/workflow.py:35-39)
- sym:6343 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:6316 ValidationError (tallyho/errors.py:8-17)
- sym:6444 Workflow (tallyho/workflow.py:23-50)
- sym:6443 _ALLOWED (tallyho/workflow.py:14-20)
- sym:6332 OrderStatus (tallyho/models.py:14-19)
- sym:6315 TallyError (tallyho/errors.py:4-5)
- sym:6328 Ledger.post (tallyho/ledger.py:28-35)
- sym:6342 Order.add (tallyho/models.py:76-84)

---
tokens: 730/800
omitted 202 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:fbdcda90 attempt, open: Caching what `Order` returns in a module-level dict broke tests that e…
- rec:958ad5a3 decision, open: Type hints on `Order` use the built-in generics (list[str], dict[str,…
- rec:82cafd51 decision, open: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:7b502763 note, open: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:0f9a20ca attempt, open: Adding type: ignore comments around `check_email` to quiet the checker…
- … and 196 more
