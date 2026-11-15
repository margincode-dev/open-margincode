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
- rec:afc66d6f decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:7464b839 decision, open: Event payloads carry money as integer minor units plus the currency code: {"id": ..., "amount_minor": 4681, "currency": "USD"}. Never decimals or strings; consumers are in other languages.
  anchors: tallyho/workflow.py:Workflow
- rec:a6b0b26b attempt, open: Tried calling `Repository.order` twice to force a refresh; the second…

## Code
- sym:4680 Order (tallyho/models.py:64-109)
  class Order:
- sym:4786 Workflow._move (tallyho/workflow.py:29-33)
- sym:4788 Workflow.pay (tallyho/workflow.py:41-44)
- sym:4787 Workflow.place (tallyho/workflow.py:35-39)
- sym:4784 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:4783 _ALLOWED (tallyho/workflow.py:14-20)
  _ALLOWED = {
- sym:4790 Workflow.cancel (tallyho/workflow.py:49-50)
- sym:4682 Order.add (tallyho/models.py:76-84)
- sym:4683 Order.remove (tallyho/models.py:86-89)
- sym:4656 ValidationError (tallyho/errors.py:8-17)
- sym:4672 OrderStatus (tallyho/models.py:14-19)
- sym:4655 TallyError (tallyho/errors.py:4-5)

---
tokens: 727/800
omitted 72 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:54716a76 attempt, open: Replacing the loop in `Workflow.ship` with a comprehension made it sho…
- rec:15bc3667 attempt, open: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:304af9dc attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:4cf2a217 decision, open: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:f5322134 decision, open: Errors about customer data must not echo the rejected value: they are…
- … and 66 more
