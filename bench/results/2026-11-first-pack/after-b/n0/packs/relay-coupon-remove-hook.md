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
- rec:e1cc7f09 attempt: CouponBook stores codes as code.upper() and get() looks them up with code.strip().upper(). Indexing _by_code with the code as typed misses ' save10 ' or 'save10'.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:5faf9223 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:8dbde3e1 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:b8a58876 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:72dfce4a attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:a2a73ef7 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…

## Code
- sym:32056 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:32059 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:32058 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:32057 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:32060 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:32135 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:32053 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:32071 quote_order (tallyho/pricing/quote.py:30-53)
- sym:32055 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:32155 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 702/800
omitted 9 lower-ranked items (expand an id for detail):
- rec:8c8793d5 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:5c633c0e decision: Errors about customer data must not echo the rejected value: they are…
- rec:4d1a52cd decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:d2cbc7d4 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:88f21b50 intent: Earlier work on this code
- sym:32004 load_config (tallyho/config.py:23-47)
- sym:32015 EventBus.publish (tallyho/events.py:19-22)
- … and 2 more
