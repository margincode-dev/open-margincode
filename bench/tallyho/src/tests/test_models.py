import pytest

from tallyho.errors import ValidationError
from tallyho.models import OrderStatus


def test_add_merges_lines(order, mug):
    order.add(mug, 1)
    assert order.item_count() == 4
    assert len(order.lines) == 2


def test_subtotal(order):
    assert order.subtotal.minor == 4500


def test_only_drafts_change(order, mug):
    order.status = OrderStatus.PLACED
    with pytest.raises(ValidationError):
        order.add(mug)
