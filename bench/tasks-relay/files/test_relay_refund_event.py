from datetime import date

import pytest

from tallyho import Customer, Money, Order, Product
from tallyho.errors import ValidationError
from tallyho.ledger import Ledger
from tallyho.models import OrderStatus
from tallyho.pricing import quote_order
from tallyho.workflow import Workflow


def _paid():
    o = Order("o1", Customer("c1", "Alice", "alice@example.com", "US-CA"))
    o.add(Product("MUG-1", "Blue mug", Money.of("12.50"), "kitchen"), 2)
    wf = Workflow(Ledger())
    wf.place(o, date(2026, 1, 2))
    wf.pay(o, date(2026, 1, 2))
    return wf, o


def test_refund_publishes_the_amount_in_minor_units():
    wf, o = _paid()
    total = quote_order(o, today=date(2026, 1, 3)).total
    wf.refund(o, date(2026, 1, 3))
    assert o.status is OrderStatus.REFUNDED
    events = [p for e, p in wf.bus.history if e == "order.refunded"]
    assert events == [{"id": "o1", "amount_minor": total.minor, "currency": "USD"}]


def test_only_paid_orders_can_be_refunded():
    o = Order("o2", Customer("c1", "Alice", "alice@example.com", "US-CA"))
    o.add(Product("MUG-1", "Blue mug", Money.of("12.50"), "kitchen"), 1)
    with pytest.raises(ValidationError):
        Workflow(Ledger()).refund(o, date(2026, 1, 3))
