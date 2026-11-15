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
- rec:304af9dc attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n)) drops or adds cents, so the parts stop adding up to the total. Money.allocate(n) splits exactly.
  anchors: tallyho/money.py:Money.allocate
- rec:e02911e7 note, open: `Workflow._move` is on the hot path of the nightly batch job; profile…
- rec:a6b0b26b attempt, open: Tried calling `Repository.order` twice to force a refresh; the second…

## Code
- sym:4680 Order (tallyho/models.py:64-109)
  class Order:
- sym:4656 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:4672 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:4786 Workflow._move (tallyho/workflow.py:29-33)
- sym:4788 Workflow.pay (tallyho/workflow.py:41-44)
- sym:4787 Workflow.place (tallyho/workflow.py:35-39)
- sym:4674 Product.__post_init__ (tallyho/models.py:30-35)
- sym:4683 Order.remove (tallyho/models.py:86-89)
- sym:4784 Workflow (tallyho/workflow.py:23-50)
- sym:4783 _ALLOWED (tallyho/workflow.py:14-20)
- sym:4660 NotFound (tallyho/errors.py:24-25)
- sym:4673 Product (tallyho/models.py:23-35)

---
tokens: 703/800
omitted 55 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:471a0535 attempt, open: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:54716a76 attempt, open: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:15bc3667 attempt, open: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:0916fbe9 attempt, open: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- … and 50 more
