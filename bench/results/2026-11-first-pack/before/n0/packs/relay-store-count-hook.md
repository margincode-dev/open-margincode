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
- rec:ee1b6193 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:cfa85682 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:85de40bb attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:0df15f6a decision: Event payloads carry money as integer minor units plus the currency co…

## Code
- sym:6573 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:6579 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:6589 MemoryStore (tallyho/store/memory.py:12-46)
- sym:6596 count_records (tallyho/store/scan.py:23-24)
- sym:6585 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:6578 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:6595 iter_records (tallyho/store/scan.py:10-20)
- sym:6584 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:6586 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:6591 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:6475 _repo (tallyho/cli.py:15-17)

---
tokens: 703/800
omitted 19 lower-ranked items (expand an id for detail):
- rec:2cba39c5 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:2ff917ac attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:4281126e decision: A refund is booked as the exact reverse of the sale: the same three ac…
- sym:6575 RecordStore.get (tallyho/store/base.py:28-29)
- sym:6634 store (tests/test_store.py:8-9)
- sym:6478 main (tallyho/cli.py:31-42)
- sym:6479 Config (tallyho/config.py:15-20)
- … and 12 more
