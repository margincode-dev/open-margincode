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
- rec:fef23872 note, open: `MemoryStore.delete` is on the hot path of the nightly batch job; profile before adding work to it.
  anchors: tallyho/store/memory.py:MemoryStore.delete
- rec:a55a10dc attempt, open: Adding type: ignore comments around `JsonFileStore.put_many` to quiet the checker hid a real Optional bug two callers up.
  anchors: tallyho/store/jsonfile.py:JsonFileStore.put_many
- rec:5dc870bd attempt, open: Tried calling `Repository.order` twice to force a refresh; the second…

## Code
- sym:4919 JsonFileStore (tallyho/store/jsonfile.py:14-72)
  class JsonFileStore(RecordStore):
- sym:4929 MemoryStore (tallyho/store/memory.py:12-46)
  class MemoryStore(RecordStore):
- sym:4913 RecordStore (tallyho/store/base.py:17-41)
  class RecordStore(ABC):
- sym:4815 _repo (tallyho/cli.py:15-17)
- sym:4936 count_records (tallyho/store/scan.py:23-24)
- sym:4819 Config (tallyho/config.py:15-20)
  class Config:
- sym:4935 iter_records (tallyho/store/scan.py:10-20)
- sym:4826 NotFound (tallyho/errors.py:24-25)
- sym:4835 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:4925 JsonFileStore.put_many (tallyho/store/jsonfile.py:47-54)
- sym:4846 Order (tallyho/models.py:64-109)
  class Order:

---
tokens: 704/800
omitted 27 lower-ranked items (expand an id for detail):
- rec:23a6539f attempt, open: Wrapping `count_records` in a bare try/except with a retry loop hid th…
- rec:ce56dbee attempt, open: Wrapping `JsonFileStore.delete` in a bare try/except with a retry loop…
- rec:360887f1 decision, open: New optional parameters of `Repository.save_order` are keyword-only (a…
- rec:6569af07 decision, open: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:6a802944 decision, open: New optional parameters of `Repository.product` are keyword-only (afte…
- … and 22 more
