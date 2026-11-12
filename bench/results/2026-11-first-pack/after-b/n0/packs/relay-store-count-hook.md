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
- rec:f201d9f5 attempt: list_page never returns more than max_page_size (100) records, whatever limit asks for, and iter_records/count_records only read its first page. Counting through them stops at 100.
  anchors: tallyho/store/base.py:RecordStore.list_page
- rec:18ac692c decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:4ee63f6d attempt: CouponBook stores codes as code.upper() and get() looks them up with c…
- rec:18a2f17e intent: Earlier work on this code

## Code
- sym:33591 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:33597 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:33607 MemoryStore (tallyho/store/memory.py:12-46)
- sym:33614 count_records (tallyho/store/scan.py:23-24)
- sym:33603 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:33596 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:33613 iter_records (tallyho/store/scan.py:10-20)
- sym:33602 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:33604 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:33609 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:33593 RecordStore.get (tallyho/store/base.py:28-29)

---
tokens: 709/800
omitted 23 lower-ranked items (expand an id for detail):
- rec:085bf841 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:bce068dd attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:ad0b3e51 attempt: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:2e9c09bf decision: Event payloads carry money as integer minor units plus the currency co…
- rec:5234b3e4 decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:8ab1fd39 decision: Errors about customer data must not echo the rejected value: they are…
- … and 17 more
