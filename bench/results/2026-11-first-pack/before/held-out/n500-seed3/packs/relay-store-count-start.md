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
- rec:0777992f attempt: Tried calling `RecordStore` twice to force a refresh; the second call…
- rec:79d5e3ce attempt: Wrapping the `memory` module in a bare try/except with a retry loop hi…
- rec:36d9ad42 attempt: Replacing the loop in `RecordStore` with a comprehension made it short…

## Code
- sym:16539 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:16549 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:16533 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:16556 count_records (tallyho/store/scan.py:23-24)
- sym:16555 iter_records (tallyho/store/scan.py:10-20)
- sym:16545 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:16538 RecordStore.put_many (tallyho/store/base.py:39-41)
- sym:16435 _repo (tallyho/cli.py:15-17)
  def _repo(cfg: Config, dry_run: bool) -> Repository:
- sym:16594 store (tests/test_store.py:8-9)
- sym:16438 main (tallyho/cli.py:31-42)
- sym:16439 Config (tallyho/config.py:15-20)
  class Config:

---
tokens: 702/800
omitted 110 lower-ranked items (expand an id for detail):
- rec:a29c0d49 decision: Tests for `RecordStore` build their inputs with small factory helpers…
- rec:1d653ae1 decision: Tests for the `jsonfile` module build their inputs with small factory…
- rec:a7dbea3e decision: Constants that the `memory` module depends on live at the top of its m…
- rec:8e06a72c decision: Type hints on `_repo` use the built-in generics (list[str], dict[str,…
- rec:b504442a attempt: Replacing the loop in `Repository.__init__` with a comprehension made…
- rec:d6e41147 attempt: Caching what `main` returns in a module-level dict broke tests that ex…
- … and 104 more
