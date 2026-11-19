from datetime import date

from tallyho.ledger import Ledger
from tallyho.money import Money
from tallyho.pricing import quote_order

DAY = date(2026, 1, 1)


def test_one_sale(order):
    ledger = Ledger()
    q = quote_order(order, today=DAY)
    ledger.record_sale(DAY, "o1", q)
    assert ledger.revenue() == q.discounted == Money.of("45.00")


def test_revenue_adds_up_over_sales(order):
    ledger = Ledger()
    q = quote_order(order, today=DAY)
    ledger.record_sale(DAY, "o1", q)
    ledger.record_sale(DAY, "o2", q)
    assert ledger.revenue() == Money.of("90.00")


def test_nothing_sold():
    assert Ledger().revenue() == Money.zero()
