## Task
# relay-order-comments · relay-order-comments · open
Goal: Staff want to keep short comments on an order. Add `Repository.add_order_comment(oid, text)` and `Repository.order_comments(oid)`, which returns that order's comments as a list of strings, oldest first, and an empty list for an order without any. Keep them in the shop's store, so they survive a restart.
Workspace: /m2b/r3/bench-work/ws/bfp43f-relay-order-comments-av-1/relay-order-comments
Changed (0): nothing yet

## Map
- repo map (35 files):
  .gitignore
  README.md
  pyproject.toml
  tallyho/ (25 files)
  tests/ (7 files)

## Code
- sym:1526 Order (tallyho/models.py:64-109)
  class Order:
- sym:1581 Repository (tallyho/repository.py:32-74)
  class Repository:
- sym:1518 OrderStatus (tallyho/models.py:14-19)
  class OrderStatus(str, Enum):
- sym:1589 Repository.order (tallyho/repository.py:57-59)
  def order(self, oid: str) -> Order:
- sym:1524 OrderLine.__post_init__ (tallyho/models.py:55-56)
  def __post_init__(self) -> None:
- sym:1523 OrderLine (tallyho/models.py:51-60)
  class OrderLine:
- sym:1619 summarize (tallyho/text.py:33-56)
  def summarize(text: str, max_length: int = 60) -> str:
- sym:1495 _repo (tallyho/cli.py:15-17)
  def _repo(cfg: Config, dry_run: bool) -> Repository:
- sym:1591 Repository.orders (tallyho/repository.py:72-74)
  def orders(self, status: OrderStatus | None = None) -> list[Order]:
- sym:1497 cmd_export (tallyho/cli.py:26-28)
  def cmd_export(repo: Repository, args: argparse.Namespace) -> int:
- sym:1498 main (tallyho/cli.py:31-42)
- sym:1590 Repository._order_from_record (tallyho/repository.py:61-70)
- sym:1515 Ledger.record_sale (tallyho/ledger.py:37-44)
- sym:1564 apply_discounts (tallyho/pricing/discounts.py:46-67)
- sym:1496 cmd_products (tallyho/cli.py:20-23)
- sym:1555 CouponBook.get (tallyho/pricing/coupons.py:49-50)
- sym:1567 quote_order (tallyho/pricing/quote.py:30-53)
- sym:1595 RecordStore.get (tallyho/store/base.py:28-29)
- sym:1659 test_summarize_first_sentence (tests/test_text.py:13-14)
  def test_summarize_first_sentence():
- sym:1656 test_list_page_cursor (tests/test_store.py:20-25)

---
tokens: 677/800
