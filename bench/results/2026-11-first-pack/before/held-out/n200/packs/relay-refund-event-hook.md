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
- rec:394573e7 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:3ae994cb attempt: Caching what `Order` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/models.py:Order
- rec:3b8045e7 attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:bd5fd517 decision: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:12980 Order (tallyho/models.py:64-109)
  class Order:
- sym:13086 Workflow._move (tallyho/workflow.py:29-33)
- sym:13088 Workflow.pay (tallyho/workflow.py:41-44)
- sym:13087 Workflow.place (tallyho/workflow.py:35-39)
- sym:13084 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:13083 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:13090 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:12982 Order.add (tallyho/models.py:76-84)
- sym:12983 Order.remove (tallyho/models.py:86-89)
- sym:12956 ValidationError (tallyho/errors.py:8-17)
- sym:12972 OrderStatus (tallyho/models.py:14-19)
- sym:12955 TallyError (tallyho/errors.py:4-5)

---
tokens: 704/800
omitted 94 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:c44a00f4 decision: New optional parameters of `Workflow.cancel` are keyword-only (after *…
- rec:48abc96f decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:bd59a937 attempt: Wrapping `Order.currency` in a bare try/except with a retry loop hid t…
- rec:16f4856a decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:c6fa493e attempt: Moving the imports inside `Repository.save_order` to speed up start-up…
- … and 88 more
