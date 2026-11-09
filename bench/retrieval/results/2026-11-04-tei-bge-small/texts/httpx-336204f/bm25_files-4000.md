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
    headers = httpx.Headers([("a", "123"), ("a", "456")])
    headers["a"] = "789"
    assert dict(headers) == {"a": "789"}


def test_headers_delete_removes_all_existing():
    headers = httpx.Headers([("a", "123"), ("a", "456")])
    del headers["a"]
    assert dict(headers) == {}


def test_headers_dict_repr():
    """
    Headers should display with a dict repr by default.
    """
    headers = httpx.Headers({"custom": "example"})
    assert repr(headers) == "Headers({'custom': 'example'})"


def test_headers_encoding_in_repr():
    """
    Headers should display an encoding in the repr if required.
    """
    headers = httpx.Headers({b"custom": "example ☃".encode("utf-8")})
    assert repr(headers) == "Headers({'custom': 'example ☃'}, encoding='utf-8')"


def test_headers_list_repr():
    """
    Headers should display with a list repr if they include multiple identical keys.
    """
    headers = httpx.Headers([("custom", "example 1"), ("custom", "example 2")])
    assert (
        repr(headers) == "Headers([('custom', 'example 1'), ('custom', 'example 2')])"
    )


def test_headers_decode_ascii():
    """
    Headers should decode as ascii by default.
    """
    raw_headers = [(b"Custom", b"Example")]
    headers = httpx.Headers(raw_headers)
    assert dict(headers) == {"custom": "Example"}
    assert headers.encoding == "ascii"


def test_headers_decode_utf_8():
    """
    Headers containing non-ascii codepoints should default to decoding as utf-8.
    """
    raw_headers = [(b"Custom", "Code point: ☃".encode("utf-8"))]
    headers = httpx.Headers(raw_headers)
    assert dict(headers) == {"custom": "Code point: ☃"}
    assert headers.encoding == "utf-8"


def test_headers_decode_iso_8859_1():
    """
    Headers containing non-UTF-8 codepoints should default to decoding as iso-8859-1.
    """
    raw_headers = [(b"Custom", "Code point: ÿ".encode("iso-8859-1"))]
    headers = httpx.Headers(raw_headers)
    assert dict(headers) == {"custom": "Code point: ÿ"}
    assert headers.encoding == "iso-8859-1"


def test_headers_decode_explicit_encoding():
    """
    An explicit encoding may be set on headers in order to force a
    particular decoding.
    """
    raw_headers = [(b"Custom", "Code point: ☃".encode("utf-8"))]
    headers = httpx.Headers(raw_headers)
    headers.encoding = "iso-8859-1"
    assert dict(headers) == {"custom": "Code point: â\x98\x83"}
    assert headers.encoding == "iso-8859-1"


def test_multiple_headers():
    """
    `Headers.get_list` should support both split_commas=False and split_commas=True.
    """
    h = httpx.Headers([("set-cookie", "a, b"), ("set-cookie", "c")])
    assert h.get_list("Set-Cookie") == ["a, b", "c"]

    h = httpx.Headers([("vary", "a, b"), ("vary", "c")])
    assert h.get_list("Vary", split_commas=True) == ["a", "b", "c"]


@pytest.mark.parametrize("header", ["authorization", "proxy-authorization"])
def test_sensitive_headers(header):
    """
    Some headers should be obfuscated because they contain sensitive data.
    """
    value = "s3kr3t"
    h = httpx.Headers({header: value})
    assert repr(h) == "Headers({'%s': '[secure]'})" % header


@pytest.mark.parametrize(
    "headers, output",
    [
        ([("content-type", "text/html")], [("content-type", "text/html")]),
        ([("authorization", "s3kr3t")], [("authorization", "[secure]")]),
        ([("proxy-authorization", "s3kr3t")], [("proxy-authorization", "[secure]")]),
    ],
)
def test_obfuscate_sensitive_headers(headers, output):
    as_dict = {k: v for k, v in output}
    headers_class = httpx.Headers({k: v for k, v in headers})
    assert repr(headers_class) == f"Headers({as_dict!r})"


@pytest.mark.parametrize(
    "value, expected",
    (
        (
            '<http:/.../front.jpeg>; rel=front; type="image/jpeg"',
            [{"url": "http:/.../front.jpeg", "rel": "front", "type": "image/jpeg"}],
        ),
        ("<http:/.../front.jpeg>", [{"url": "http:/.../front.jpeg"}]),
        ("<http:/.../front.jpeg>;", [{"url": "http:/.../front.jpeg"}]),
        (
            '<http:/.../front.jpeg>; type="image/jpeg",<http://.../back.jpeg>;',
            [
                {"url": "http:/.../front.jpeg", "type": "image/jpeg"},
                {"url": "http://.../back.jpeg"},
            ],
        ),
        ("", []),
    ),
)
def test_parse_header_links(value, expected):
    all_links = httpx.Response(200, headers={"link": value}).links.values()
    assert all(link in all_links for link in expected)


def test_parse_header_links_no_link():
    all_links = httpx.Response(200).links
    assert all_links == {}
```
### httpx/_transports/default.py
```python
"""
Custom transports, with nicely configured defaults.

The following additional keyword arguments are currently supported by httpcore...

* uds: str
* local_address: str
* retries: int

Example usages...

# Disable HTTP/2 on a single specific domain.
mounts = {
    "all://": httpx.HTTPTransport(http2=True),
    "all://*example.org": httpx.HTTPTransport()
}

# Using advanced httpcore configuration, with connection retries.
transport = httpx.HTTPTransport(retries=1)
client = httpx.Client(transport=transport)

# Using advanced httpcore configuration, with unix domain sockets.
transport = httpx.HTTPTransport(uds="socket.uds")
client = httpx.Client(transport=transport)
"""

from __future__ import annotations

import contextlib
import typing
from types import TracebackType

if typing.TYPE_CHECKING:
    import ssl  # pragma: no cover

    import httpx  # pragma: no cover

from .._config import DEFAULT_LIMITS, Limits, Proxy, create_ssl_context
from .._exceptions import (
    ConnectError,
    ConnectTimeout,
    LocalProtocolError,
    NetworkError,
    PoolTimeout,
    ProtocolError,
    ProxyError,
    ReadError,
    ReadTimeout,
    RemoteProtocolError,
    TimeoutException,
    UnsupportedProtocol,
    WriteError,
    WriteTimeout,
)
from .._models import Request, Response
from .._types import AsyncByteStream, CertTypes, ProxyTypes, SyncByteStream
from .._urls import URL
from .base import AsyncBaseTransport, BaseTransport

T = typing.TypeVar("T", bound="HTTPTransport")
A = typing.TypeVar("A", bound="AsyncHTTPTransport")

SOCKET_OPTION = typing.Union[
    typing.Tuple[int, int, int],
    typing.Tuple[int, int, typing.Union[bytes, bytearray]],
    typing.Tuple[int, int, None, int],
]

__all__ = ["AsyncHTTPTransport", "HTTPTransport"]

HTTPCORE_EXC_MAP: dict[type[Exception], type[httpx.HTTPError]] = {}


def _load_httpcore_exceptions() -> dict[type[Exception], type[httpx.HTTPError]]:
    import httpcore

    return {
        httpcore.TimeoutException: TimeoutException,
        httpcore.ConnectTimeout: ConnectTimeout,
        httpcore.ReadTimeout: ReadTimeout,
        httpcore.WriteTimeout: WriteTimeout,
        httpcore.PoolTimeout: PoolTimeout,
        httpcore.NetworkError: NetworkError,
        httpcore.ConnectError: ConnectError,
        httpcore.ReadError: ReadError,
        httpcore.WriteError: WriteError,
        httpcore.ProxyError: ProxyError,
        httpcore.UnsupportedProtocol: UnsupportedProtocol,
        httpcore.ProtocolError: ProtocolError,
        httpcore.LocalProtocolError: LocalProtocolError,
        httpcore.RemoteProtocolError: RemoteProtocolError,
    }


@contextlib.contextmanager
def map_httpcore_exceptions() -> typing.Iterator[None]:
    global HTTPCORE_EXC_MAP
    if len(HTTPCORE_EXC_MAP) == 0:
        HTTPCORE_EXC_MAP = _load_httpcore_exceptions()
    try:
        yield
    except Exception as exc:
        mapped_exc = None

        for from_exc, to_exc in HTTPCORE_EXC_MAP.items():
            if not isinstance(exc, from_exc):
                continue
            # We want to map to the most specific exception we can find.
            # Eg if `exc` is an `httpcore.ReadTimeout`, we want to map to
            # `httpx.ReadTimeout`, not just `httpx.TimeoutException`.
            if mapped_exc is None or issubclass(to_exc, mapped_exc):
                mapped_exc = to_exc

        if mapped_exc is None:  # pragma: no cover
            raise

        message = str(exc)
        raise mapped_exc(message) from exc


class ResponseStream(SyncByteStream):
    def __init__(self, httpcore_stream: typing.Iterable[bytes]) -> None:
        self._httpcore_stream = httpcore_stream

    def __iter__(self) -> typing.Iterator[bytes]:
        with map_httpcore_exceptions():
            for part in self._httpcore_stream:
                yield part

    def close(self) -> None:
        if hasattr(self._httpcore_stream, "close"):
            self._httpcore_stream.close()


class HTTPTransport(BaseTransport):
    def __init__(
        self,
        verify: ssl.SSLContext | str | bool = True,
        cert: CertTypes | None = None,
        trust_env: bool = True,
        http1: bool = True,
        http2: bool = False,
        limits: Limits = DEFAULT_LIMITS,
        proxy: ProxyTypes | None = None,
        uds: str | None = None,
        local_address: str | None = None,
        retries: int = 0,
        socket_options: typing.Iterable[SOCKET_OPTION] | None = None,
    ) -> None:
        import httpcore

        proxy = Proxy(url=proxy) if isinstance(proxy, (str, URL)) else proxy
        ssl_context = create_ssl_context(verify=verify, cert=cert, trust_env=trust_env)

        if proxy is None:
            self._pool = httpcore.ConnectionPool(
                ssl_context=ssl_context,
                max_connections=limits.max_connections,
                max_keepalive_connections=limits.max_keepalive_connections,
                keepalive_expiry=limits.keepalive_expiry,
                http1=http1,
                http2=http2,
                uds=uds,
                local_address=local_address,
                retries=retries,
                socket_options=socket_options,
            )
        elif proxy.url.scheme in ("http", "https"):
            self._pool = httpcore.HTTPProxy(
                proxy_url=httpcore.URL(
                    scheme=proxy.url.raw_scheme,
                    host=proxy.url.raw_host,
                    port=proxy.url.port,
                    target=proxy.url.raw_path,
                ),
                proxy_auth=proxy.raw_auth,
                proxy_headers=proxy.headers.raw,
                ssl_context=ssl_context,
                proxy_ssl_context=proxy.ssl_context,
                max_connections=limits.max_connections,
                max_keepalive_connections=limits.max_keepalive_connections,
                keepalive_expiry=limits.keepalive_expiry,
                http1=http1,
                http2=http2,
                socket_options=socket_options,
            )
        elif proxy.url.scheme in ("socks5", "socks5h"):
            try:
                import socksio  # noqa
            except ImportError:  # pragma: no cover
                raise ImportError(
                    "Using SOCKS proxy, but the 'socksio' package is not installed. "
                    "Make sure to install httpx using `pip install httpx[socks]`."
                ) from None

            self._pool = httpcore.SOCKSProxy(
                proxy_url=httpcore.URL(
                    scheme=proxy.url.raw_scheme,
                    host=proxy.url.raw_host,
                    port=proxy.url.port,
                    target=proxy.url.raw_path,
                ),
                proxy_auth=proxy.raw_auth,
                ssl_context=ssl_context,
                max_connections=limits.max_connections,
                max_keepalive_connections=limits.max_keepalive_connections,
                keepalive_expiry=limits.keepalive_expiry,
                http1=http1,
                http2=http2,
            )
        else:  # pragma: no cover
            raise ValueError(
                "Proxy protocol must be either 'http', 'https', 'socks5', or 'socks5h',"
                f" but got {proxy.url.scheme!r}."
            )

    def __enter__(self: T) -> T:  # Use generics for subclass support.
        self._pool.__enter__()
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None = None,
        exc_value: BaseException | None = None,
        traceback: TracebackType | None = None,
    ) -> None:
        with map_httpcore_exceptions():
            self._pool.__exit__(exc_type, exc_value, traceback)

    def handle_request(
        self,
        request: Request,
    ) -> Response:
        assert isinstance(request.stream, SyncByteStream)
        import httpcore

        req = httpcore.Request(
            method=request.method,
            url=httpcore.URL(
                scheme=request.url.raw_scheme,
                host=request.url.raw_host,
                port=request.url.port,
                target=request.url.raw_path,
            ),
            headers=request.headers.raw,
            content=request.stream,
            extensions=request.extensions,
        )
        with map_httpcore_exceptions():
            resp = self._pool.handle_request(req)

        assert isinstance(resp.stream, typing.Iterable)

        return Response(
            status_code=resp.status,
            headers=resp.headers,
            stream=ResponseStream(resp.stream),
            extensions=resp.extensions,
        )

    def close(self) -> None:
        self._pool.close()


class AsyncResponseStream(AsyncByteStream):
    def __init__(self, httpcore_stream: typing.AsyncIterable[bytes]) -> None:
        self._httpcore_stream = httpcore_stream

    async def __aiter__(self) -> typing.AsyncIterator[bytes]:
        with map_httpcore_exceptions():
            async for part in self._httpcore_stream:
                yield part

    async def aclose(self) -> None:
        if hasattr(self._httpcore_stream, "aclose"):
            await self._httpcore_stream.aclose()


class AsyncHTTPTransport(AsyncBaseTransport):
    def __init__(
        self,
