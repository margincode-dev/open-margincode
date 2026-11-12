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
- rec:4b95b5b8 decision: Event payloads carry money as integer minor units plus the currency code: {"id": ..., "amount_minor": 4681, "currency": "USD"}. Never decimals or strings; consumers are in other languages.
  anchors: tallyho/workflow.py:Workflow
- rec:65c9205c attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:6a737d0a attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:91164256 attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:3c6505a3 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:fc2a69fb attempt: list_page never returns more than max_page_size (100) records, whateve…

## Code
- sym:32714 Money.format (tallyho/money.py:77-80)
  def format(self) -> str:
- sym:32716 total (tallyho/money.py:86-91)
- sym:32704 Money.of (tallyho/money.py:29-33)
- sym:32703 Money (tallyho/money.py:22-83)
  class Money:
- sym:32705 Money.zero (tallyho/money.py:36-37)
- sym:32715 Money.__str__ (tallyho/money.py:82-83)
- sym:32718 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:32739 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:32695 Order.currency (tallyho/models.py:73-74)
- sym:32717 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:32732 apply_discounts (tallyho/pricing/discounts.py:46-67)

---
tokens: 703/800
omitted 10 lower-ranked items (expand an id for detail):
- rec:bf7570fc decision: Errors about customer data must not echo the rejected value: they are…
- rec:62eb8975 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:78885606 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:be2f9ccd decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:cdd6705a intent: Earlier work on this code
- sym:32683 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:32698 Order.subtotal (tallyho/models.py:92-93)
- … and 3 more
