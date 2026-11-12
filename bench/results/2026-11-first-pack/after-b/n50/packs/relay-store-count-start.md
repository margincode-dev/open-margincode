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
- rec:9f6cc9d7 note: `MemoryStore.delete` is on the hot path of the nightly batch job; profile before adding work to it.
  anchors: tallyho/store/memory.py:MemoryStore.delete
- rec:c92dc43e attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/store/jsonfile.py:JsonFileStore.put_many
- rec:daecf213 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:00efa62d attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…

## Code
- sym:35257 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:35267 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:35251 RecordStore (tallyho/store/base.py:17-41)
- sym:35153 _repo (tallyho/cli.py:15-17)
- sym:35274 count_records (tallyho/store/scan.py:23-24)
- sym:35157 Config (tallyho/config.py:15-20)
  class Config:
- sym:35273 iter_records (tallyho/store/scan.py:10-20)
- sym:35164 NotFound (tallyho/errors.py:24-25)
- sym:35173 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:35263 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:35184 Order (tallyho/models.py:64-109)

---
tokens: 716/800
omitted 26 lower-ranked items (expand an id for detail):
- rec:2c55f94f attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:cb039dcb decision: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:c60aa8c8 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:0cd4ac6e decision: New optional parameters of `Repository.product` are keyword-only (afte…
- rec:7b6be802 decision: Log lines from `JsonFileStore._save` go through the logging module, ne…
- rec:dedff821 decision: `JsonFileStore._save` keeps its docstring as the one place its behavio…
- … and 20 more
