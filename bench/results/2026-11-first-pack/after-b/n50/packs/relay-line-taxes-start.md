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
- rec:5bedb2c3 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:287583da note: The repr of objects around `cmd_export` shows up in logs, so keep it s…
- rec:f6d788d5 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:b87f7e09 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…

## Code
- sym:34188 Order (tallyho/models.py:64-109)
  class Order:
- sym:34229 quote_order (tallyho/pricing/quote.py:30-53)
- sym:34228 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:34227 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:34187 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:34181 Product (tallyho/models.py:23-35)
- sym:34177 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:34191 Order.remove (tallyho/models.py:86-89)
- sym:34190 Order.add (tallyho/models.py:76-84)
- sym:34158 cmd_products (tallyho/cli.py:20-23)
- sym:34185 OrderLine (tallyho/models.py:51-60)
- sym:34182 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 706/800
omitted 51 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:140b9c0a attempt: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:f7988e89 decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- rec:76b2e130 decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:a60e5f9c attempt: Moving the imports inside `Money.zero` to speed up start-up saved unde…
- rec:98cb9608 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- … and 45 more
