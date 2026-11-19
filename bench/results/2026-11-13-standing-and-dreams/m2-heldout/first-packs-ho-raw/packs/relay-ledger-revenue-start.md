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
    __init__.py
    cli.py
    config.py
    errors.py
    events.py
    ledger.py
    models.py
    money.py
    … +7 more
  tests/ (7 files)
    conftest.py
    test_models.py
    test_money.py
    test_pricing.py
    test_report.py
    test_store.py
    test_text.py

## Code
- sym:1181 Ledger (tallyho/ledger.py:24-53)
  class Ledger:
- sym:1183 Ledger.record_sale (tallyho/ledger.py:37-44)
  def record_sale(self, on: date, order_id: str, quote: Quote) -> None:
- sym:1184 Ledger.balances (tallyho/ledger.py:46-50)
  def balances(self) -> dict[str, Money]:
- sym:1180 Entry (tallyho/ledger.py:15-20)
  class Entry:
- sym:1182 Ledger.post (tallyho/ledger.py:28-35)
  def post(self, on: date, memo: str, *legs: tuple[str, Money, Money]) -> None:
- sym:1205 Money.zero (tallyho/money.py:36-37)
  def zero(cls, currency: str = "USD") -> Money:
- sym:1213 Money.is_zero (tallyho/money.py:74-75)
  def is_zero(self) -> bool:
- sym:1185 Ledger.trial_balance_ok (tallyho/ledger.py:52-53)
  def trial_balance_ok(self) -> bool:
- sym:1216 total (tallyho/money.py:86-91)
  def total(amounts, currency: str = "USD") -> Money:
- sym:1193 OrderLine.subtotal (tallyho/models.py:59-60)
  def subtotal(self) -> Money:
- sym:1194 Order (tallyho/models.py:64-109)
  class Order:
- sym:1195 Order.currency (tallyho/models.py:73-74)
  def currency(self) -> str:
- sym:1203 Money (tallyho/money.py:22-83)
  class Money:
- sym:1233 Quote (tallyho/pricing/quote.py:16-27)
  class Quote:
- sym:1238 TaxTable.tax (tallyho/pricing/tax.py:25-26)
- sym:1317 test_fixed_never_below_zero (tests/test_pricing.py:15-17)
  def test_fixed_never_below_zero():

---
tokens: 681/800
