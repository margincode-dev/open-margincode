"""CSV export without the csv module, so the output is byte-stable across Python versions."""

from __future__ import annotations

from ..models import Order
from ..pricing import quote_order

ORDER_COLUMNS = ["id", "customer", "status", "items", "subtotal", "total", "currency"]


def _cell(value: object) -> str:
    text = "" if value is None else str(value)
    if "," in text:
        return f'"{text}"'
    return text


def rows_to_csv(headers: list[str], rows: list[list[object]]) -> str:
    """CSV text with a header line; lines end in ``\\n``."""
    out = [",".join(_cell(h) for h in headers)]
    for row in rows:
        out.append(",".join(_cell(v) for v in row))
    return "\n".join(out) + "\n"


def orders_to_csv(orders: list[Order]) -> str:
    rows = []
    for o in orders:
        q = quote_order(o)
        rows.append([o.id, o.customer.name, o.status.value, o.item_count(),
                     q.subtotal.amount, q.total.amount, o.currency])
    return rows_to_csv(ORDER_COLUMNS, rows)
