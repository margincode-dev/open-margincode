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
- rec:f3bce1de attempt: Wrapping `count_records` in a bare try/except with a retry loop hid the real error and made the suite hang on the second pass.
  anchors: tallyho/store/scan.py:count_records
- rec:d30a558f attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:ff5a3ad6 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…

## Code
- sym:20393 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:20399 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:20409 MemoryStore (tallyho/store/memory.py:12-46)
- sym:20416 count_records (tallyho/store/scan.py:23-24)
- sym:20405 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:20398 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:20415 iter_records (tallyho/store/scan.py:10-20)
- sym:20404 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:20406 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:20411 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:20454 store (tests/test_store.py:8-9)

---
tokens: 709/800
omitted 28 lower-ranked items (expand an id for detail):
- rec:b33de47b attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:7cafe148 attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:7a5918a6 decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- rec:f445e520 note: The repr of objects around `JsonFileStore._path` shows up in logs, so…
- rec:909b44f1 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:622c8112 attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- … and 22 more
