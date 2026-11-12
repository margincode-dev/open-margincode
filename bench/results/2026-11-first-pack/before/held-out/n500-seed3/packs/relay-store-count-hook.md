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
- rec:7678592e note: Benchmarks for `MemoryStore` are in a separate, manual job; the unit tests must not depend on timing.
  anchors: tallyho/store/memory.py:MemoryStore
- rec:36d9ad42 attempt: Replacing the loop in `RecordStore` with a comprehension made it short…
- rec:9b1044e6 attempt: list_page never returns more than max_page_size (100) records, whateve…
- rec:880fa64f decision: New optional parameters of `RecordStore.put` are keyword-only (after *…

## Code
- sym:16533 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:16539 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:16549 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:16556 count_records (tallyho/store/scan.py:23-24)
- sym:16545 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:16538 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:16555 iter_records (tallyho/store/scan.py:10-20)
- sym:16544 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- sym:16546 JsonFileStore.get (tallyho/store/jsonfile.py:56-60)
- sym:16551 MemoryStore.put (tallyho/store/memory.py:18-26)
- sym:16435 _repo (tallyho/cli.py:15-17)

---
tokens: 704/800
omitted 117 lower-ranked items (expand an id for detail):
- rec:94e98cf9 note: `JsonFileStore.put_many` is on the hot path of the nightly batch job;…
- rec:8116929b note: The repr of objects around `JsonFileStore.list_page` shows up in logs,…
- rec:0777992f attempt: Tried calling `RecordStore` twice to force a refresh; the second call…
- rec:79d5e3ce attempt: Wrapping the `memory` module in a bare try/except with a retry loop hi…
- rec:1fcac4ad note: Benchmarks for `JsonFileStore.put_many` are in a separate, manual job;…
- rec:c2242c59 note: The repr of objects around `FixedDiscount.amount_off` shows up in logs…
- … and 111 more
