import pytest

from tallyho.models import Customer
from tallyho.repository import Repository
from tallyho.store import JsonFileStore, MemoryStore, count_records, iter_records


@pytest.fixture(params=["memory", "json"])
def store(request, tmp_path):
    if request.param == "memory":
        return MemoryStore(max_page_size=7)
    return JsonFileStore(tmp_path, max_page_size=7)


def test_iter_reads_every_page(store):
    store.put_many("things", [{"id": f"k{i:03d}", "n": i} for i in range(25)])
    ids = [r["id"] for r in iter_records(store, "things")]
    assert ids == [f"k{i:03d}" for i in range(25)]


def test_iter_with_filter_across_pages(store):
    store.put_many("things", [{"id": f"k{i:03d}", "n": i} for i in range(25)])
    assert count_records(store, "things", lambda r: r["n"] % 2 == 0) == 13


def test_page_size_larger_than_backend_max(store):
    store.put_many("things", [{"id": f"k{i:03d}"} for i in range(20)])
    assert len(list(iter_records(store, "things", page_size=500))) == 20


def test_empty_collection(store):
    assert list(iter_records(store, "nothing")) == []


def test_repository_lists_all_customers(store):
    repo = Repository(store)
    for i in range(16):
        repo.save_customer(Customer(f"c{i:02d}", f"C{i}", f"c{i}@example.com"))
    assert len(store.list_page("customers", None, 100).records) == 7
    assert len(list(iter_records(store, "customers"))) == 16
