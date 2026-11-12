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
- rec:7790c164 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:3d7b5794 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:85023574 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:32296 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:32294 check_sku (tallyho/validate.py:14-16)
- sym:32190 Product.__post_init__ (tallyho/models.py:30-35)
- sym:32298 parse_quantity (tallyho/validate.py:34-42)
- sym:32295 check_email (tallyho/validate.py:19-21)
- sym:32297 require_positive (tallyho/validate.py:29-31)
- sym:32172 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:32189 Product (tallyho/models.py:23-35)
  class Product:
- sym:32196 Order (tallyho/models.py:64-109)
- sym:32293 _REGION (tallyho/validate.py:11-11)
- sym:32191 Customer (tallyho/models.py:39-47)
- sym:32269 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 719/800
omitted 41 lower-ranked items (expand an id for detail):
- rec:7a268184 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:0dd40680 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:96a85291 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:7c13cd8d attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:88fbcdde attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:616856dd decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- … and 35 more
