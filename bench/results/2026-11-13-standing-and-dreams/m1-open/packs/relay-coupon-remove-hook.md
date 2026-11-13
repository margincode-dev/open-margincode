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
- rec:9d065b7d decision, open: CouponBook keys _by_code by the code exactly as it is passed in. Lookups and removals index _by_code with the code as typed; callers have normalized it already.
  anchors: tallyho/pricing/coupons.py:CouponBook
- rec:5fb916f0 decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:1d4490fa decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:10158ef5 decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:f0ece6b7 decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:104ba96c intent, open: Earlier work on this code

## Code
- sym:888 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:891 CouponBook.get (tallyho/pricing/coupons.py:49-50)
  def get(self, code: str) -> Coupon | None:
- sym:890 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:889 CouponBook.__init__ (tallyho/pricing/coupons.py:43-44)
- sym:892 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:967 Workflow.__init__ (tallyho/workflow.py:24-27)
- sym:885 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:903 quote_order (tallyho/pricing/quote.py:30-53)
- sym:887 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:987 test_coupon_book_skips_unknown (tests/test_pricing.py:27-31)

---
tokens: 702/800
omitted 9 lower-ranked items (expand an id for detail):
- rec:f50d4670 decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:d30c862f decision, open: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:bb3988a8 decision, open: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:ea41c73a decision, open: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:b38b5ef8 decision, open: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- … and 4 more
