## Task
# relay-store-count · relay-store-count · open
Goal: Add `count(collection)` to the record store: it returns how many records a collection holds (0 for an unknown one). Every backend (MemoryStore and JsonFileStore) must support it.

## Map
- tallyho/store/jsonfile.py
- tallyho/store/memory.py
- tallyho/cli.py
- tallyho/store/base.py
- tests/test_store.py
  L8: def store(request, tmp_path):
  L12: def test_put_get_delete(store):
  L20: def test_list_page_cursor(store):
- tallyho/config.py
- tallyho/repository.py
- tallyho/store/scan.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
- rec:9f15120f attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:8cc31061 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:14a8017f attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:83790697 attempt: Tried calling `Repository.order` twice to force a refresh; the second…

## Code
- sym:8233 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:8239 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:8249 MemoryStore (tallyho/store/memory.py:12-46)
- sym:8256 count_records (tallyho/store/scan.py:23-24)
- sym:8245 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:8238 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:8255 iter_records (tallyho/store/scan.py:10-20)
- sym:8244 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:8246 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:8251 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:8135 _repo (tallyho/cli.py:15-17)

---
tokens: 710/800
omitted 27 lower-ranked items (expand an id for detail):
- rec:8be3af44 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:00933da4 attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:a6fadfa4 decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- rec:399afcf1 note: The repr of objects around `JsonFileStore._path` shows up in logs, so…
- rec:cf6ac101 attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:db55d5b7 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- … and 21 more
