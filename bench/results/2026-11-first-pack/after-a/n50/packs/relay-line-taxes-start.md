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
- rec:8e77f0b0 attempt: Taxing each line separately (taxes.tax per line) and rounding per line drifts from Quote.tax by a cent or more. Compute the tax once, then split it across lines by largest remainder.
  anchors: tallyho/pricing/quote.py:quote_order
- rec:989e9e0b attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:6be421bd attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:0af4d9b5 attempt: Replacing the loop in `Money` with a comprehension made it shorter but…

## Code
- sym:19330 Order (tallyho/models.py:64-109)
  class Order:
- sym:19371 quote_order (tallyho/pricing/quote.py:30-53)
- sym:19370 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:19369 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:19329 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:19323 Product (tallyho/models.py:23-35)
- sym:19319 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:19333 Order.remove (tallyho/models.py:86-89)
- sym:19332 Order.add (tallyho/models.py:76-84)
- sym:19300 cmd_products (tallyho/cli.py:20-23)
- sym:19327 OrderLine (tallyho/models.py:51-60)
- sym:19324 Product.__post_init__ (tallyho/models.py:30-35)

---
tokens: 716/800
omitted 51 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:e82f8b2d decision: `Workflow.pay` keeps its docstring as the one place its behaviour is d…
- rec:7e433e0b decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:a117aa82 attempt: Moving the imports inside `Money.zero` to speed up start-up saved unde…
- rec:1f73dc6c attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:3e2c0c8d attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- … and 45 more
