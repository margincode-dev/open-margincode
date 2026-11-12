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
- rec:58af22fa attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:d7bec680 note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platform default encoding differs on the Windows build agents.
  anchors: tallyho/errors.py:ValidationError.__str__
- rec:6d64a1a6 attempt: Caching what `Order` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/models.py:Order

## Code
- sym:38338 Order (tallyho/models.py:64-109)
  class Order:
- sym:38341 Order.remove (tallyho/models.py:86-89)
- sym:38314 ValidationError (tallyho/errors.py:8-17)
- sym:38330 OrderStatus (tallyho/models.py:14-19)
- sym:38340 Order.add (tallyho/models.py:76-84)
- sym:38444 Workflow._move (tallyho/workflow.py:29-33)
- sym:38446 Workflow.pay (tallyho/workflow.py:41-44)
- sym:38445 Workflow.place (tallyho/workflow.py:35-39)
- sym:38332 Product.__post_init__ (tallyho/models.py:30-35)
- sym:38442 Workflow (tallyho/workflow.py:23-50)
- sym:38441 _ALLOWED (tallyho/workflow.py:14-20)
- sym:38318 NotFound (tallyho/errors.py:24-25)

---
tokens: 714/800
omitted 95 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:92acadf0 decision: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:c5e15b78 attempt: Moving the imports inside `Repository.save_order` to speed up start-up…
- rec:6e33de80 attempt: Tried calling `check_sku` twice to force a refresh; the second call re…
- rec:53299d52 attempt: Replacing the loop in `TallyError` with a comprehension made it shorte…
- rec:fd4f03d2 attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- … and 89 more
