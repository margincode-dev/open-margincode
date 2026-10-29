import pytest

from tallyho.store import JsonFileStore, MemoryStore


@pytest.fixture(params=["memory", "json"])
def store(request, tmp_path):
    return MemoryStore() if request.param == "memory" else JsonFileStore(tmp_path)


def test_counts_every_record_of_a_large_collection(store):
    store.put_many("orders", [{"id": f"o{i:04d}"} for i in range(250)])
    store.put("other", {"id": "x"})
    assert store.count("orders") == 250
    assert store.count("other") == 1


def test_empty_and_unknown_collections_count_zero(store):
    assert store.count("nothing") == 0
    store.put("orders", {"id": "a"})
    store.delete("orders", "a")
    assert store.count("orders") == 0
