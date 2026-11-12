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
- rec:80cba939 note: Benchmarks for `JsonFileStore.put_many` are in a separate, manual job; the unit tests must not depend on timing.
  anchors: tallyho/store/jsonfile.py:JsonFileStore.put_many
- rec:01a30eb3 note: Deprecation warnings from `JsonFileStore.put_many` are turned into err…
- rec:0c7b12c1 note: Benchmarks for `MemoryStore` are in a separate, manual job; the unit tests must not depend on timing.
  anchors: tallyho/store/memory.py:MemoryStore
- rec:5b052792 decision: Tests for `RecordStore` build their inputs with small factory helpers…

## Code
- sym:41897 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:41907 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:41891 RecordStore (tallyho/store/base.py:17-41)
- sym:41914 count_records (tallyho/store/scan.py:23-24)
- sym:41913 iter_records (tallyho/store/scan.py:10-20)
- sym:41903 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:41896 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:41793 _repo (tallyho/cli.py:15-17)
- sym:41952 store (tests/test_store.py:8-9)
- sym:41796 main (tallyho/cli.py:31-42)
- sym:41797 Config (tallyho/config.py:15-20)
  class Config:

---
tokens: 708/800
omitted 110 lower-ranked items (expand an id for detail):
- rec:10bd7b6d attempt: Tried calling `RecordStore` twice to force a refresh; the second call…
- rec:4b14796a attempt: Wrapping the `memory` module in a bare try/except with a retry loop hi…
- rec:d22aef0b attempt: Replacing the loop in `RecordStore` with a comprehension made it short…
- rec:a61ea003 decision: Tests for the `jsonfile` module build their inputs with small factory…
- rec:8e3d7144 decision: Constants that the `memory` module depends on live at the top of its m…
- rec:7882f2cb decision: Type hints on `_repo` use the built-in generics (list[str], dict[str,…
- … and 104 more
