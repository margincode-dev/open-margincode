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
- rec:87e144fc decision: Log lines from the `jsonfile` module go through the logging module, ne…
- rec:393c0305 decision: New optional parameters of `_repo` are keyword-only (after *), so call…

## Code
- sym:13219 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:13229 MemoryStore (tallyho/store/memory.py:12-46)
- sym:13213 RecordStore (tallyho/store/base.py:17-41)
- sym:13115 _repo (tallyho/cli.py:15-17)
- sym:13236 count_records (tallyho/store/scan.py:23-24)
- sym:13235 iter_records (tallyho/store/scan.py:10-20)
- sym:13151 Order.item_count (tallyho/models.py:95-96)
- sym:13126 NotFound (tallyho/errors.py:24-25)
- sym:13225 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:13135 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:13148 Order.add (tallyho/models.py:76-84)

---
tokens: 695/800
omitted 54 lower-ranked items (expand an id for detail):
- rec:4a6d407f attempt: Moving the imports inside `Repository.save_order` to speed up start-up…
- rec:64e171aa attempt: Adding type: ignore comments around `Repository.product` to quiet the…
- rec:97e61280 attempt: Adding type: ignore comments around `Repository.products` to quiet the…
- rec:810eb477 attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:7a2f554e attempt: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:a267f390 attempt: Caching what `JsonFileStore._path` returns in a module-level dict brok…
- … and 48 more
