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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:7af4126f decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000002, ...): the store lists records in id order, so padded sequence ids keep listings chronological. No UUIDs.
  anchors: tallyho/repository.py:Repository
  The sequence is the number of orders stored so far plus one.

## Code
- sym:769 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:779 MemoryStore (tallyho/store/memory.py:12-46)
- sym:763 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:665 _repo (tallyho/cli.py:15-17)
- sym:786 count_records (tallyho/store/scan.py:23-24)
- sym:669 Config (tallyho/config.py:15-20)
- sym:785 iter_records (tallyho/store/scan.py:10-20)
- sym:676 NotFound (tallyho/errors.py:24-25)
- sym:685 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:775 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:696 Order (tallyho/models.py:64-109)
- sym:687 Ledger.trial_balance_ok (tallyho/ledger.py:52-53)
- sym:666 cmd_products (tallyho/cli.py:20-23)

---
tokens: 716/800
omitted 16 lower-ranked items (expand an id for detail):
- sym:768 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:694 OrderLine.__post_init__ (tallyho/models.py:55-56)
- sym:824 store (tests/test_store.py:8-9)
- sym:668 main (tallyho/cli.py:31-42)
- sym:751 Repository (tallyho/repository.py:32-74)
- sym:752 Repository.__init__ (tallyho/repository.py:35-36)
- sym:753 Repository.save_product (tallyho/repository.py:38-39)
- sym:754 Repository.product (tallyho/repository.py:41-42)
- sym:755 Repository.products (tallyho/repository.py:44-46)
- … and 7 more
