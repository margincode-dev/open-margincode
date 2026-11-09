## Task
# httpx-336204f · httpx-336204f-2 · open
Goal: Display proxy protocol scheme on error (#3571)
Workspace: /root/bench/ws/rebf815-httpx-336204f/httpx-336204f-2
Changed (0): nothing yet

## Map
- repo map (125 files):
  .github/ (8 files)
  .gitignore
  CHANGELOG.md
  LICENSE.md
  README.md
  docs/ (36 files)
  httpx/ (24 files)
  mkdocs.yml
  pyproject.toml
  requirements.txt
  scripts/ (10 files)
  tests/ (40 files)

## Code
- sym:56307 _load_httpcore_exceptions (httpx/_transports/default.py:74-92)
```python
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
```
- sym:56022 Proxy (httpx/_config.py:201-243)
  class Proxy:
- sym:56023 Proxy.__init__ (httpx/_config.py:202-224)
  def __init__( self, url: URL | str, *, ssl_context: ssl.SSLContext | None = None, auth: tuple[str, str] | None = None, headers: HeaderTypes | None = None, ) -> None:
- sym:56314 HTTPTransport.__init__ (httpx/_transports/default.py:136-215)
- sym:56433 URLPattern (httpx/_utils.py:120-226)
  class URLPattern:
- sym:56313 HTTPTransport (httpx/_transports/default.py:135-262)
  class HTTPTransport(BaseTransport):
- sym:55945 BaseClient.auth (httpx/_client.py:273-281)
  def auth(self) -> Auth | None:
- sym:55949 BaseClient.headers (httpx/_client.py:299-303)
  def headers(self) -> Headers:
- sym:56093 HTTPError (httpx/_exceptions.py:74-104)
  class HTTPError(Exception):
- sym:56100 TimeoutException (httpx/_exceptions.py:132-137)
  class TimeoutException(TransportError):
- sym:56946 test_invalid_proxy_scheme (tests/test_config.py:182-184)
  def test_invalid_proxy_scheme():
- sym:56609 test_unsupported_proxy_scheme (tests/client/test_proxies.py:123-125)
  def test_unsupported_proxy_scheme():

---
tokens: 672/800
