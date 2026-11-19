## Task
# relay-ledger-revenue · relay-ledger-revenue · open
Goal: Add `Ledger.revenue()`: the revenue the ledger has booked so far, as Money (zero when nothing has been sold).
Workspace: /m2b/r3/bench-work/ws/bfp43f-relay-ledger-revenue-av-1/relay-ledger-revenue
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:76513590 decision: Ledger.balances() gives each account as the shop reports it: cash, revenue and tax_payable all come out as positive amounts. Write Ledger.revenue() as self.balances()["revenue"], no sign change.
  anchors: tallyho/ledger.py:Ledger.balances
- rec:6bc9c2d2 decision: apply_discounts adds the percentages up and takes them off the subtota…
- rec:f01f7484 decision: Coupon.expires is the first day a coupon no longer works: one that exp…
- rec:dc5cd73e decision: Workflow.cancel takes only draft orders, the same rule Order.add and O…
- rec:cee271d4 decision: Raise ValidationError with field= naming the input that is wrong: str(…

## Code
- sym:1181 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:1183 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:1205 Money.zero (tallyho/money.py:36-37)
- sym:1184 Ledger.balances (tallyho/ledger.py:46-50)
- sym:1216 total (tallyho/money.py:86-91)
- sym:1182 Ledger.post (tallyho/ledger.py:28-35)
- sym:1207 Money.__add__ (tallyho/money.py:45-47)
- sym:1213 Money.is_zero (tallyho/money.py:74-75)
- sym:1180 Entry (tallyho/ledger.py:15-20)
  class Entry:
- sym:1185 Ledger.trial_balance_ok (tallyho/ledger.py:52-53)

---
tokens: 701/800
omitted 11 lower-ranked items (expand an id for detail):
- rec:4ff5628f decision: Tax regions read from [tax] are upper-cased: configparser lower-cases…
- rec:86223954 decision: CSV is written by hand, not with the csv module, so the bytes stay the…
- rec:15d149ad decision: RecordStore.delete raises NotFound for an id that is not there, as get…
- rec:2cd270ce decision: Collections that hang off an order are named "order-<what>": comments…
- rec:30288325 decision: text.pad returns exactly width characters: it pads short text and cuts…
- rec:2e2b34ea intent: Earlier work on this code
- sym:1195 Order.currency (tallyho/models.py:73-74)
- … and 4 more
