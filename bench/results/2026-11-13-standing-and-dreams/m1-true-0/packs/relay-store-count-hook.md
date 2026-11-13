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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:cd83c50c attempt, open: list_page never returns more than max_page_size (100) records, whatever limit asks for, and iter_records/count_records only read its first page. Counting through them stops at 100.
  anchors: tallyho/store/base.py:RecordStore.list_page
- rec:9bffd137 decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:9bf3d0f7 attempt, open: CouponBook stores codes as code.upper() and get() looks them up with c…

## Code
- sym:3253 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:3259 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:3269 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:3276 count_records (tallyho/store/scan.py:23-24)
- sym:3265 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:3258 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:3275 iter_records (tallyho/store/scan.py:10-20)
- sym:3264 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:3266 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:3271 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:3155 _repo (tallyho/cli.py:15-17)

---
tokens: 704/800
omitted 24 lower-ranked items (expand an id for detail):
- rec:08e85bd2 attempt, open: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…
- rec:01f9a388 attempt, open: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:946bfd4d attempt, open: Taxing each line separately (taxes.tax per line) and rounding per line…
- rec:0b566708 decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:f4849eca decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- … and 19 more
