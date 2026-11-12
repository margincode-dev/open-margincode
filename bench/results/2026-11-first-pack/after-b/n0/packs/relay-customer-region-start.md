## Task
# relay-customer-region · relay-customer-region · open

## Map
- tallyho/errors.py
- tallyho/ledger.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/store/jsonfile.py
- tallyho/store/memory.py
- tallyho/validate.py
- tallyho/workflow.py
- tests/test_models.py
- tallyho/events.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
- rec:3381e43d decision: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:3d7b5794 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:85023574 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:96a85291 decision: A refund is booked as the exact reverse of the sale: the same three ac…

## Code
- sym:32296 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:32298 parse_quantity (tallyho/validate.py:34-42)
- sym:32294 check_sku (tallyho/validate.py:14-16)
- sym:32295 check_email (tallyho/validate.py:19-21)
- sym:32297 require_positive (tallyho/validate.py:29-31)
- sym:32190 Product.__post_init__ (tallyho/models.py:30-35)
- sym:32172 ValidationError (tallyho/errors.py:8-17)
- sym:32196 Order (tallyho/models.py:64-109)
  class Order:
- sym:32269 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:32171 TallyError (tallyho/errors.py:4-5)
- sym:32184 Ledger.post (tallyho/ledger.py:28-35)
- sym:32176 NotFound (tallyho/errors.py:24-25)

---
tokens: 703/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:0dd40680 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:32198 Order.add (tallyho/models.py:76-84)
- sym:32199 Order.remove (tallyho/models.py:86-89)
- sym:32204 _exponent (tallyho/money.py:14-18)
- sym:32213 Money.allocate (tallyho/money.py:63-68)
- sym:32220 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:32230 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- … and 24 more
