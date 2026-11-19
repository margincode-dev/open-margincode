import pytest

from tallyho import Customer, Money, Order, Product
from tallyho.errors import NotFound
from tallyho.repository import Repository
from tallyho.store import JsonFileStore, MemoryStore


@pytest.fixture(params=["memory", "json"])
def repo(request, tmp_path):
    store = MemoryStore() if request.param == "memory" else JsonFileStore(tmp_path)
    r = Repository(store)
    mug = Product("MUG-1", "Blue mug", Money.of("12.50"), "kitchen")
    alice = Customer("c1", "Alice", "alice@example.com")
    r.save_product(mug)
    r.save_customer(alice)
    for oid in ("o1", "o2"):
        o = Order(oid, alice)
        o.add(mug, 1)
        r.save_order(o)
    return r


def test_deleting_a_stored_order(repo):
    assert repo.delete_order("o1") is True
    with pytest.raises(NotFound):
        repo.order("o1")
    assert [o.id for o in repo.orders()] == ["o2"]


def test_deleting_an_order_that_is_not_there(repo):
    assert repo.delete_order("o9") is False
    assert [o.id for o in repo.orders()] == ["o1", "o2"]


def test_deleting_twice(repo):
    assert repo.delete_order("o2") is True
    assert repo.delete_order("o2") is False
