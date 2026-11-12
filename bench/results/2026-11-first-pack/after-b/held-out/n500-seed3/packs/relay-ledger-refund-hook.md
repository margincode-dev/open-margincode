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
- rec:67494c6d decision: A refund is booked as the exact reverse of the sale: the same three accounts with debit and credit swapped, memo "refund <order_id>", so reports pair a sale and its refund by id.
  anchors: tallyho/ledger.py:Ledger.record_sale
- rec:2674d8f9 decision: New optional parameters of `Repository._order_from_record` are keyword…
- rec:ba79aa62 decision: Constants that `Quote` depends on live at the top of its module in UPP…
- rec:ce818a08 note: Benchmarks for `Quote` are in a separate, manual job; the unit tests m…

## Code
- sym:40651 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:40770 Workflow.pay (tallyho/workflow.py:41-44)
- sym:40649 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:40703 quote_order (tallyho/pricing/quote.py:30-53)
- sym:40701 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:40654 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:40766 Workflow (tallyho/workflow.py:23-50)
  class Workflow:
- sym:40731 RecordStore.get (tallyho/store/base.py:28-29)
- sym:40726 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:40642 NotFound (tallyho/errors.py:24-25)
- sym:40765 _ALLOWED (tallyho/workflow.py:14-20)

---
tokens: 712/800
omitted 204 lower-ranked items (expand an id for detail):
- rec:e8c8df7e attempt: Tried calling `Order.currency` twice to force a refresh; the second ca…
- rec:04a6ddda attempt: Wrapping `Money._check` in a bare try/except with a retry loop hid the…
- rec:299a9a33 attempt: Tried calling `Ledger.balances` twice to force a refresh; the second c…
- rec:fac3c507 attempt: Tried calling `Money.format` twice to force a refresh; the second call…
- rec:80be007a attempt: Wrapping `Discount.amount_off` in a bare try/except with a retry loop…
- rec:65770ce9 attempt: Tried calling `Discount.amount_off` twice to force a refresh; the seco…
- … and 198 more
