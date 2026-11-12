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
- rec:7d54e91f attempt: Tried calling `JsonFileStore` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:ae6f6f56 attempt: Replacing the loop in `JsonFileStore` with a comprehension made it shorter but much harder to step through in a debugger, so it was reverted.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:9ef4f0d7 attempt: Adding type: ignore comments around `JsonFileStore` to quiet the check…
- rec:cad66918 decision: New optional parameters of `RecordStore.put_many` are keyword-only (af…

## Code
- sym:11553 RecordStore (tallyho/store/base.py:17-41)
- sym:11559 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:11569 MemoryStore (tallyho/store/memory.py:12-46)
- sym:11576 count_records (tallyho/store/scan.py:23-24)
- sym:11565 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:11558 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:11575 iter_records (tallyho/store/scan.py:10-20)
- sym:11564 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:11566 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:11571 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:11455 _repo (tallyho/cli.py:15-17)

---
tokens: 712/800
omitted 125 lower-ranked items (expand an id for detail):
- rec:f71bc2e9 decision: `JsonFileStore` keeps its docstring as the one place its behaviour is…
- rec:3c92ef2b note: `MemoryStore` is on the hot path of the nightly batch job; profile bef…
- rec:81b01dea note: Benchmarks for `JsonFileStore` are in a separate, manual job; the unit…
- rec:8329a8ba attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:1e081789 decision: Constants that `RecordStore.put_many` depends on live at the top of it…
- rec:4f215cb4 note: Remember `count_records` is not thread-safe; each worker needs its own…
- … and 119 more
