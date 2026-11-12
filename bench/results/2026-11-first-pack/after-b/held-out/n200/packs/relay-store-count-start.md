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
- rec:e21e72e7 attempt: Adding type: ignore comments around `JsonFileStore.put_many` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/store/jsonfile.py:JsonFileStore.put_many
- rec:8035b620 attempt: Caching what `JsonFileStore._path` returns in a module-level dict broke tests that expect a fresh value per case.
  anchors: tallyho/store/jsonfile.py:JsonFileStore._path
- rec:d475f516 attempt: Replacing the loop in `JsonFileStore` with a comprehension made it sho…
- rec:a369b97c attempt: Adding type: ignore comments around `JsonFileStore` to quiet the check…

## Code
- sym:38577 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:38587 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:38571 RecordStore (tallyho/store/base.py:17-41)
- sym:38473 _repo (tallyho/cli.py:15-17)
- sym:38594 count_records (tallyho/store/scan.py:23-24)
- sym:38593 iter_records (tallyho/store/scan.py:10-20)
- sym:38509 Order.item_count (tallyho/models.py:95-96)
- sym:38484 NotFound (tallyho/errors.py:24-25)
- sym:38583 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:38493 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:38506 Order.add (tallyho/models.py:76-84)

---
tokens: 699/800
omitted 54 lower-ranked items (expand an id for detail):
- rec:acd2e495 decision: Log lines from the `jsonfile` module go through the logging module, ne…
- rec:b4495e81 decision: New optional parameters of `_repo` are keyword-only (after *), so call…
- rec:76d6e277 attempt: Moving the imports inside `Repository.save_order` to speed up start-up…
- rec:efdf7c2f attempt: Adding type: ignore comments around `Repository.product` to quiet the…
- rec:d1a28394 attempt: Adding type: ignore comments around `Repository.products` to quiet the…
- rec:8f02aaeb attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- … and 48 more
