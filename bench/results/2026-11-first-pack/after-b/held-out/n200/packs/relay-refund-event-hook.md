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
- rec:ef44e68b decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:029070e4 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:92acadf0 decision: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:6d64a1a6 attempt: Caching what `Order` returns in a module-level dict broke tests that e…

## Code
- sym:38338 Order (tallyho/models.py:64-109)
  class Order:
- sym:38444 Workflow._move (tallyho/workflow.py:29-33)
- sym:38446 Workflow.pay (tallyho/workflow.py:41-44)
- sym:38445 Workflow.place (tallyho/workflow.py:35-39)
- sym:38442 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:38441 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:38448 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:38340 Order.add (tallyho/models.py:76-84)
- sym:38341 Order.remove (tallyho/models.py:86-89)
- sym:38314 ValidationError (tallyho/errors.py:8-17)
- sym:38330 OrderStatus (tallyho/models.py:14-19)
- sym:38313 TallyError (tallyho/errors.py:4-5)

---
tokens: 714/800
omitted 107 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:520428f6 attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:5fe90fd4 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:fd4f03d2 attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:fe057ca3 attempt: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:53299d52 attempt: Replacing the loop in `TallyError` with a comprehension made it shorte…
- … and 101 more
