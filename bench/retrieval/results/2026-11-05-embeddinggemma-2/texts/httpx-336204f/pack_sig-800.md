## Task
# httpx-336204f · httpx-336204f · open
Goal: Display proxy protocol scheme on error (#3571)
Workspace: /root/bench/av_gemma/ws/rec728b-httpx-336204f/httpx-336204f
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
- sym:56022 Proxy (httpx/_config.py:201-243)
  class Proxy:
- sym:56307 _load_httpcore_exceptions (httpx/_transports/default.py:74-92)
  def _load_httpcore_exceptions() -> dict[type[Exception], type[httpx.HTTPError]]:
- sym:55940 BaseClient._get_proxy_map (httpx/_client.py:239-251)
  def _get_proxy_map( self, proxy: ProxyTypes | None, allow_env_proxies: bool ) -> dict[str, Proxy | None]:
- sym:56023 Proxy.__init__ (httpx/_config.py:202-224)
  def __init__( self, url: URL | str, *, ssl_context: ssl.SSLContext | None = None, auth: tuple[str, str] | None = None, headers: HeaderTypes | None = None, ) -> None:
- sym:56144 trace (httpx/_main.py:212-248)
  def trace( name: str, info: typing.Mapping[str, typing.Any], verbose: bool = False ) -> None:
- sym:56314 HTTPTransport.__init__ (httpx/_transports/default.py:136-215)
- sym:56433 URLPattern (httpx/_utils.py:120-226)
  class URLPattern:
- sym:56150 SENSITIVE_HEADERS (httpx/_models.py:53-53)
  SENSITIVE_HEADERS = {"authorization", "proxy-authorization"}
- sym:56313 HTTPTransport (httpx/_transports/default.py:135-262)
  class HTTPTransport(BaseTransport):
- sym:56946 test_invalid_proxy_scheme (tests/test_config.py:182-184)
  def test_invalid_proxy_scheme():
- sym:56609 test_unsupported_proxy_scheme (tests/client/test_proxies.py:123-125)
  def test_unsupported_proxy_scheme():
- sym:55945 BaseClient.auth (httpx/_client.py:273-281)
  def auth(self) -> Auth | None:
- sym:55949 BaseClient.headers (httpx/_client.py:299-303)
- sym:56093 HTTPError (httpx/_exceptions.py:74-104)
  class HTTPError(Exception):
- sym:56100 TimeoutException (httpx/_exceptions.py:132-137)
- sym:56605 PROXY_URL (tests/client/test_proxies.py:31-31)
- sym:57010 test_errors (tests/test_main.py:181-187)

---
tokens: 679/800
