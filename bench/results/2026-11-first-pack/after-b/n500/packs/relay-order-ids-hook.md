## Task
# relay-order-ids · relay-order-ids · open
Goal: Add `Repository.next_order_id()`, which returns the id to give a new order.

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Decisions & dead ends
- rec:a58f28bc decision: `Repository.save_order` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/repository.py:Repository.save_order
- rec:b40bc2db decision: New optional parameters of `Repository.save_order` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.save_order
- rec:0a889caa attempt: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:4f9981d8 attempt: Wrapping `Repository.save_order` in a bare try/except with a retry loo…
- rec:11240957 attempt: Adding type: ignore comments around `Repository._order_from_record` to…
- rec:dbc43b88 attempt: Adding type: ignore comments around `Repository.save_order` to quiet t…

## Code
- sym:36243 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:36242 Repository.save_order (tallyho/repository.py:54-55)
- sym:36245 Repository.orders (tallyho/repository.py:72-74)
- sym:36244 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:36235 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:36246 Page (tallyho/store/base.py:10-14)
  class Page:
- sym:36182 Order.add (tallyho/models.py:76-84)
- sym:36218 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:36180 Order (tallyho/models.py:64-109)
- sym:36186 Order.to_dict (tallyho/models.py:98-109)
- sym:36209 CouponBook.get (tallyho/pricing/coupons.py:49-50)

---
tokens: 697/800
omitted 30 lower-ranked items (expand an id for detail):
- rec:a300f7d3 attempt: Replacing the loop in `Repository.save_order` with a comprehension mad…
- rec:3062cae4 attempt: Moving the imports inside `Repository.save_order` to speed up start-up…
- rec:e53e5286 attempt: Tried calling `RecordStore.put` twice to force a refresh; the second c…
- rec:51ff612c attempt: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- rec:c567c0c9 attempt: Tried calling `CouponBook.get` twice to force a refresh; the second ca…
- rec:bd86eec4 attempt: Replacing the loop in `OrderLine.__post_init__` with a comprehension m…
- … and 24 more
