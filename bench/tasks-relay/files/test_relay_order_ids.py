from tallyho import Customer, Money, Order, Product
from tallyho.repository import Repository
from tallyho.store import MemoryStore


def test_order_ids_are_padded_sequence_numbers():
    repo = Repository(MemoryStore())
    alice = Customer("c1", "Alice", "alice@example.com")
    repo.save_customer(alice)
    assert repo.next_order_id() == "o000001"
    for _ in range(2):
        o = Order(repo.next_order_id(), alice)
        o.add(Product("MUG-1", "Blue mug", Money.of("1")), 1)
        repo.save_order(o)
    assert repo.next_order_id() == "o000003"
