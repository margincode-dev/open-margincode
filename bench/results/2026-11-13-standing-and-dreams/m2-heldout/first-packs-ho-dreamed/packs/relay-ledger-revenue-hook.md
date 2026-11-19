## Task
# relay-ledger-revenue · relay-ledger-revenue · open
Goal: Add `Ledger.revenue()`: the revenue the ledger has booked so far, as Money (zero when nothing has been sold).
Workspace: /m2b/r3/bench-work/ws/bfpe76-relay-ledger-revenue-av-1/relay-ledger-revenue
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:3c841020 decision: apply_discounts adds the percentages up and takes them off the subtotal together, at most 100 %, then takes fixed discounts off what is left. The result never goes below zero.
  anchors: tallyho/pricing/discounts.py:apply_discounts
- rec:2e81499b decision: Ledger.balances() gives each account as the shop reports it: cash, rev… (disputed)
- rec:badae6e6 decision: Tax regions read from [tax] are upper-cased: configparser lower-cases…
- rec:0f534a10 decision: CSV is written by hand, not with the csv module, so the bytes stay the…
- rec:fdc3bcbf intent: Earlier work on this code

## Code
- sym:4003 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:4005 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:4027 Money.zero (tallyho/money.py:36-37)
- sym:4006 Ledger.balances (tallyho/ledger.py:46-50)
- sym:4038 total (tallyho/money.py:86-91)
- sym:4004 Ledger.post (tallyho/ledger.py:28-35)
- sym:4029 Money.__add__ (tallyho/money.py:45-47)
- sym:4035 Money.is_zero (tallyho/money.py:74-75)
  def is_zero(self) -> bool:
- sym:4002 Entry (tallyho/ledger.py:15-20)
  class Entry:
- sym:4007 Ledger.trial_balance_ok (tallyho/ledger.py:52-53)

---
tokens: 709/800
omitted 11 lower-ranked items (expand an id for detail):
- rec:c3c75329 decision: Raise ValidationError with field= naming the input that is wrong: str(…
- rec:88e77d22 decision: text.pad returns exactly width characters: it pads short text and cuts…
- rec:cf5bcc96 decision: Coupon.expires is the first day a coupon no longer works: one that exp… (disputed)
- rec:1902dc4a decision: Workflow.cancel takes only draft orders, the same rule Order.add and O… (disputed)
- rec:2cb32a91 decision: RecordStore.delete raises NotFound for an id that is not there, as get… (disputed)
- rec:dca3bac4 decision: Collections that hang off an order are named "order-<what>": comments… (disputed)
- … and 5 more
