## Task
# relay-order-merge · relay-order-merge · open

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
- rec:776ca6b5 decision: Merge orders with self.lines.extend(other.lines) after the draft check. Do not loop over Order.add: it rescans every line for each product and replaces line objects that other code already holds.
  anchors: tallyho/models.py:Order
- rec:a9d30883 decision: Split a Money amount with total.times(Decimal(1) / n) for each of the…
- rec:ba9a13f9 decision: Errors about customer data must not echo the rejected value: they are…
- rec:0c28f7c1 decision: A refund is booked as the exact reverse of the sale: the same three ac…

## Code
- sym:366 Order.add (tallyho/models.py:76-84)
- sym:364 Order (tallyho/models.py:64-109)
  class Order:
- sym:471 Workflow.place (tallyho/workflow.py:35-39)
- sym:367 Order.remove (tallyho/models.py:86-89)
  def remove(self, sku: str) -> None:
- sym:340 ValidationError (tallyho/errors.py:8-17)
- sym:419 Repository (tallyho/repository.py:32-74)
- sym:421 Repository.save_product (tallyho/repository.py:38-39)
- sym:426 Repository.save_order (tallyho/repository.py:54-55)
- sym:402 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:424 Repository.save_customer (tallyho/repository.py:48-49)
- sym:376 Money._check (tallyho/money.py:39-43)
- sym:423 Repository.products (tallyho/repository.py:44-46)
- sym:339 TallyError (tallyho/errors.py:4-5)

---
tokens: 711/800
omitted 47 lower-ranked items (expand an id for detail):
- rec:ebe2193e decision: Product slugs are the slugified SKU and name together: MUG-1 "Blue mug…
- rec:c99dd035 decision: Every currency tallyho supports has two decimals. Plain amounts are mi…
- rec:d99152b9 decision: EventBus.subscribe dedupes by name: skip a handler when one with the s…
- rec:4ee855f2 decision: Event payloads carry money as integer minor units plus the currency co…
- rec:cff1a96f decision: Order ids are "o" plus a six-digit zero-padded sequence (o000001, o000…
- rec:1de08201 decision: Copy a TaxTable with dataclasses.replace(self), then set the rate on t…
- … and 41 more
