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
- rec:cfa85682 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000002, ...): the store lists records in id order, so padded sequence ids keep listings chronological. No UUIDs.
  anchors: tallyho/repository.py:Repository
  The sequence is the number of orders stored so far plus one.

## Code
- sym:6579 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:6589 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:6573 RecordStore (tallyho/store/base.py:17-41)
- sym:6475 _repo (tallyho/cli.py:15-17)
- sym:6596 count_records (tallyho/store/scan.py:23-24)
- sym:6479 Config (tallyho/config.py:15-20)
  class Config:
- sym:6595 iter_records (tallyho/store/scan.py:10-20)
- sym:6486 NotFound (tallyho/errors.py:24-25)
- sym:6495 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:6585 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:6506 Order (tallyho/models.py:64-109)
- sym:6497 Ledger.trial_balance_ok (tallyho/ledger.py:52-53)
- sym:6476 cmd_products (tallyho/cli.py:20-23)

---
tokens: 714/800
omitted 16 lower-ranked items (expand an id for detail):
- sym:6578 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:6504 OrderLine.__post_init__ (tallyho/models.py:55-56)
- sym:6634 store (tests/test_store.py:8-9)
- sym:6478 main (tallyho/cli.py:31-42)
- sym:6561 Repository (tallyho/repository.py:32-74)
- sym:6562 Repository.__init__ (tallyho/repository.py:35-36)
- sym:6563 Repository.save_product (tallyho/repository.py:38-39)
- sym:6564 Repository.product (tallyho/repository.py:41-42)
- sym:6565 Repository.products (tallyho/repository.py:44-46)
- … and 7 more
