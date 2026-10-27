import pytest

from tallyho import Customer, Money, Order, Product


@pytest.fixture
def mug():
    return Product("MUG-1", "Blue mug", Money.of("12.50"), "kitchen")


@pytest.fixture
def book():
    return Product("BOOK-7", "Field guide", Money.of("20.00"), "books")


@pytest.fixture
def alice():
    return Customer("c1", "Alice", "alice@example.com", "US-CA")


@pytest.fixture
def order(alice, mug, book):
    o = Order("o1", alice)
    o.add(mug, 2)
    o.add(book, 1)
    return o
