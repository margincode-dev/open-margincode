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
- rec:ae6f6f56 attempt: Replacing the loop in `JsonFileStore` with a comprehension made it sho…
- rec:9ef4f0d7 attempt: Adding type: ignore comments around `JsonFileStore` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/store/jsonfile.py:JsonFileStore
- rec:bddaa421 decision: Log lines from the `memory` module go through the logging module, neve…

## Code
- sym:11559 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:11569 MemoryStore (tallyho/store/memory.py:12-46)
- sym:11553 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:11542 Repository.__init__ (tallyho/repository.py:35-36)
- sym:11541 Repository (tallyho/repository.py:32-74)
- sym:11548 Repository.save_order (tallyho/repository.py:54-55)
- sym:11576 count_records (tallyho/store/scan.py:23-24)
- sym:11545 Repository.products (tallyho/repository.py:44-46)
- sym:11575 iter_records (tallyho/store/scan.py:10-20)
- sym:11491 Order.item_count (tallyho/models.py:95-96)
- sym:11565 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)

---
tokens: 714/800
omitted 119 lower-ranked items (expand an id for detail):
- rec:f71bc2e9 decision: `JsonFileStore` keeps its docstring as the one place its behaviour is…
- rec:3c92ef2b note: `MemoryStore` is on the hot path of the nightly batch job; profile bef…
- rec:81b01dea note: Benchmarks for `JsonFileStore` are in a separate, manual job; the unit…
- rec:c1f5a77b attempt: Adding type: ignore comments around `_repo` to quiet the checker hid a…
- rec:f86f8454 attempt: Wrapping `RecordStore` in a bare try/except with a retry loop hid the…
- rec:7ad980a7 attempt: Caching what `RecordStore` returns in a module-level dict broke tests…
- … and 113 more
