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
- rec:3ae994cb attempt: Caching what `Order` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/models.py:Order
- rec:394573e7 decision: New optional parameters of `ValidationError` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/errors.py:ValidationError
- rec:c6fa493e attempt: Moving the imports inside `Repository.save_order` to speed up start-up…
- rec:b4b68bd4 attempt: Tried calling `check_sku` twice to force a refresh; the second call re…

## Code
- sym:12980 Order (tallyho/models.py:64-109)
  class Order:
- sym:12983 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:12956 ValidationError (tallyho/errors.py:8-17)
- sym:12972 OrderStatus (tallyho/models.py:14-19)
- sym:12982 Order.add (tallyho/models.py:76-84)
- sym:13086 Workflow._move (tallyho/workflow.py:29-33)
- sym:13088 Workflow.pay (tallyho/workflow.py:41-44)
- sym:13087 Workflow.place (tallyho/workflow.py:35-39)
- sym:12974 Product.__post_init__ (tallyho/models.py:30-35)
- sym:13084 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:13083 _ALLOWED (tallyho/workflow.py:14-20)
- sym:12960 NotFound (tallyho/errors.py:24-25)

---
tokens: 722/800
omitted 94 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:e528481d attempt: Replacing the loop in `TallyError` with a comprehension made it shorte…
- rec:3cf33a60 attempt: Wrapping `CouponBook.add` in a bare try/except with a retry loop hid t…
- rec:b0aabef0 attempt: Caching what `Product.__post_init__` returns in a module-level dict br…
- rec:3b8045e7 attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:8e1b142a attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- … and 88 more
