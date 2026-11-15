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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:1ce5dee5 decision, open: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:a22d4b72 attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:288d969d attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:1852 Product.__post_init__ (tallyho/models.py:30-35)
  def __post_init__(self) -> None:
- sym:1834 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:1958 check_region (tallyho/validate.py:24-26)
- sym:1960 parse_quantity (tallyho/validate.py:34-42)
- sym:1957 check_email (tallyho/validate.py:19-21)
- sym:1956 check_sku (tallyho/validate.py:14-16)
- sym:1959 require_positive (tallyho/validate.py:29-31)
- sym:1851 Product (tallyho/models.py:23-35)
  class Product:
- sym:1838 NotFound (tallyho/errors.py:24-25)
- sym:1858 Order (tallyho/models.py:64-109)
  class Order:
- sym:1853 Customer (tallyho/models.py:39-47)
- sym:1836 ValidationError.__str__ (tallyho/errors.py:15-17)

---
tokens: 722/800
omitted 36 lower-ranked items (expand an id for detail):
- rec:b6c3606e decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:8c08f1af decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:1835 ValidationError.__init__ (tallyho/errors.py:11-13)
- sym:1931 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:1832 load_config (tallyho/config.py:23-47)
- sym:1833 TallyError (tallyho/errors.py:4-5)
- sym:1846 Ledger.post (tallyho/ledger.py:28-35)
- … and 29 more
