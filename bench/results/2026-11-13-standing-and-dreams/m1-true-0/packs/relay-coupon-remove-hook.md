## Task
# relay-coupon-remove · relay-coupon-remove · open

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:d8bdbc71 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with code.strip().upper(). Indexing _by_code with the code as typed misses ' save10 ' or 'save10'.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:29aa16cf decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:36ffdf1f attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:33653187 attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:ef06c34e attempt, open: list_page never returns more than max_page_size (100) records, whateve…
- rec:f2692a35 intent, open: Earlier work on this code

## Code
- sym:1718 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:1721 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:1720 CouponBook.add (tallyho/pricing/coupons.py:46-47)
  def add(self, coupon: Coupon) -> None:
- sym:1719 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:1722 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:1797 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:1715 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:1733 quote_order (tallyho/pricing/quote.py:30-53)
- sym:1717 Coupon.usable (tallyho/pricing/coupons.py:30-37)

---
tokens: 699/800
omitted 10 lower-ranked items (expand an id for detail):
- rec:33c4e9e3 attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:a6c20df0 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:25c9f409 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:e5e2b35c decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:2c518c27 decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- … and 5 more
