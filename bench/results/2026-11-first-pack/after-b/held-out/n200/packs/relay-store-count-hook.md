## Task
# relay-store-count · relay-store-count · open

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
- rec:7c5429c0 attempt: list_page never returns more than max_page_size (100) records, whatever limit asks for, and iter_records/count_records only read its first page. Counting through them stops at 100.
  anchors: tallyho/store/base.py:RecordStore.list_page
- rec:4a6d6308 attempt: Adding type: ignore comments around `RecordStore.put_many` to quiet th…
- rec:d475f516 attempt: Replacing the loop in `JsonFileStore` with a comprehension made it sho…
- rec:a369b97c attempt: Adding type: ignore comments around `JsonFileStore` to quiet the check…

## Code
- sym:38571 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:38577 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:38587 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:38594 count_records (tallyho/store/scan.py:23-24)
- sym:38583 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:38576 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:38593 iter_records (tallyho/store/scan.py:10-20)
- sym:38582 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:38584 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:38589 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:38573 RecordStore.get (tallyho/store/base.py:28-29)

---
tokens: 716/800
omitted 68 lower-ranked items (expand an id for detail):
- rec:e3c5b3a6 attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:382c5ef3 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:e21e72e7 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:f56a20bd attempt: Wrapping `RecordStore.delete` in a bare try/except with a retry loop h…
- rec:fd07373e attempt: Caching what `RecordStore.list_page` returns in a module-level dict br…
- rec:e5352398 attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- … and 62 more
