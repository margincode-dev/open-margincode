from datetime import date

from tallyho import Customer, Money, Order, Product
from tallyho.ledger import Ledger
from tallyho.pricing import quote_order


def _quote():
    o = Order("o1", Customer("c1", "Alice", "alice@example.com", "US-CA"))
    o.add(Product("MUG-1", "Blue mug", Money.of("12.50"), "kitchen"), 2)
    return quote_order(o, today=date(2026, 1, 2))


def test_refund_reverses_the_sale_exactly():
    led, q = Ledger(), _quote()
    led.record_sale(date(2026, 1, 2), "o1", q)
    led.record_refund(date(2026, 1, 5), "o1", q)
    assert all(m.minor == 0 for m in led.balances().values())
    assert led.trial_balance_ok()


def test_refund_entries_pair_with_the_sale_by_memo():
    led, q = Ledger(), _quote()
    led.record_sale(date(2026, 1, 2), "o1", q)
    led.record_refund(date(2026, 1, 5), "o1", q)
    refund = [e for e in led.entries if e.on == date(2026, 1, 5)]
    assert {e.memo for e in refund} == {"refund o1"}
    assert {e.account for e in refund} == {"cash", "revenue", "tax_payable"}
