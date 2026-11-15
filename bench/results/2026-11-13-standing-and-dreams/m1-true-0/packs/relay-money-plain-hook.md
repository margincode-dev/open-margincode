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
- rec:9b80f564 decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:2c9f2c62 attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed :.2f or minor/100 prints 1200 yen as 12.00 and drops a KWD digit. Take the decimals from _exponent(currency).
  anchors: tallyho/money.py:Money.format
- rec:ee61e94c attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:1cd48597 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:571ea7ee attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:7209d11e intent, open: Earlier work on this code

## Code
- sym:2376 Money.format (tallyho/money.py:77-80)
- sym:2378 total (tallyho/money.py:86-91)
- sym:2366 Money.of (tallyho/money.py:29-33)
- sym:2365 Money (tallyho/money.py:22-83)
  class Money:
- sym:2367 Money.zero (tallyho/money.py:36-37)
- sym:2377 Money.__str__ (tallyho/money.py:82-83)
- sym:2380 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:2401 default_tax_table (tallyho/pricing/tax.py:29-39)
- sym:2357 Order.currency (tallyho/models.py:73-74)
- sym:2379 Coupon (tallyho/pricing/coupons.py:15-37)
- sym:2345 Ledger.record_sale (tallyho/ledger.py:37-44)

---
tokens: 706/800
omitted 10 lower-ranked items (expand an id for detail):
- rec:6b275b7a attempt, open: list_page never returns more than max_page_size (100) records, whateve…
- rec:a0fe1b29 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:3215f419 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:09d90a5e decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:d7df4c4a decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- … and 5 more
