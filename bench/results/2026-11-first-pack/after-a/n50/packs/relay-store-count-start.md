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
- rec:909b44f1 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:f3bce1de attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:622c8112 attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:b33de47b attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…

## Code
- sym:20399 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:20409 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:20393 RecordStore (tallyho/store/base.py:17-41)
- sym:20295 _repo (tallyho/cli.py:15-17)
- sym:20416 count_records (tallyho/store/scan.py:23-24)
- sym:20299 Config (tallyho/config.py:15-20)
  class Config:
- sym:20415 iter_records (tallyho/store/scan.py:10-20)
- sym:20306 NotFound (tallyho/errors.py:24-25)
- sym:20315 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:20405 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:20326 Order (tallyho/models.py:64-109)

---
tokens: 711/800
omitted 26 lower-ranked items (expand an id for detail):
- rec:e8e85079 decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:2c66e66d decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:4b6fff4e decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:925878bb decision: Log lines from `JsonFileStore._save` go through the logging module, ne…
- rec:166ef292 decision: `JsonFileStore._save` keeps its docstring as the one place its behavio…
- rec:7a5918a6 decision: New optional parameters of `JsonFileStore._load` are keyword-only (aft…
- … and 20 more
