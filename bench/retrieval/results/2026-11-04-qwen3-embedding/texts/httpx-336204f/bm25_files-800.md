### tests/models/test_headers.py
```python
import pytest

import httpx


def test_headers():
    h = httpx.Headers([("a", "123"), ("a", "456"), ("b", "789")])
    assert "a" in h
    assert "A" in h
    assert "b" in h
    assert "B" in h
    assert "c" not in h
    assert h["a"] == "123, 456"
    assert h.get("a") == "123, 456"
    assert h.get("nope", default=None) is None
    assert h.get_list("a") == ["123", "456"]

    assert list(h.keys()) == ["a", "b"]
    assert list(h.values()) == ["123, 456", "789"]
    assert list(h.items()) == [("a", "123, 456"), ("b", "789")]
    assert h.multi_items() == [("a", "123"), ("a", "456"), ("b", "789")]
    assert list(h) == ["a", "b"]
    assert dict(h) == {"a": "123, 456", "b": "789"}
    assert repr(h) == "Headers([('a', '123'), ('a', '456'), ('b', '789')])"
    assert h == [("a", "123"), ("b", "789"), ("a", "456")]
    assert h == [("a", "123"), ("A", "456"), ("b", "789")]
    assert h == {"a": "123", "A": "456", "b": "789"}
    assert h != "a: 123\nA: 456\nb: 789"

    h = httpx.Headers({"a": "123", "b": "789"})
    assert h["A"] == "123"
    assert h["B"] == "789"
    assert h.raw == [(b"a", b"123"), (b"b", b"789")]
    assert repr(h) == "Headers({'a': '123', 'b': '789'})"


def test_header_mutations():
    h = httpx.Headers()
    assert dict(h) == {}
    h["a"] = "1"
    assert dict(h) == {"a": "1"}
    h["a"] = "2"
    assert dict(h) == {"a": "2"}
    h.setdefault("a", "3")
    assert dict(h) == {"a": "2"}
    h.setdefault("b", "4")
    assert dict(h) == {"a": "2", "b": "4"}
    del h["a"]
    assert dict(h) == {"b": "4"}
    assert h.raw == [(b"b", b"4")]


def test_copy_headers_method():
    headers = httpx.Headers({"custom": "example"})
    headers_copy = headers.copy()
    assert headers == headers_copy
    assert headers is not headers_copy


def test_copy_headers_init():
    headers = httpx.Headers({"custom": "example"})
    headers_copy = httpx.Headers(headers)
    assert headers == headers_copy


def test_headers_insert_retains_ordering():
    headers = httpx.Headers({"a": "a", "b": "b", "c": "c"})
    headers["b"] = "123"
    assert list(headers.values()) == ["a", "123", "c"]


def test_headers_insert_appends_if_new():
    headers = httpx.Headers({"a": "a", "b": "b", "c": "c"})
    headers["d"] = "123"
    assert list(headers.values()) == ["a", "b", "c", "123"]


def test_headers_insert_removes_all_existing():
