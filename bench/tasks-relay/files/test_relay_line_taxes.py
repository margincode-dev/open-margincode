from tallyho import Customer, Money, Order, Product
from tallyho.pricing import quote_order


def _order(region="US-CA"):
    o = Order("o1", Customer("c1", "Alice", "alice@example.com", region))
    for i in range(3):
        o.add(Product(f"PEN-{i}", f"Pen {i}", Money.of("0.10"), "office"), 1)
    o.add(Product("BOOK-7", "Field guide", Money.of("20.00"), "books"), 1)
    o.add(Product("CARD-1", "Gift card", Money.of("5.00"), "gifts", taxable=False), 1)
    return o


def test_line_taxes_cover_the_taxable_lines_in_order():
    q = quote_order(_order())
    assert [sku for sku, _ in q.line_taxes] == ["PEN-0", "PEN-1", "PEN-2"]


def test_line_taxes_add_up_exactly_to_the_tax():
    for region in ["US-CA", "US-NY", "US-TX", "GB"]:
        q = quote_order(_order(region))
        assert sum(m.minor for _, m in q.line_taxes) == q.tax.minor, region


def test_line_taxes_of_a_large_order_add_up():
    o = Order("o2", Customer("c1", "Alice", "alice@example.com", "US-CA"))
    for i, price in enumerate(["3.33", "7.77", "1.01", "9.99", "0.55"]):
        o.add(Product(f"ITEM-{i}", f"Item {i}", Money.of(price), "misc"), i + 1)
    q = quote_order(o)
    assert len(q.line_taxes) == 5
    assert sum(m.minor for _, m in q.line_taxes) == q.tax.minor
