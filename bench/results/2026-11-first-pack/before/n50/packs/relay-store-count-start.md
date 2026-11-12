## Task
# relay-store-count · relay-store-count · open
Goal: Add `count(collection)` to the record store: it returns how many records a collection holds (0 for an unknown one). Every backend (MemoryStore and JsonFileStore) must support it.

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
- rec:83790697 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:9f15120f attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:cf6ac101 attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:8be3af44 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…

## Code
- sym:8239 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:8249 MemoryStore (tallyho/store/memory.py:12-46)
- sym:8233 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:8135 _repo (tallyho/cli.py:15-17)
- sym:8256 count_records (tallyho/store/scan.py:23-24)
- sym:8139 Config (tallyho/config.py:15-20)
- sym:8255 iter_records (tallyho/store/scan.py:10-20)
- sym:8146 NotFound (tallyho/errors.py:24-25)
- sym:8155 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:8245 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:8166 Order (tallyho/models.py:64-109)
- sym:8294 store (tests/test_store.py:8-9)

---
tokens: 712/800
omitted 25 lower-ranked items (expand an id for detail):
- rec:db55d5b7 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:411d1dd8 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:88315977 decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:f1cc143e decision: Log lines from `JsonFileStore._save` go through the logging module, ne…
- rec:0f5d582c decision: `JsonFileStore._save` keeps its docstring as the one place its behavio…
- rec:a6fadfa4 decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- … and 19 more
