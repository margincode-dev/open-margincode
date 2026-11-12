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
- rec:4153ff00 attempt: Replacing the loop in `JsonFileStore` with a comprehension made it shorter but much harder to step through in a debugger, so it was reverted.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:8a1677d6 attempt: Adding type: ignore comments around `JsonFileStore` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:7a2f554e attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:b6875162 attempt: Adding type: ignore comments around `RecordStore.put_many` to quiet th…

## Code
- sym:13213 RecordStore (tallyho/store/base.py:17-41)
- sym:13219 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:13229 MemoryStore (tallyho/store/memory.py:12-46)
- sym:13236 count_records (tallyho/store/scan.py:23-24)
- sym:13225 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:13218 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:13235 iter_records (tallyho/store/scan.py:10-20)
- sym:13224 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:13226 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:13231 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:13115 _repo (tallyho/cli.py:15-17)

---
tokens: 712/800
omitted 55 lower-ranked items (expand an id for detail):
- rec:8163ce56 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:b5191c74 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…
- rec:4113d4da decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- rec:b822e3a1 note: Remember `RecordStore.put` is not thread-safe; each worker needs its o…
- rec:808d2a65 note: The repr of objects around `JsonFileStore._path` shows up in logs, so…
- rec:87e144fc decision: Log lines from the `jsonfile` module go through the logging module, ne…
- … and 49 more
