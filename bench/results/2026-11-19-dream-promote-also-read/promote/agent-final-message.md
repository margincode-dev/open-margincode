I wrote `verdicts.json` with five `disputed` verdicts out of the 13 records. I haven't checked that the daemon accepts the quotes.

**Disputed**
- **`rec:07332ec6`:** `CouponBook` does normalize codes inside the book. `get` strips and uppercases, and `add` uppercases.
- **`rec:28276355`:** `list_page` is capped at `max_page_size` (100), so counting with a huge limit stops at 100.
- **`rec:3811637e`:** JPY has 0 decimals and KWD has 3, and `format()` uses `_exponent(currency)` for the digits instead of a fixed `:.2f`.
- **`rec:6a9b6d47`:** `dataclasses.replace` is a shallow copy, so the copy shares the original's `rates` dict. Setting a rate on the copy also changes the original.
- **`rec:8df77536`:** `Quote.tax` is one rounded tax on the discounted taxable total, so per-line rounded taxes need not add up to it.

**Left out because I couldn't tell**
- **`rec:396fbd00`, `rec:b964b8f1`, `rec:c1fc90af`:** each states a rule, and code that breaks a rule doesn't dispute it. `subscribe` and `check_region` look like the bugs the rules target, and the workflow publishes no money payloads yet.
- **`rec:91703658`, `rec:adb16324`, `rec:fb1e7ed9`:** I found no code for refunds, order-id generation or product slugs. `slugify` exists in `tallyho/text.py`, but nothing uses it for products.
- **`rec:7b87e59b`:** `allocate` is documented as an exact-sum split, but the record's reason (uneven parts) does hold, so I couldn't judge the advice.
- **`rec:d581f309`:** `extend` skips the SKU merging that `add` does, so it may leave duplicate lines for one SKU. I couldn't confirm whether any code relies on one line per SKU.
