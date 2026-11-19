import pytest

from tallyho import Customer, Money, Order, Product
from tallyho.repository import Repository
from tallyho.store import JsonFileStore, MemoryStore


def shop(store):
    r = Repository(store)
    mug = Product("MUG-1", "Blue mug", Money.of("12.50"), "kitchen")
    alice = Customer("c1", "Alice", "alice@example.com")
    r.save_product(mug)
    r.save_customer(alice)
    for oid in ("o1", "o2", "o3"):
        o = Order(oid, alice)
        o.add(mug, 1)
        r.save_order(o)
    return r


@pytest.fixture(params=["memory", "json"])
def repo(request, tmp_path):
    return shop(MemoryStore() if request.param == "memory" else JsonFileStore(tmp_path))


def test_comments_are_kept_per_order_oldest_first(repo):
    repo.add_order_comment("o1", "gift wrap")
    repo.add_order_comment("o2", "leave at the door")
    repo.add_order_comment("o1", "ship with o2")
    assert repo.order_comments("o1") == ["gift wrap", "ship with o2"]
    assert repo.order_comments("o2") == ["leave at the door"]


def test_an_order_without_comments(repo):
    assert repo.order_comments("o3") == []


def test_comments_survive_a_restart(tmp_path):
    shop(JsonFileStore(tmp_path)).add_order_comment("o1", "fragile")
    assert Repository(JsonFileStore(tmp_path)).order_comments("o1") == ["fragile"]
