from tallyho.text import pad, plural, slugify, summarize


def test_slugify():
    assert slugify("Blue Mug (Large)") == "blue-mug-large"


def test_pad():
    assert pad("abc", 5, "right") == "  abc"
    assert pad("abcdef", 4) == "abc…"


def test_summarize_first_sentence():
    assert summarize("Lists products. Use --category to filter.") == "Lists products."


def test_plural():
    assert plural(1, "order") == "1 order"
    assert plural(2, "order") == "2 orders"
