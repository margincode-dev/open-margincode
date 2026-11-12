## Task
# relay-customer-region · relay-customer-region · open

## Map
- tallyho/errors.py
- tallyho/ledger.py
- tallyho/models.py
- tallyho/money.py
- tallyho/pricing/coupons.py
- tallyho/pricing/discounts.py
- tallyho/store/jsonfile.py
- tallyho/store/memory.py
- tallyho/validate.py
- tallyho/workflow.py
- tests/test_models.py
- tallyho/events.py
- repo map: .gitignore README.md pyproject.toml tallyho/ tests/ (35 files)

## Decisions & dead ends
- rec:fdd3c27e decision: Errors about customer data must not echo the rejected value: they are logged and sent to support. Say what is wrong ("invalid region"), set field=, never include the value.
  anchors: tallyho/validate.py:check_region
- rec:c7b8cd3b note: `ValidationError.__str__` reads files as UTF-8 explicitly; the platfor…
- rec:0a62fad3 decision: New optional parameters of `ValidationError` are keyword-only (after *…
- rec:7d33ee90 note: Benchmarks for `ValidationError` are in a separate, manual job; the unit tests must not depend on timing.
  anchors: tallyho/errors.py:ValidationError

## Code
- sym:35616 check_region (tallyho/validate.py:24-26)
- sym:35614 check_sku (tallyho/validate.py:14-16)
- sym:35510 Product.__post_init__ (tallyho/models.py:30-35)
- sym:35618 parse_quantity (tallyho/validate.py:34-42)
- sym:35615 check_email (tallyho/validate.py:19-21)
- sym:35617 require_positive (tallyho/validate.py:29-31)
- sym:35492 ValidationError (tallyho/errors.py:8-17)
  class ValidationError(TallyError):
- sym:35509 Product (tallyho/models.py:23-35)
- sym:35516 Order (tallyho/models.py:64-109)
- sym:35613 _REGION (tallyho/validate.py:11-11)
- sym:35511 Customer (tallyho/models.py:39-47)
- sym:35589 JsonFileStore (tallyho/store/jsonfile.py:14-72)

---
tokens: 702/800
omitted 184 lower-ranked items (expand an id for detail):
- rec:45201186 attempt: Wrapping `ValidationError` in a bare try/except with a retry loop hid…
- rec:ac6c1140 decision: Tests for `check_region` build their inputs with small factory helpers…
- rec:5f97d6ba decision: Constants that `Customer.__post_init__` depends on live at the top of…
- rec:685fa72d note: `Workflow.place` reads files as UTF-8 explicitly; the platform default…
- rec:86a666e6 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:87ec60dd note: The `validate` module reads files as UTF-8 explicitly; the platform de…
- … and 178 more
