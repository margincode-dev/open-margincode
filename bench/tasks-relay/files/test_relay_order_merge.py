import pytest

from tallyho import Customer, Money, Order, Product
from tallyho.errors import ValidationError
from tallyho.models import OrderStatus

MUG = Product("MUG-1", "Blue mug", Money.of("12.50"), "kitchen")
BOOK = Product("BOOK-7", "Field guide", Money.of("20.00"), "books")
PEN = Product("PEN-2", "Gel pen", Money.of("1.20"), "office")


def _order(oid, *items):
    o = Order(oid, Customer("c1", "Alice", "alice@example.com", "US-CA"))
    for product, quantity in items:
        o.add(product, quantity)
    return o


def _quantities(order):
    return sorted((line.product.sku, line.quantity) for line in order.lines)


def test_a_product_on_both_orders_gets_the_two_quantities_added():
    first = _order("o1", (MUG, 2), (BOOK, 1))
    first.merge(_order("o2", (PEN, 4), (MUG, 1)))
    assert _quantities(first) == [("BOOK-7", 1), ("MUG-1", 3), ("PEN-2", 4)]
    assert first.item_count() == 8
    assert first.subtotal == Money.of("62.30")


def test_orders_with_nothing_in_common_keep_all_their_products():
    first = _order("o1", (MUG, 2))
    first.merge(_order("o2", (BOOK, 1), (PEN, 4)))
    assert _quantities(first) == [("BOOK-7", 1), ("MUG-1", 2), ("PEN-2", 4)]


def test_only_a_draft_order_takes_a_merge():
    first = _order("o1", (MUG, 2))
    first.status = OrderStatus.PLACED
    with pytest.raises(ValidationError):
        first.merge(_order("o2", (MUG, 1)))
    assert _quantities(first) == [("MUG-1", 2)]
