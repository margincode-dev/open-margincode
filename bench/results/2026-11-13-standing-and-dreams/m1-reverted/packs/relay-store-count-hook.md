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
- rec:7af4126f decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000002, ...): the store lists records in id order, so padded sequence ids keep listings chronological. No UUIDs.
  anchors: tallyho/repository.py:Repository
- rec:93650d16 decision, reverted: Count through the base class: len(self.list_page(collection, None, 10*…
- rec:c0f51f38 decision, open: Event payloads carry money as integer minor units plus the currency co…
- rec:7121843c intent, reverted: Earlier work on this code

## Code
- sym:763 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:769 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:779 MemoryStore (tallyho/store/memory.py:12-46)
- sym:786 count_records (tallyho/store/scan.py:23-24)
- sym:775 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:768 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:785 iter_records (tallyho/store/scan.py:10-20)
- sym:774 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:776 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:781 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:765 RecordStore.get (tallyho/store/base.py:28-29)

---
tokens: 709/800
omitted 26 lower-ranked items (expand an id for detail):
- rec:ea141c98 decision, open: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:20bd37fa decision, open: Errors about customer data must not echo the rejected value: they are…
- rec:057f0d4c decision, open: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:5ea19088 note, open: merge 61308357e198 reverted by bench-merger in 3054476c9c1c: the bench…
- rec:413c764f decision, reverted: Every currency tallyho supports has two decimals. Plain amounts are mi…
- … and 21 more
