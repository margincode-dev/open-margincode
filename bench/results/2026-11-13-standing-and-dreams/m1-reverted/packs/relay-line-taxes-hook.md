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
- rec:1dd0c297 decision, reverted: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- rec:e0e8f06d decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:c9d2cf73 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:80d90b0b intent, open: Earlier work on this code

## Code
- sym:239 quote_order (tallyho/pricing/quote.py:30-53)
  def quote_order( order: Order, coupons: CouponBook | None = None, taxes: TaxTable | None = None, today: date | None = None, ) -> Quote:
- sym:238 Quote.lines (tallyho/pricing/quote.py:23-27)
  def lines(self) -> list[str]:
- sym:237 Quote (tallyho/pricing/quote.py:16-27)
- sym:198 Order (tallyho/models.py:64-109)
- sym:200 Order.add (tallyho/models.py:76-84)
- sym:242 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:201 Order.remove (tallyho/models.py:86-89)
- sym:191 Product (tallyho/models.py:23-35)
- sym:253 Repository (tallyho/repository.py:32-74)
- sym:322 test_quote_taxes_only_taxable (tests/test_pricing.py:20-24)
- sym:197 OrderLine.subtotal (tallyho/models.py:59-60)
- sym:199 Order.currency (tallyho/models.py:73-74)

---
tokens: 711/800
omitted 41 lower-ranked items (expand an id for detail):
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)
- rec:da2f8292 decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:c2037aa5 decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:22c91061 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:3f8cfb0c note, open: merge f68c61491f5c reverted by bench-merger in c16cb466ed46: the bench…
- … and 36 more
