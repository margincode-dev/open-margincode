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
- rec:97c09cc8 decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:7af98ca8 decision: Type hints on `Ledger.record_sale` use the built-in generics (list[str…
- rec:7b7105a7 attempt: Adding type: ignore comments around `Quote` to quiet the checker hid a…
- rec:540f4571 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…

## Code
- sym:38991 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:39110 Workflow.pay (tallyho/workflow.py:41-44)
  def pay(self, order: Order, on: date) -> None:
- sym:39043 quote_order (tallyho/pricing/quote.py:30-53)
- sym:39041 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:38994 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:39106 Workflow (tallyho/workflow.py:23-50)
- sym:38989 Ledger (tallyho/ledger.py:24-53)
- sym:39105 _ALLOWED (tallyho/workflow.py:14-20)
- sym:38990 Ledger.post (tallyho/ledger.py:28-35)
- sym:39003 Order.currency (tallyho/models.py:73-74)
- sym:39011 Money (tallyho/money.py:22-83)

---
tokens: 709/800
omitted 182 lower-ranked items (expand an id for detail):
- rec:8ec3fa36 attempt: Caching what `Ledger.record_sale` returns in a module-level dict broke…
- rec:38696f3f attempt: Moving the imports inside `quote_order` to speed up start-up saved und…
- rec:a702228e attempt: Replacing the loop in `quote_order` with a comprehension made it short…
- rec:218a0c67 attempt: Tried calling `TaxTable.tax` twice to force a refresh; the second call…
- rec:a3ec0ddb attempt: Adding type: ignore comments around `total` to quiet the checker hid a…
- rec:764ea496 attempt: Wrapping the `ledger` module in a bare try/except with a retry loop hi…
- … and 176 more
