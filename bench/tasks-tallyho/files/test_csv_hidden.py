from tallyho.report import rows_to_csv


def test_plain():
    assert rows_to_csv(["a", "b"], [[1, "x"]]) == "a,b\n1,x\n"


def test_comma_quoted():
    assert rows_to_csv(["x"], [["a,b"]]) == 'x\n"a,b"\n'


def test_quotes_are_doubled():
    assert rows_to_csv(["x"], [['say "hi"']]) == 'x\n"say ""hi"""\n'


def test_newlines_quoted():
    assert rows_to_csv(["x"], [["two\nlines"]]) == 'x\n"two\nlines"\n'
    assert rows_to_csv(["x"], [["cr\rhere"]]) == 'x\n"cr\rhere"\n'


def test_leading_trailing_space_quoted():
    assert rows_to_csv(["x"], [[" padded "]]) == 'x\n" padded "\n'


def test_none_is_empty():
    assert rows_to_csv(["x", "y"], [[None, "v"]]) == "x,y\n,v\n"


def test_header_cells_escaped_too():
    assert rows_to_csv(['a"b'], []) == '"a""b"\n'
