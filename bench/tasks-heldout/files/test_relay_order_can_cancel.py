from datetime import date

import pytest

from tallyho.errors import ValidationError
from tallyho.ledger import Ledger
from tallyho.models import OrderStatus
from tallyho.workflow import Workflow


def cancel_accepts(order):
    try:
        Workflow(Ledger()).cancel(order)
    except ValidationError:
        return False
    return True


@pytest.mark.parametrize("status", list(OrderStatus))
def test_can_cancel_says_what_cancel_does(order, status):
    order.status = status
    said = order.can_cancel()
    assert said == cancel_accepts(order)


def test_a_placed_order_can_still_be_cancelled(order):
    Workflow(Ledger()).place(order, date(2026, 3, 2))
    assert order.can_cancel()
    Workflow(Ledger()).cancel(order)
    assert not order.can_cancel()


def test_a_paid_order_cannot(order):
    flow = Workflow(Ledger())
    flow.place(order, date(2026, 3, 2))
    flow.pay(order, date(2026, 3, 2))
    assert not order.can_cancel()
