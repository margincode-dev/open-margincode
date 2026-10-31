"""A one-page HTML report. Unstyled for now (task: style the report)."""

from html import escape


def render(items):
    rows = "".join(
        f"<tr><td>{escape(i.name)}</td><td>{escape(i.category)}</td>"
        f"<td>{i.quantity}</td><td>{i.expires}</td></tr>"
        for i in sorted(items, key=lambda i: i.expires)
    )
    return (
        "<!DOCTYPE html><html><head><meta charset='utf-8'><title>Pantry</title></head><body>"
        "<h1>Pantry</h1><table><tr><th>Item</th><th>Category</th><th>Qty</th><th>Expires</th></tr>"
        f"{rows}</table></body></html>\n"
    )
