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
- rec:93fb8260 decision: Count through the base class: len(self.list_page(collection, None, 10**9).records). list_page returns every record when the limit is large, so one count() in RecordStore serves all backends.
  anchors: tallyho/store/base.py:RecordStore.list_page
- rec:be5cbe42 decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:44743a2b decision: Event payloads carry money as integer minor units plus the currency co…
- rec:59386a26 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…

## Code
- sym:42721 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:42727 JsonFileStore (tallyho/store/jsonfile.py:14-72)
- sym:42737 MemoryStore (tallyho/store/memory.py:12-46)
- sym:42744 count_records (tallyho/store/scan.py:23-24)
- sym:42733 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:42726 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:42743 iter_records (tallyho/store/scan.py:10-20)
- sym:42732 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:42734 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:42739 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:42723 RecordStore.get (tallyho/store/base.py:28-29)

---
tokens: 708/800
omitted 23 lower-ranked items (expand an id for detail):
- rec:06c07ca7 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:8803061a decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:709ffdcc decision: CouponBook keys _by_code by the code exactly as it is passed in. Looku…
- rec:d6eefaf2 decision: Errors about customer data must not echo the rejected value: they are…
- rec:5377de32 decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:1fe0615c decision: Tax each line on its own: taxes.tax(line.subtotal, region) per taxable…
- … and 17 more
