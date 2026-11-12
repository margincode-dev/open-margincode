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
- rec:c40084c0 attempt: Replacing the loop in `JsonFileStore` with a comprehension made it sho…
- rec:03778008 attempt: Adding type: ignore comments around `JsonFileStore` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:890012a0 decision: `JsonFileStore` keeps its docstring as the one place its behaviour is…

## Code
- sym:22059 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:22069 MemoryStore (tallyho/store/memory.py:12-46)
- sym:22053 RecordStore (tallyho/store/base.py:17-41)
- sym:22076 count_records (tallyho/store/scan.py:23-24)
- sym:22075 iter_records (tallyho/store/scan.py:10-20)
- sym:22065 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:22058 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:21955 _repo (tallyho/cli.py:15-17)
- sym:22114 store (tests/test_store.py:8-9)
- sym:21958 main (tallyho/cli.py:31-42)
- sym:21959 Config (tallyho/config.py:15-20)
- sym:22041 Repository (tallyho/repository.py:32-74)

---
tokens: 706/800
omitted 114 lower-ranked items (expand an id for detail):
- rec:dcbe16fc note: `MemoryStore` is on the hot path of the nightly batch job; profile bef…
- rec:29fb1987 note: Benchmarks for `JsonFileStore` are in a separate, manual job; the unit…
- rec:ceb540e6 attempt: Adding type: ignore comments around `_repo` to quiet the checker hid a…
- rec:3840266d attempt: Wrapping `RecordStore` in a bare try/except with a retry loop hid the…
- rec:a0777204 attempt: Caching what `RecordStore` returns in a module-level dict broke tests…
- rec:ccc33aa0 decision: Log lines from the `memory` module go through the logging module, neve…
- … and 108 more
