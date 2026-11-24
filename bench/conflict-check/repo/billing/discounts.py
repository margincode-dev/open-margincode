def apply_discount(lines, rate):
    """The total of `lines` after a discount of `rate` (0.1 is 10 %)."""
    total = sum(line["price"] * line["qty"] for line in lines) / 100
    return round(total * (1 - rate), 2)
