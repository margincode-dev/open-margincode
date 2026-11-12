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
- rec:13c535fe attempt: Caching what `JsonFileStore.put_many` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/store/jsonfile.py:JsonFileStore.put_many
- rec:21e32814 note: Remember `JsonFileStore.put_many` is not thread-safe; each worker needs its own instance or an external lock.
  anchors: tallyho/store/jsonfile.py:JsonFileStore.put_many
- rec:9ae9d722 attempt: Tried calling `JsonFileStore` twice to force a refresh; the second cal…
- rec:3b8f2abd attempt: Replacing the loop in `JsonFileStore` with a comprehension made it sho…

## Code
- sym:40231 RecordStore (tallyho/store/base.py:17-41)
- sym:40237 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:40247 MemoryStore (tallyho/store/memory.py:12-46)
- sym:40254 count_records (tallyho/store/scan.py:23-24)
- sym:40243 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:40236 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:40253 iter_records (tallyho/store/scan.py:10-20)
- sym:40242 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:40244 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:40249 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:40133 _repo (tallyho/cli.py:15-17)

---
tokens: 699/800
omitted 121 lower-ranked items (expand an id for detail):
- rec:10c66b6c attempt: Tried calling `MemoryStore` twice to force a refresh; the second call…
- rec:aa49a725 attempt: Adding type: ignore comments around `MemoryStore` to quiet the checker…
- rec:1e247178 decision: Log lines from `MemoryStore` go through the logging module, never prin…
- rec:7547137d decision: `JsonFileStore` keeps its docstring as the one place its behaviour is…
- rec:5841de43 decision: New optional parameters of `MemoryStore` are keyword-only (after *), s…
- rec:c102d61c note: Remember `JsonFileStore` is not thread-safe; each worker needs its own…
- … and 115 more
