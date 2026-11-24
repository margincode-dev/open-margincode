from datetime import datetime, time, timedelta, timezone

TAX_RATE = 0.2


def invoice_total(lines):
    subtotal = 0.0
    for line in lines:
        subtotal += line["price"] / 100 * line["qty"]
    return round(subtotal * (1 + TAX_RATE), 2)


def due_date(issued, days):
    day = (issued + timedelta(days=days)).date()
    return datetime.combine(day, time(0, 0), tzinfo=timezone.utc)


def next_invoice_number(year, last):
    """`last` is the newest number issued in `year`, or None."""
    n = 1 if last is None else int(last.split("-")[1]) + 1
    return f"{year}-{n:04d}"
