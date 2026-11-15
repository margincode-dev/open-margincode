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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:f1ad2213 attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:ee26fa9c note, open: The repr of objects around `cmd_export` shows up in logs, so keep it s…
- rec:c3257a72 attempt, open: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:449a1018 attempt, open: Replacing the loop in `Money` with a comprehension made it shorter but…

## Code
- sym:3850 Order (tallyho/models.py:64-109)
  class Order:
- sym:3891 quote_order (tallyho/pricing/quote.py:30-53)
- sym:3890 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:3889 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:3849 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:3843 Product (tallyho/models.py:23-35)
  class Product:
- sym:3839 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:3853 Order.remove (tallyho/models.py:86-89)
- sym:3852 Order.add (tallyho/models.py:76-84)
- sym:3820 cmd_products (tallyho/cli.py:20-23)
- sym:3847 OrderLine (tallyho/models.py:51-60)

---
tokens: 726/800
omitted 52 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:10f9f00f attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:955df7f5 decision, open: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- rec:d489452d decision, open: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:a8ae797d attempt, open: Moving the imports inside `Money.zero` to speed up start-up saved unde…
- rec:c784eb79 attempt, open: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- … and 46 more
