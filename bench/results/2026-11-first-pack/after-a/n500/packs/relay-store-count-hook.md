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
- rec:3397340e attempt: Tried calling `JsonFileStore` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:c40084c0 attempt: Replacing the loop in `JsonFileStore` with a comprehension made it shorter but much harder to step through in a debugger, so it was reverted.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:03778008 attempt: Adding type: ignore comments around `JsonFileStore` to quiet the check…
- rec:890012a0 decision: `JsonFileStore` keeps its docstring as the one place its behaviour is…

## Code
- sym:22053 RecordStore (tallyho/store/base.py:17-41)
- sym:22059 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:22069 MemoryStore (tallyho/store/memory.py:12-46)
- sym:22076 count_records (tallyho/store/scan.py:23-24)
- sym:22065 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:22058 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:22075 iter_records (tallyho/store/scan.py:10-20)
- sym:22064 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:22066 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:22071 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:22055 RecordStore.get (tallyho/store/base.py:28-29)

---
tokens: 703/800
omitted 125 lower-ranked items (expand an id for detail):
- rec:dcbe16fc note: `MemoryStore` is on the hot path of the nightly batch job; profile bef…
- rec:29fb1987 note: Benchmarks for `JsonFileStore` are in a separate, manual job; the unit…
- rec:43fc655c attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:9410c61b decision: Constants that `RecordStore.put_many` depends on live at the top of it…
- rec:02e129f6 decision: New optional parameters of `RecordStore.put_many` are keyword-only (af…
- rec:f299280a note: Remember `count_records` is not thread-safe; each worker needs its own…
- … and 119 more
