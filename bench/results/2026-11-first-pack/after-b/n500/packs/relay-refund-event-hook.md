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
- rec:a647a6dc decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:2bb32981 decision: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:e1e81051 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid…
- rec:fea44599 attempt: Caching what `Order` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/models.py:Order

## Code
- sym:36678 Order (tallyho/models.py:64-109)
  class Order:
- sym:36784 Workflow._move (tallyho/workflow.py:29-33)
- sym:36786 Workflow.pay (tallyho/workflow.py:41-44)
- sym:36785 Workflow.place (tallyho/workflow.py:35-39)
- sym:36782 Workflow (tallyho/workflow.py:23-50)
- sym:36781 _ALLOWED (tallyho/workflow.py:14-20)
- sym:36788 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:36680 Order.add (tallyho/models.py:76-84)
- sym:36681 Order.remove (tallyho/models.py:86-89)
- sym:36654 ValidationError (tallyho/errors.py:8-17)
- sym:36670 OrderStatus (tallyho/models.py:14-19)
- sym:36653 TallyError (tallyho/errors.py:4-5)

---
tokens: 714/800
omitted 206 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:bebcc601 decision: Type hints on `Order` use the built-in generics (list[str], dict[str,…
- rec:3964d858 note: Benchmarks for `ValidationError` are in a separate, manual job; the un…
- rec:c11293e9 attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:fcfb8494 attempt: Wrapping `Repository.save_order` in a bare try/except with a retry loo…
- rec:368518cc attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- … and 200 more
