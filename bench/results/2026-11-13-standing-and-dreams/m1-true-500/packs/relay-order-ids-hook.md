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
Claims by earlier agents: check against the code; correct wrong ones with `av record --supersedes rec:…`
- rec:a3c16b76 decision, open: `Repository.save_order` keeps its docstring as the one place its behaviour is described; the README only links to it.
  anchors: tallyho/repository.py:Repository.save_order
- rec:7988577a decision, open: New optional parameters of `Repository.save_order` are keyword-only (after *), so call sites stay readable when options are added.
  anchors: tallyho/repository.py:Repository.save_order
- rec:d20f5ea4 attempt, open: Tried calling `Repository.order` twice to force a refresh; the second…
- rec:fb4803a8 attempt, open: Wrapping `Repository.save_order` in a bare try/except with a retry loo…
- rec:ec8c5942 attempt, open: Adding type: ignore comments around `Repository._order_from_record` to…

## Code
- sym:5905 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:5904 Repository.save_order (tallyho/repository.py:54-55)
- sym:5907 Repository.orders (tallyho/repository.py:72-74)
- sym:5906 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:5897 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:5908 Page (tallyho/store/base.py:10-14)
- sym:5844 Order.add (tallyho/models.py:76-84)
- sym:5880 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:5842 Order (tallyho/models.py:64-109)
- sym:5848 Order.to_dict (tallyho/models.py:98-109)
- sym:5871 CouponBook.get (tallyho/pricing/coupons.py:49-50)

---
tokens: 688/800
omitted 31 lower-ranked items (expand an id for detail):
- rec:d696e413 attempt, open: Replacing the loop in `Repository.save_order` with a comprehension mad…
- rec:4ce75335 attempt, open: Adding type: ignore comments around `Repository.save_order` to quiet t…
- rec:50d96a2b attempt, open: Moving the imports inside `Repository.save_order` to speed up start-up…
- rec:c737b29e attempt, open: Tried calling `RecordStore.put` twice to force a refresh; the second c…
- rec:5fdeab27 attempt, open: Wrapping `orders_to_csv` in a bare try/except with a retry loop hid th…
- … and 26 more
