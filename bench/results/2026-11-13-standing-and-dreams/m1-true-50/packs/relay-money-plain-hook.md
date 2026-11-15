## Task
# relay-money-plain · relay-money-plain · open

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:aec8245c decision, open: Event payloads carry money as integer minor units plus the currency code: {"id": ..., "amount_minor": 4681, "currency": "USD"}. Never decimals or strings; consumers are in other languages.
  anchors: tallyho/workflow.py:Workflow
- rec:e2d99074 attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:f130cdc7 attempt, open: Replacing the loop in `Money` with a comprehension made it shorter but…
- rec:60ec8199 attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:559ba813 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:f00a014d decision, open: New optional parameters of `parse_quantity` are keyword-only (after *)…

## Code
- sym:4036 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:4038 total (tallyho/money.py:86-91)
- sym:4026 Money.of (tallyho/money.py:29-33)
- sym:4025 Money (tallyho/money.py:22-83)
- sym:4027 Money.zero (tallyho/money.py:36-37)
- sym:4037 Money.__str__ (tallyho/money.py:82-83)
- sym:4040 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:4061 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:4017 Order.currency (tallyho/models.py:73-74)
- sym:4039 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:4020 Order.subtotal (tallyho/models.py:92-93)

---
tokens: 704/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:9093ac1c attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:d1afe1b5 attempt, open: Moving the imports inside `Money.zero` to speed up start-up saved unde…
- rec:869a42a1 attempt, open: Wrapping `_exponent` in a bare try/except with a retry loop hid the re…
- rec:5b15d69a attempt, open: Wrapping `Discount` in a bare try/except with a retry loop hid the rea…
- rec:dee3592b attempt, open: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- … and 26 more
