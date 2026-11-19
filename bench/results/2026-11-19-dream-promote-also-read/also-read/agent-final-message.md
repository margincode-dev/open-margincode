I wrote `verdicts.json` with five `disputed` verdicts and left the other eight records out. I didn't run the daemon's quote check, so a quote may still be refused.

**Disputed**
- **`rec:e54af11e`**: it says every supported currency has two decimals. `MINOR_UNITS` gives JPY 0 and KWD 3.
- **`rec:ea0ec9a6`**: it says `list_page` returns every record when the limit is large. The docstring caps it at `min(limit, max_page_size)`, and `max_page_size` is 100, so the count would stop at 100.
- **`rec:ad6cdb90`**: it says `CouponBook` keys codes exactly as passed in. The book uppercases codes on add, and `get` strips and uppercases them.
- **`rec:2c5ac268`**: it says `replace()` leaves the original table's rates alone. `replace()` is a shallow copy, so the copy shares the `rates` dict and setting a rate on it changes the original.
- **`rec:d028e8f1`**: it says per-line taxes add up to `Quote.tax`. `quote_order` computes one tax on the discounted taxable base, so per-line rounding can differ.

**Left out**
- **`rec:0cd47a9b`**: it is a rule. `check_region` echoes the value, but code that breaks a rule doesn't dispute it.
- **`rec:da0c8013`**: it is also a rule. `subscribe` doesn't dedupe, but that doesn't show the rule is wrong.
- **`rec:e56b3a4c`**: the part about `allocate` holds, and I couldn't check the advice to split with `times(Decimal(1) / n)`.
- **`rec:bb19466a`**: `OrderLine` is frozen and `Order.add` does replace lines. I couldn't tell whether `extend` leaves duplicate SKU lines unmerged.
- **`rec:463a5370`**: I found no refund code to check it against.
- **`rec:4d2c4799`**: I found no order-id generation code to check it against.
- **`rec:77c350d1`**: I found no slug code to check it against.
- **`rec:e8bdeb23`**: the only event published carries just `{"id": order.id}`, so there is nothing to check the money format against.
