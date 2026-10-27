"""Text helpers for reports and the CLI."""

from __future__ import annotations

import unicodedata


def slugify(text: str) -> str:
    """``"Blue Mug (Large)"`` -> ``"blue-mug-large"``."""
    norm = unicodedata.normalize("NFKD", text).encode("ascii", "ignore").decode()
    out, dash = [], False
    for ch in norm.lower():
        if ch.isalnum():
            out.append(ch)
            dash = False
        elif not dash and out:
            out.append("-")
            dash = True
    return "".join(out).rstrip("-")


def pad(text: str, width: int, align: str = "left") -> str:
    """Pad or cut ``text`` to exactly ``width`` characters."""
    if len(text) > width:
        return text[: max(width - 1, 0)] + "…" if width else ""
    if align == "right":
        return text.rjust(width)
    if align == "center":
        return text.center(width)
    return text.ljust(width)


def summarize(text: str, max_length: int = 60) -> str:
    """The first sentence of ``text``, for one-line listings.

    Only the first paragraph counts and whitespace is collapsed. The text is cut after
    the first period; if that is still longer than ``max_length`` it is cut on a word
    boundary and ``"..."`` is appended.
    """
    para = text.split("\n\n", 1)[0]
    words = para.split()
    if not words:
        return ""
    length = 0
    for i, word in enumerate(words):
        length += len(word) + (1 if i else 0)
        if word.endswith("."):
            words = words[: i + 1]
            break
    result = " ".join(words)
    if len(result) <= max_length:
        return result
    cut = result[: max_length - 3]
    if " " in cut:
        cut = cut[: cut.rindex(" ")]
    return cut + "..."


def plural(n: int, word: str, suffix: str = "s") -> str:
    return f"{n} {word}" if n == 1 else f"{n} {word}{suffix}"
