## Task
# relay-customer-region · relay-customer-region · open
Goal: Customer regions are never checked, so typos such as "us-ca" end up in the store. Validate the region when a Customer is created and raise ValidationError (with field "region") for an invalid one.

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
- rec:f6716ed2 decision: Errors about customer data must not echo the rejected value: they are…
- rec:7f4e6732 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:0d1ded20 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:b240ecee decision: New optional parameters of `Repository.product` are keyword-only (afte…

## Code
- sym:6938 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:6936 check_sku (tallyho/validate.py:14-16)
- sym:6832 Product.__post_init__ (tallyho/models.py:30-35)
- sym:6940 parse_quantity (tallyho/validate.py:34-42)
- sym:6937 check_email (tallyho/validate.py:19-21)
- sym:6939 require_positive (tallyho/validate.py:29-31)
- sym:6814 ValidationError (tallyho/errors.py:8-17)
- sym:6831 Product (tallyho/models.py:23-35)
  class Product:
- sym:6838 Order (tallyho/models.py:64-109)
  class Order:
- sym:6935 _REGION (tallyho/validate.py:11-11)
- sym:6833 Customer (tallyho/models.py:39-47)
- sym:6911 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 717/800
omitted 55 lower-ranked items (expand an id for detail):
- rec:06c9c012 attempt: Moving the imports inside `Ledger.post` to speed up start-up saved und…
- rec:f825fa52 attempt: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:f393fc54 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:506bb54b attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:96d4064f note: The `scan` module reads files as UTF-8 explicitly; the platform defaul…
- rec:34b3c7d7 note: The `cli` module reads files as UTF-8 explicitly; the platform default…
- … and 49 more
