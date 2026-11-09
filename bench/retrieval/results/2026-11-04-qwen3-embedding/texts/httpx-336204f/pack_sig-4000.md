## Task
# httpx-336204f · httpx-336204f · open
Goal: Display proxy protocol scheme on error (#3571)
Workspace: /root/bench/ws/rebf815-httpx-336204f/httpx-336204f
Changed (0): nothing yet

## Map
- repo map (125 files):
  .github/ (8 files)
    CONTRIBUTING.md
    FUNDING.yml
    ISSUE_TEMPLATE/ (2 files)
    PULL_REQUEST_TEMPLATE.md
    dependabot.yml
    workflows/ (2 files)
  .gitignore
  CHANGELOG.md
  LICENSE.md
  README.md
  docs/ (36 files)
    CNAME
    advanced/ (10 files)
    api.md
    async.md
    code_of_conduct.md
    compatibility.md
    contributing.md
    css/ (1 files)
    environment_variables.md
    exceptions.md
    http2.md
    img/ (10 files)
    index.md
    logging.md
    overrides/ (1 files)
    quickstart.md
    third_party_packages.md
    troubleshooting.md
  httpx/ (24 files)
    __init__.py
    __version__.py
    _api.py
    _auth.py
    _client.py
    _config.py
    _content.py
    _decoders.py
    _exceptions.py
    _main.py
    _models.py
    _multipart.py
    _status_codes.py
    _transports/ (6 files)
    _types.py
    _urlparse.py
    _urls.py
    _utils.py
    py.typed
  mkdocs.yml
  pyproject.toml
  requirements.txt
  scripts/ (10 files)
    build
    check
    clean
    coverage
    docs
    install
    lint
    publish
    sync-version
    test
  tests/ (40 files)
    __init__.py
    client/ (11 files)
    common.py
    concurrency.py
    conftest.py
    fixtures/ (2 files)
    models/ (9 files)
    test_api.py
    test_asgi.py
    test_auth.py
    test_config.py
    test_content.py
    test_decoders.py
    test_exceptions.py
    test_exported_members.py
    test_main.py
    test_multipart.py
    test_status_codes.py
    test_timeouts.py
    test_utils.py
    test_wsgi.py

## Code
- sym:56022 Proxy (httpx/_config.py:201-243)
  class Proxy:
- sym:56114 RemoteProtocolError (httpx/_exceptions.py:232-237)
  class RemoteProtocolError(ProtocolError):
- sym:56023 Proxy.__init__ (httpx/_config.py:202-224)
  def __init__( self, url: URL | str, *, ssl_context: ssl.SSLContext | None = None, auth: tuple[str, str] | None = None, headers: HeaderTypes | None = None, ) -> None:
- sym:56112 ProtocolError (httpx/_exceptions.py:216-219)
  class ProtocolError(TransportError):
- sym:56113 LocalProtocolError (httpx/_exceptions.py:222-229)
  class LocalProtocolError(ProtocolError):
- sym:56427 get_environment_proxies (httpx/_utils.py:30-76)
  def get_environment_proxies() -> dict[str, str | None]:
- sym:56307 _load_httpcore_exceptions (httpx/_transports/default.py:74-92)
  def _load_httpcore_exceptions() -> dict[type[Exception], type[httpx.HTTPError]]:
- sym:56110 ProxyError (httpx/_exceptions.py:202-205)
  class ProxyError(TransportError):
- sym:56111 UnsupportedProtocol (httpx/_exceptions.py:208-213)
  class UnsupportedProtocol(TransportError):
- sym:56314 HTTPTransport.__init__ (httpx/_transports/default.py:136-215)
  def __init__( self, verify: ssl.SSLContext | str | bool = True, cert: CertTypes | None = None, trust_env: bool = True, http1: bool = True, http2: bool = False, limits: Limits = DEFAULT_LIMITS, proxy: ProxyTypes | None = None, uds: str | None = None, local_address: str | None = None, retries: int = 0, socket_options: typing.Iterable[SOCKET_OPTION] | None = None, ) -> None:
- sym:56433 URLPattern (httpx/_utils.py:120-226)
  class URLPattern:
- sym:56313 HTTPTransport (httpx/_transports/default.py:135-262)
  class HTTPTransport(BaseTransport):
- sym:56324 AsyncHTTPTransport.__init__ (httpx/_transports/default.py:280-359)
  def __init__( self, verify: ssl.SSLContext | str | bool = True, cert: CertTypes | None = None, trust_env: bool = True, http1: bool = True, http2: bool = False, limits: Limits = DEFAULT_LIMITS, proxy: ProxyTypes | None = None, uds: str | None = None, local_address: str | None = None, retries: int = 0, socket_options: typing.Iterable[SOCKET_OPTION] | None = None, ) -> None:
- sym:56025 Proxy.__repr__ (httpx/_config.py:235-243)
  def __repr__(self) -> str: # The authentication is represented with the password component masked.
- sym:56323 AsyncHTTPTransport (httpx/_transports/default.py:279-406)
  class AsyncHTTPTransport(AsyncBaseTransport):
- sym:55940 BaseClient._get_proxy_map (httpx/_client.py:239-251)
  def _get_proxy_map( self, proxy: ProxyTypes | None, allow_env_proxies: bool ) -> dict[str, Proxy | None]:
- sym:56609 test_unsupported_proxy_scheme (tests/client/test_proxies.py:123-125)
  def test_unsupported_proxy_scheme():
- sym:56946 test_invalid_proxy_scheme (tests/test_config.py:182-184)
  def test_invalid_proxy_scheme():
- sym:55888 get (httpx/_api.py:174-207)
  def get( url: URL | str, *, params: QueryParamTypes | None = None, headers: HeaderTypes | None = None, cookies: CookieTypes | None = None, auth: AuthTypes | None = None, proxy: ProxyTypes | None = None, follow_redirects: bool = False, verify: ssl.SSLContext | str | bool = True, timeout: TimeoutTypes = DEFAULT_TIMEOUT_CONFIG, trust_env: bool = True, ) -> Response:
- sym:55945 BaseClient.auth (httpx/_client.py:273-281)
  def auth(self) -> Auth | None:
- sym:55949 BaseClient.headers (httpx/_client.py:299-303)
  def headers(self) -> Headers:
- sym:55979 Client.get (httpx/_client.py:1036-1063)
  def get( self, url: URL | str, *, params: QueryParamTypes | None = None, headers: HeaderTypes | None = None, cookies: CookieTypes | None = None, auth: AuthTypes | UseClientDefault | None = USE_CLIENT_DEFAULT, follow_redirects: bool | UseClientDefault = USE_CLIENT_DEFAULT, timeout: TimeoutTypes | UseClientDefault = USE_CLIENT_DEFAULT, extensions: RequestExtensions | None = None, ) -> Response:
- sym:56000 AsyncClient.get (httpx/_client.py:1751-1778)
  async def get( self, url: URL | str, *, params: QueryParamTypes | None = None, headers: HeaderTypes | None = None, cookies: CookieTypes | None = None, auth: AuthTypes | UseClientDefault | None = USE_CLIENT_DEFAULT, follow_redirects: bool | UseClientDefault = USE_CLIENT_DEFAULT, timeout: TimeoutTypes | UseClientDefault = USE_CLIENT_DEFAULT, extensions: RequestExtensions | None = None, ) -> Response:
- sym:56157 Headers (httpx/_models.py:139-379)
  class Headers(typing.MutableMapping[str, str]):
- sym:56166 Headers.get (httpx/_models.py:242-250)
  def get(self, key: str, default: typing.Any = None) -> typing.Any:
- sym:56196 Response.url (httpx/_models.py:629-633)
  def url(self) -> URL:
- sym:56605 PROXY_URL (tests/client/test_proxies.py:31-31)
  PROXY_URL = "http://[::1]"
- sym:56647 test_redirect_custom_scheme (tests/client/test_redirects.py:434-438)
  def test_redirect_custom_scheme():
- sym:56944 test_proxy_from_url (tests/test_config.py:164-170)
  def test_proxy_from_url():
- sym:56512 test_sync_digest_auth_raises_protocol_error_on_malformed_header (tests/client/test_auth.py:623-632)
  def test_sync_digest_auth_raises_protocol_error_on_malformed_header( auth_header: str, ) -> None:
- sym:56945 test_proxy_with_auth_from_url (tests/test_config.py:173-179)
  def test_proxy_with_auth_from_url():
- sym:56511 test_async_digest_auth_raises_protocol_error_on_malformed_header (tests/client/test_auth.py:604-613)
  async def test_async_digest_auth_raises_protocol_error_on_malformed_header( auth_header: str, ) -> None:

---
tokens: 2065/4000
