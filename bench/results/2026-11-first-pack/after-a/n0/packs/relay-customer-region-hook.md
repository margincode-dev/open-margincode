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
- rec:ea4df8df decision: Errors about customer data must not echo the rejected value: they are…
- rec:e40561a1 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:653eb548 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:2c197979 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…

## Code
- sym:17438 check_region (tallyho/validate.py:24-26)
- sym:17436 check_sku (tallyho/validate.py:14-16)
- sym:17332 Product.__post_init__ (tallyho/models.py:30-35)
- sym:17440 parse_quantity (tallyho/validate.py:34-42)
- sym:17437 check_email (tallyho/validate.py:19-21)
- sym:17439 require_positive (tallyho/validate.py:29-31)
- sym:17314 ValidationError (tallyho/errors.py:8-17)
- sym:17331 Product (tallyho/models.py:23-35)
  class Product:
- sym:17338 Order (tallyho/models.py:64-109)
- sym:17435 _REGION (tallyho/validate.py:11-11)
- sym:17333 Customer (tallyho/models.py:39-47)
- sym:17411 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 716/800
omitted 38 lower-ranked items (expand an id for detail):
- rec:fb85ea43 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:0ffd12ce decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:c8935aac decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:7cd82051 intent: Earlier work on this code
- sym:17433 _SKU (tallyho/validate.py:9-9)
- sym:17316 ValidationError.__str__ (tallyho/errors.py:15-17)
- sym:17313 TallyError (tallyho/errors.py:4-5)
- sym:17326 Ledger.post (tallyho/ledger.py:28-35)
- … and 30 more
