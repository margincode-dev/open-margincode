from tallyho.text import summarize


def test_abbreviation_does_not_end_sentence():
    assert summarize("Compare mugs vs. cups and report.") == "Compare mugs vs. cups and report."


def test_eg_inside_sentence():
    assert summarize("Sort orders, e.g. by date. Then print them.") == "Sort orders, e.g. by date."


def test_period_before_capital_ends_sentence():
    assert summarize("Lists products. Use --category to filter.") == "Lists products."


def test_long_sentence_is_shortened_with_suffix():
    out = summarize("Export every paid order of the current month into one file for the accountant", 30)
    assert out.endswith("...")
    assert len(out) <= 30
    assert out == "Export every paid order of..."


def test_hyphenated_words_are_not_split():
    out = summarize("Re-export the well-known best-selling products list now", 24)
    assert out == "Re-export the..."


def test_tiny_max_length():
    assert summarize("Something long enough", 2) == "..."


def test_only_first_paragraph_and_whitespace_collapsed():
    assert summarize("First   line\ncontinues here\n\nSecond paragraph.") == "First line continues here"


def test_empty():
    assert summarize("   ") == ""
