## Task
# relay-quote-split · relay-quote-split · open
Goal: Customers want to split a bill evenly between several payers. Add `Quote.split(parts)`, returning what each of the `parts` payers pays, as a list of Money. Raise ValidationError when parts is less than 1.

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
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:9aa7b2c7 attempt: Splitting Money by multiplying with 1/n (total.times(Decimal(1) / n))…
- rec:51abd097 decision: Errors about customer data must not echo the rejected value: they are…
- rec:75fc7202 attempt: Currencies have different minor units (JPY 0 decimals, KWD 3). A fixed…

## Code
- sym:18351 Money.allocate (tallyho/money.py:63-68)
- sym:18368 PercentDiscount.__post_init__ (tallyho/pricing/discounts.py:30-32)
- sym:18310 ValidationError (tallyho/errors.py:8-17)
- sym:18343 Money (tallyho/money.py:22-83)
- sym:18321 Ledger (tallyho/ledger.py:24-53)
- sym:18367 PercentDiscount (tallyho/pricing/discounts.py:27-35)
- sym:18373 Quote (tallyho/pricing/quote.py:16-27)
- sym:18374 Quote.lines (tallyho/pricing/quote.py:23-27)
- sym:18309 TallyError (tallyho/errors.py:4-5)
- sym:18322 Ledger.post (tallyho/ledger.py:28-35)
- sym:18328 Product.__post_init__ (tallyho/models.py:30-35)
- sym:18336 Order.add (tallyho/models.py:76-84)

---
tokens: 709/800
omitted 36 lower-ranked items (expand an id for detail):
- rec:460ab8dd decision: A refund is booked as the exact reverse of the sale: the same three ac…
- rec:abac29ba decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- sym:18337 Order.remove (tallyho/models.py:86-89)
- sym:18342 _exponent (tallyho/money.py:14-18)
- sym:18358 Coupon.discount (tallyho/pricing/coupons.py:23-28)
- sym:18409 JsonFileStore._path (tallyho/store/jsonfile.py:20-23)
- sym:18412 JsonFileStore.put (tallyho/store/jsonfile.py:39-45)
- … and 29 more
