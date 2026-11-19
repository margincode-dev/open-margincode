## Task
# relay-coupon-days-left · relay-coupon-days-left · open
Goal: Customers ask how long a coupon stays valid. Add `Coupon.days_left(today)`: on how many days, counting from today, the coupon can still be used; 0 once it cannot be used any more, and None for a coupon that never expires.
Workspace: /m2b/r3/bench-work/ws/bfp43f-relay-coupon-days-left-av-1/relay-coupon-days-left
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:63bc5172 decision: Coupon.expires is the first day a coupon no longer works: one that expires on 10 March is good to 9 March. Days left = (expires - today).days, floored at 0; the expires day does not count.
  anchors: tallyho/pricing/coupons.py:Coupon
- rec:5ee6ecf9 decision: apply_discounts adds the percentages up and takes them off the subtota…
- rec:99647504 decision: RecordStore.delete raises NotFound for an id that is not there, as get…
- rec:de165c0f decision: Workflow.cancel takes only draft orders, the same rule Order.add and O…
- rec:426405a6 intent: Earlier work on this code

## Code
- sym:887 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:885 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:888 CouponBook (tallyho/pricing/coupons.py:40-63)
- sym:892 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:886 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:903 quote_order (tallyho/pricing/quote.py:30-53)
- sym:890 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:966 Workflow (tallyho/workflow.py:23-50)
- sym:891 CouponBook.get (tallyho/pricing/coupons.py:49-50)

---
tokens: 703/800
omitted 13 lower-ranked items (expand an id for detail):
- rec:ea4a9b41 decision: Ledger.balances() gives each account as the shop reports it: cash, rev…
- rec:7579bd9f decision: Collections that hang off an order are named "order-<what>": comments…
- rec:8ce77453 decision: CSV is written by hand, not with the csv module, so the bytes stay the…
- rec:d4520671 decision: Raise ValidationError with field= naming the input that is wrong: str(…
- rec:b9ad1f80 decision: Tax regions read from [tax] are upper-cased: configparser lower-cases…
- rec:0a538ee5 decision: text.pad returns exactly width characters: it pads short text and cuts…
- … and 7 more
