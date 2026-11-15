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
- rec:a27b3c1a decision, open: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:675cce1b decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:3fa380b3 attempt, open: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:1726c433 decision, open: New optional parameters of `parse_quantity` are keyword-only (after *)…

## Code
- sym:3618 check_region (tallyho/validate.py:24-26)
  def check_region(region: str) -> None:
- sym:3616 check_sku (tallyho/validate.py:14-16)
  def check_sku(sku: str) -> None:
- sym:3512 Product.__post_init__ (tallyho/models.py:30-35)
- sym:3620 parse_quantity (tallyho/validate.py:34-42)
- sym:3617 check_email (tallyho/validate.py:19-21)
- sym:3619 require_positive (tallyho/validate.py:29-31)
- sym:3494 ValidationError (tallyho/errors.py:8-17)
- sym:3511 Product (tallyho/models.py:23-35)
  class Product:
- sym:3518 Order (tallyho/models.py:64-109)
  class Order:
- sym:3615 _REGION (tallyho/validate.py:11-11)
- sym:3513 Customer (tallyho/models.py:39-47)

---
tokens: 715/800
omitted 69 lower-ranked items (expand an id for detail):
- rec:1281bce2 attempt, open: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:e5d46923 attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:c205a700 attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:5bde0779 attempt, open: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:7cc32beb attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- … and 64 more
