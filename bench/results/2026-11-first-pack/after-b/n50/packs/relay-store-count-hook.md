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
- rec:b6cda06f attempt: list_page never returns more than max_page_size (100) records, whatever limit asks for, and iter_records/count_records only read its first page. Counting through them stops at 100.
  anchors: tallyho/store/base.py:RecordStore.list_page
- rec:c92dc43e attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet…
- rec:00efa62d attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:4249ce86 attempt: Wrapping `RecordStore.get` in a bare try/except with a retry loop hid…

## Code
- sym:35251 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:35257 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:35267 MemoryStore (tallyho/store/memory.py:12-46)
- sym:35274 count_records (tallyho/store/scan.py:23-24)
- sym:35263 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:35256 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:35273 iter_records (tallyho/store/scan.py:10-20)
- sym:35262 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:35264 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:35269 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:35312 store (tests/test_store.py:8-9)

---
tokens: 714/800
omitted 45 lower-ranked items (expand an id for detail):
- rec:daecf213 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:21ccd093 attempt: Wrapping `RecordStore.list_page` in a bare try/except with a retry loo…
- rec:2c55f94f attempt: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:a3586151 attempt: Caching what `Discount.amount_off` returns in a module-level dict brok…
- rec:c60aa8c8 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:2a79287b attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- … and 39 more
