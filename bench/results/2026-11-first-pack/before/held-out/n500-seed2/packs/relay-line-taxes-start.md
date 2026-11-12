## Task
# relay-line-taxes · relay-line-taxes · open

## Map
- tallyho/pricing/quote.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/pricing/tax.py
- tallyho/report/csv_export.py
- tallyho/repository.py
- tallyho/workflow.py
- tests/conftest.py
- tests/test_pricing.py
- tallyho/cli.py
- tallyho/config.py
- tallyho/ledger.py

## Decisions & dead ends
- rec:2f3eca67 attempt: Moving the imports inside `quote_order` to speed up start-up saved under a millisecond and confused the linter.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:bcd520ae attempt: Replacing the loop in `quote_order` with a comprehension made it shorter but much harder to step through in a debugger, so it was reverted.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:1b02640c attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:3bb87f53 attempt: Tried calling `quote_order` twice to force a refresh; the second call…

## Code
- sym:13851 quote_order (tallyho/pricing/quote.py:30-53)
- sym:13850 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:13849 Quote (tallyho/pricing/quote.py:16-27)
- sym:13810 Order (tallyho/models.py:64-109)
- sym:13812 Order.add (tallyho/models.py:76-84)
- sym:13854 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:13803 Product (tallyho/models.py:23-35)
- sym:13813 Order.remove (tallyho/models.py:86-89)
- sym:13865 Repository (tallyho/repository.py:32-74)
- sym:13874 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:13809 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:13811 Order.currency (tallyho/models.py:73-74)

---
tokens: 726/800
omitted 152 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:5d299909 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:21469a6d attempt: Caching what `Repository.product` returns in a module-level dict broke…
- rec:8291cb7a attempt: Replacing the loop in `Repository.product` with a comprehension made i…
- rec:f346c44a attempt: Adding type: ignore comments around `Quote` to quiet the checker hid a…
- rec:92d11ece attempt: Tried calling `apply_discounts` twice to force a refresh; the second c…
- rec:e5997631 attempt: Replacing the loop in `default_tax_table` with a comprehension made it…
- … and 145 more
