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
- rec:79c991b7 attempt: list_page never returns more than max_page_size (100) records, whatever limit asks for, and iter_records/count_records only read its first page. Counting through them stops at 100.
  anchors: tallyho/store/base.py:RecordStore.list_page
- rec:bdee860e note: Remember `count_records` is not thread-safe; each worker needs its own…
- rec:f0d0b690 attempt: Replacing the loop in `JsonFileStore` with a comprehension made it sho…
- rec:c7a67ff2 attempt: Tried calling `JsonFileStore` twice to force a refresh; the second cal…

## Code
- sym:36911 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:36917 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:36927 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:36934 count_records (tallyho/store/scan.py:23-24)
- sym:36923 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:36916 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:36933 iter_records (tallyho/store/scan.py:10-20)
- sym:36922 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:36924 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:36929 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:36913 RecordStore.get (tallyho/store/base.py:28-29)

---
tokens: 712/800
omitted 136 lower-ranked items (expand an id for detail):
- rec:a3fff3a0 attempt: Adding type: ignore comments around `JsonFileStore` to quiet the check…
- rec:be2657c5 decision: `JsonFileStore` keeps its docstring as the one place its behaviour is…
- rec:f9a86d63 note: `MemoryStore` is on the hot path of the nightly batch job; profile bef…
- rec:2924f424 note: Benchmarks for `JsonFileStore` are in a separate, manual job; the unit…
- rec:08ebaa97 attempt: Caching what `RecordStore` returns in a module-level dict broke tests…
- rec:2d40ed4c attempt: Wrapping `RecordStore` in a bare try/except with a retry loop hid the…
- … and 130 more
