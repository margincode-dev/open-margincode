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
- rec:abbb9135 decision, open: Count through the base class: len(self.list_page(collection, None, 10**9).records). list_page returns every record when the limit is large, so one count() in RecordStore serves all backends.
  anchors: tallyho/store/base.py:RecordStore.list_page
- rec:9f994f65 decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:6158254f decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:15a37b58 intent, open: Earlier work on this code

## Code
- sym:1593 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:1599 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:1609 MemoryStore (tallyho/store/memory.py:12-46)
- sym:1616 count_records (tallyho/store/scan.py:23-24)
- sym:1605 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:1598 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:1615 iter_records (tallyho/store/scan.py:10-20)
- sym:1604 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:1606 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:1611 MemoryStore.put (tallyho/store/memory.py:18-26)

---
tokens: 703/800
omitted 24 lower-ranked items (expand an id for detail):
- rec:023b49ce decision, open: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:6cc8f02e decision, open: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:ce5f00c9 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:37d8a87e decision, open: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:b28affc3 decision, open: Errors about customer data must not echo the rejected value: they are…
- … and 19 more
