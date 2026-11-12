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
- rec:9ec22164 decision: Errors about customer data must not echo the rejected value: they are…
- rec:6e7a0a59 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:a3adf76d attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:ce589140 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…

## Code
- sym:5278 check_region (tallyho/validate.py:24-26)
- sym:5276 check_sku (tallyho/validate.py:14-16)
- sym:5172 Product.__post_init__ (tallyho/models.py:30-35)
- sym:5280 parse_quantity (tallyho/validate.py:34-42)
- sym:5277 check_email (tallyho/validate.py:19-21)
- sym:5279 require_positive (tallyho/validate.py:29-31)
- sym:5154 ValidationError (tallyho/errors.py:8-17)
- sym:5171 Product (tallyho/models.py:23-35)
  class Product:
- sym:5178 Order (tallyho/models.py:64-109)
- sym:5275 _REGION (tallyho/validate.py:11-11)
- sym:5173 Customer (tallyho/models.py:39-47)
- sym:5251 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 721/800
omitted 38 lower-ranked items (expand an id for detail):
- rec:c4da6b94 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:c2c25629 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:e05f65d8 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:52764a48 intent: Earlier work on this code
- sym:5273 _SKU (tallyho/validate.py:9-9)
- sym:5156 ValidationError.__str__ (tallyho/errors.py:15-17)
- sym:5153 TallyError (tallyho/errors.py:4-5)
- sym:5166 Ledger.post (tallyho/ledger.py:28-35)
- … and 30 more
