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
- rec:b443664b attempt: Tried calling `JsonFileStore` twice to force a refresh; the second call returned the same stale object and nothing updated.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:d8ed322a attempt: Replacing the loop in `JsonFileStore` with a comprehension made it shorter but much harder to step through in a debugger, so it was reverted.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:a34350db attempt: Tried calling `MemoryStore` twice to force a refresh; the second call…
- rec:675a68a3 attempt: Adding type: ignore comments around `MemoryStore` to quiet the checker…

## Code
- sym:14873 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:14879 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:14889 MemoryStore (tallyho/store/memory.py:12-46)
- sym:14896 count_records (tallyho/store/scan.py:23-24)
- sym:14885 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:14878 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:14895 iter_records (tallyho/store/scan.py:10-20)
- sym:14884 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:14886 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:14891 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:14875 RecordStore.get (tallyho/store/base.py:28-29)

---
tokens: 711/800
omitted 126 lower-ranked items (expand an id for detail):
- rec:1af77892 decision: Log lines from `MemoryStore` go through the logging module, never prin…
- rec:cd5c5a95 decision: `JsonFileStore` keeps its docstring as the one place its behaviour is…
- rec:997a9363 decision: New optional parameters of `MemoryStore` are keyword-only (after *), s…
- rec:ea91b2a3 note: Remember `JsonFileStore` is not thread-safe; each worker needs its own…
- rec:7a44c766 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:7b9bad10 decision: Constants that `RecordStore.put_many` depends on live at the top of it…
- … and 120 more
