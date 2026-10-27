"""Fixed-width text tables."""

from __future__ import annotations

from ..text import pad


def render_table(headers: list[str], rows: list[list[str]], align: list[str] | None = None) -> str:
    """Render ``rows`` under ``headers`` with columns as wide as their widest cell."""
    align = align or ["left"] * len(headers)
    widths = [len(h) for h in headers]
    for row in rows:
        for i, cell in enumerate(row):
            widths[i] = max(widths[i], len(cell))
    lines = ["  ".join(pad(h, widths[i], align[i]) for i, h in enumerate(headers))]
    lines.append("  ".join("-" * w for w in widths))
    for row in rows:
        lines.append("  ".join(pad(c, widths[i], align[i]) for i, c in enumerate(row)))
    return "\n".join(l.rstrip() for l in lines)
