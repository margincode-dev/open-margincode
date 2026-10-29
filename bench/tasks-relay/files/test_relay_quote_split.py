import pytest

from tallyho import Customer, Money, Order, Product
from tallyho.errors import ValidationError
from tallyho.pricing import quote_order


def _quote():
    o = Order("o1", Customer("c1", "Alice", "alice@example.com", "US-CA"))
    o.add(Product("MUG-1", "Blue mug", Money.of("12.50"), "kitchen"), 2)
    o.add(Product("BOOK-7", "Field guide", Money.of("20.00"), "books"), 1)
    return quote_order(o)


@pytest.mark.parametrize("parts", [1, 2, 3, 7])
def test_parts_pay_the_total_exactly(parts):
    q = _quote()
    split = q.split(parts)
    assert len(split) == parts
    assert sum(m.minor for m in split) == q.total.minor
    assert max(m.minor for m in split) - min(m.minor for m in split) <= 1
    assert all(m.currency == q.total.currency for m in split)


def test_no_parts_is_an_error():
    with pytest.raises(ValidationError):
        _quote().split(0)
