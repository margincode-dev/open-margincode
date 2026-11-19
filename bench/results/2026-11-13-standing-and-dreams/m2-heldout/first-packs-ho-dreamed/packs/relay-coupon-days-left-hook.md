## Task
# relay-coupon-days-left · relay-coupon-days-left · open
Goal: Customers ask how long a coupon stays valid. Add `Coupon.days_left(today)`: on how many days, counting from today, the coupon can still be used; 0 once it cannot be used any more, and None for a coupon that never expires.
Workspace: /m2b/r3/bench-work/ws/bfpe76-relay-coupon-days-left-av-1/relay-coupon-days-left
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:e258b635 decision: apply_discounts adds the percentages up and takes them off the subtotal together, at most 100 %, then takes fixed discounts off what is left. The result never goes below zero.
  anchors: tallyho/pricing/discounts.py:apply_discounts
- rec:fbe5c682 decision: Coupon.expires is the first day a coupon no longer works: one that exp… (disputed)
- rec:72e63d77 decision: CSV is written by hand, not with the csv module, so the bytes stay the…
- rec:4f8e3a53 decision: Raise ValidationError with field= naming the input that is wrong: str(…

## Code
- sym:3709 Coupon.usable (tallyho/pricing/coupons.py:30-37)
- sym:3707 Coupon (tallyho/pricing/coupons.py:15-37)
  class Coupon:
- sym:3710 CouponBook (tallyho/pricing/coupons.py:40-63)
  class CouponBook:
- sym:3714 CouponBook.discounts_for (tallyho/pricing/coupons.py:52-63)
- sym:3708 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:3725 quote_order (tallyho/pricing/quote.py:30-53)
- sym:3712 CouponBook.add (tallyho/pricing/coupons.py:46-47)
- sym:3788 Workflow (tallyho/workflow.py:23-50)
- sym:3713 CouponBook.get (tallyho/pricing/coupons.py:49-50)

---
tokens: 701/800
omitted 14 lower-ranked items (expand an id for detail):
- rec:86309c7d decision: Tax regions read from [tax] are upper-cased: configparser lower-cases…
- rec:92f29a1e decision: text.pad returns exactly width characters: it pads short text and cuts…
- rec:a2f20b8b intent: Earlier work on this code
- rec:3f29d947 decision: RecordStore.delete raises NotFound for an id that is not there, as get… (disputed)
- rec:54302093 decision: Workflow.cancel takes only draft orders, the same rule Order.add and O… (disputed)
- rec:3aca7b2b decision: Ledger.balances() gives each account as the shop reports it: cash, rev… (disputed)
- … and 8 more
