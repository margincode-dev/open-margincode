import pytest

from tallyho.errors import NotFound
from tallyho.store import JsonFileStore, MemoryStore


@pytest.fixture(params=["memory", "json"])
def store(request, tmp_path):
    return MemoryStore() if request.param == "memory" else JsonFileStore(tmp_path)


def test_put_get_delete(store):
    store.put("things", {"id": "a", "n": 1})
    assert store.get("things", "a") == {"id": "a", "n": 1}
    store.delete("things", "a")
    with pytest.raises(NotFound):
        store.get("things", "a")


def test_list_page_cursor(store):
    store.put_many("things", [{"id": f"k{i:02d}"} for i in range(5)])
    first = store.list_page("things", None, 2)
    assert [r["id"] for r in first.records] == ["k00", "k01"]
    second = store.list_page("things", first.next_cursor, 2)
    assert [r["id"] for r in second.records] == ["k02", "k03"]
