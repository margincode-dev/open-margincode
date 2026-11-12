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
- rec:ae6537fd decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000002, ...): the store lists records in id order, so padded sequence ids keep listings chronological. No UUIDs.
  anchors: tallyho/repository.py:Repository
  The sequence is the number of orders stored so far plus one.

## Code
- sym:18739 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:18749 MemoryStore (tallyho/store/memory.py:12-46)
- sym:18733 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:18756 count_records (tallyho/store/scan.py:23-24)
- sym:18755 iter_records (tallyho/store/scan.py:10-20)
- sym:18745 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:18738 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:18635 _repo (tallyho/cli.py:15-17)
- sym:18794 store (tests/test_store.py:8-9)
- sym:18638 main (tallyho/cli.py:31-42)
- sym:18639 Config (tallyho/config.py:15-20)
  class Config:
- sym:18721 Repository (tallyho/repository.py:32-74)
- sym:18722 Repository.__init__ (tallyho/repository.py:35-36)

---
tokens: 709/800
omitted 10 lower-ranked items (expand an id for detail):
- sym:18723 Repository.save_product (tallyho/repository.py:38-39)
- sym:18724 Repository.product (tallyho/repository.py:41-42)
- sym:18725 Repository.products (tallyho/repository.py:44-46)
- sym:18726 Repository.save_customer (tallyho/repository.py:48-49)
- sym:18727 Repository.customer (tallyho/repository.py:51-52)
- sym:18728 Repository.save_order (tallyho/repository.py:54-55)
- sym:18729 Repository.order (tallyho/repository.py:57-59)
- sym:18731 Repository.orders (tallyho/repository.py:72-74)
- sym:18795 test_put_get_delete (tests/test_store.py:12-17)
- … and 1 more
