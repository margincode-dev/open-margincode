from tallyho.report import render_table, rows_to_csv


def test_render_table():
    out = render_table(["a", "b"], [["1", "22"]])
    assert out.splitlines()[0] == "a  b"


def test_csv_quotes_commas():
    assert rows_to_csv(["x"], [["a,b"]]) == 'x\n"a,b"\n'
