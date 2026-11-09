## Task
# httpx-336204f · httpx-336204f · open
Goal: Display proxy protocol scheme on error (#3571)
Workspace: /root/bench/av_gemma/ws/rec728b-httpx-336204f/httpx-336204f
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
- sym:56313 HTTPTransport (httpx/_transports/default.py:135-262)
  class HTTPTransport(BaseTransport):
- sym:56314 HTTPTransport.__init__ (httpx/_transports/default.py:136-215)
  def __init__( self, verify: ssl.SSLContext | str | bool = True, cert: CertTypes | None = None, trust_env: bool = True, http1: bool = True, http2: bool = False, limits: Limits = DEFAULT_LIMITS, proxy: ProxyTypes | None = None, uds: str | None = None, local_address: str | None = None, retries: int = 0, socket_options: typing.Iterable[SOCKET_OPTION] | None = None, ) -> None:
- sym:56323 AsyncHTTPTransport (httpx/_transports/default.py:279-406)
  class AsyncHTTPTransport(AsyncBaseTransport):
- sym:56110 ProxyError (httpx/_exceptions.py:202-205)
  class ProxyError(TransportError):
- sym:56023 Proxy.__init__ (httpx/_config.py:202-224)
  def __init__( self, url: URL | str, *, ssl_context: ssl.SSLContext | None = None, auth: tuple[str, str] | None = None, headers: HeaderTypes | None = None, ) -> None:
- sym:56427 get_environment_proxies (httpx/_utils.py:30-76)
  def get_environment_proxies() -> dict[str, str | None]:
- sym:56113 LocalProtocolError (httpx/_exceptions.py:222-229)
  class LocalProtocolError(ProtocolError):
- sym:56307 _load_httpcore_exceptions (httpx/_transports/default.py:74-92)
  def _load_httpcore_exceptions() -> dict[type[Exception], type[httpx.HTTPError]]:
- sym:55940 BaseClient._get_proxy_map (httpx/_client.py:239-251)
  def _get_proxy_map( self, proxy: ProxyTypes | None, allow_env_proxies: bool ) -> dict[str, Proxy | None]:
- sym:56144 trace (httpx/_main.py:212-248)
  def trace( name: str, info: typing.Mapping[str, typing.Any], verbose: bool = False ) -> None:
- sym:56433 URLPattern (httpx/_utils.py:120-226)
  class URLPattern:
- sym:56150 SENSITIVE_HEADERS (httpx/_models.py:53-53)
  SENSITIVE_HEADERS = {"authorization", "proxy-authorization"}
- sym:56324 AsyncHTTPTransport.__init__ (httpx/_transports/default.py:280-359)
  def __init__( self, verify: ssl.SSLContext | str | bool = True, cert: CertTypes | None = None, trust_env: bool = True, http1: bool = True, http2: bool = False, limits: Limits = DEFAULT_LIMITS, proxy: ProxyTypes | None = None, uds: str | None = None, local_address: str | None = None, retries: int = 0, socket_options: typing.Iterable[SOCKET_OPTION] | None = None, ) -> None:
- sym:55925 ACCEPT_ENCODING (httpx/_client.py:120-122)
  ACCEPT_ENCODING = ", ".join(
- sym:56112 ProtocolError (httpx/_exceptions.py:216-219)
  class ProtocolError(TransportError):
- sym:56138 print_request_headers (httpx/_main.py:147-153)
  def print_request_headers(request: httpcore.Request, http2: bool = False) -> None:
- sym:56114 RemoteProtocolError (httpx/_exceptions.py:232-237)
  class RemoteProtocolError(ProtocolError):
- sym:55935 BaseClient (httpx/_client.py:188-591)
  class BaseClient:
- sym:56139 print_response_headers (httpx/_main.py:156-167)
  def print_response_headers( http_version: bytes, status: int, reason_phrase: bytes | None, headers: list[tuple[bytes, bytes]], ) -> None:
- sym:56011 UNSET (httpx/_config.py:20-20)
  UNSET = UnsetType()
- sym:56434 URLPattern.__init__ (httpx/_utils.py:162-190)
  def __init__(self, pattern: str) -> None:
- sym:56111 UnsupportedProtocol (httpx/_exceptions.py:208-213)
  class UnsupportedProtocol(TransportError):
- sym:55971 Client._init_proxy_transport (httpx/_client.py:740-758)
  def _init_proxy_transport( self, proxy: Proxy, verify: ssl.SSLContext | str | bool = True, cert: CertTypes | None = None, trust_env: bool = True, http1: bool = True, http2: bool = False, limits: Limits = DEFAULT_LIMITS, ) -> BaseTransport:
- sym:56149 main (httpx/_main.py:452-506)
  def main( url: str, method: str, params: list[tuple[str, str]], content: str, data: list[tuple[str, str]], files: list[tuple[str, click.File]], json: str, headers: list[tuple[str, str]], cookies: list[tuple[str, str]], auth: tuple[str, str] | None, proxy: str, timeout: float, follow_redirects: bool, verify: bool, http2: bool, download: typing.BinaryIO | None, verbose: bool, ) -> None:
- sym:55992 AsyncClient._init_proxy_transport (httpx/_client.py:1454-1472)
  def _init_proxy_transport( self, proxy: Proxy, verify: ssl.SSLContext | str | bool = True, cert: CertTypes | None = None, trust_env: bool = True, http1: bool = True, http2: bool = False, limits: Limits = DEFAULT_LIMITS, ) -> AsyncBaseTransport:
- sym:56306 HTTPCORE_EXC_MAP (httpx/_transports/default.py:71-71)
  HTTPCORE_EXC_MAP: dict[type[Exception], type[httpx.HTTPError]] = {}
- sym:56140 print_response (httpx/_main.py:170-186)
  def print_response(response: Response) -> None:
- sym:55923 USE_CLIENT_DEFAULT (httpx/_client.py:114-114)
  USE_CLIENT_DEFAULT = UseClientDefault()
- sym:55913 DigestAuth._parse_challenge (httpx/_auth.py:224-253)
  def _parse_challenge( self, request: Request, response: Response, auth_header: str ) -> _DigestAuthChallenge:
- sym:56349 UNRESERVED_CHARACTERS (httpx/_urlparse.py:32-34)
  UNRESERVED_CHARACTERS = (
- sym:56341 WSGITransport.handle_request (httpx/_transports/wsgi.py:91-149)
  def handle_request(self, request: Request) -> Response:
- sym:56350 SUB_DELIMS (httpx/_urlparse.py:35-35)
  SUB_DELIMS = "!$&'()*+,;="
- sym:55924 USER_AGENT (httpx/_client.py:119-119)
  USER_AGENT = f"python-httpx/{__version__}"
- sym:56021 Limits.__repr__ (httpx/_config.py:192-198)
  def __repr__(self) -> str:
- sym:56008 AsyncClient.__aenter__ (httpx/_client.py:1990-2006)
  async def __aenter__(self: U) -> U:
- sym:56946 test_invalid_proxy_scheme (tests/test_config.py:182-184)
  def test_invalid_proxy_scheme():
- sym:56609 test_unsupported_proxy_scheme (tests/client/test_proxies.py:123-125)
  def test_unsupported_proxy_scheme():
- sym:56647 test_redirect_custom_scheme (tests/client/test_redirects.py:434-438)
  def test_redirect_custom_scheme():
- sym:55938 BaseClient.trust_env (httpx/_client.py:231-232)
  def trust_env(self) -> bool:
- sym:55945 BaseClient.auth (httpx/_client.py:273-281)
  def auth(self) -> Auth | None:
- sym:55949 BaseClient.headers (httpx/_client.py:299-303)
  def headers(self) -> Headers:
- sym:56012 create_ssl_context (httpx/_config.py:23-69)
  def create_ssl_context( verify: ssl.SSLContext | str | bool = True, cert: CertTypes | None = None, trust_env: bool = True, ) -> ssl.SSLContext:
- sym:56018 Limits (httpx/_config.py:159-198)
  class Limits:
- sym:56024 Proxy.raw_auth (httpx/_config.py:227-233)
  def raw_auth(self) -> tuple[bytes, bytes] | None: # The proxy authentication as raw bytes.
- sym:56027 DEFAULT_LIMITS (httpx/_config.py:247-247)
  DEFAULT_LIMITS = Limits(max_connections=100, max_keepalive_connections=20)
- sym:56157 Headers (httpx/_models.py:139-379)
  class Headers(typing.MutableMapping[str, str]):
- sym:56161 Headers.raw (httpx/_models.py:196-200)
  def raw(self) -> list[tuple[bytes, bytes]]:
- sym:56196 Response.url (httpx/_models.py:629-633)
  def url(self) -> URL:
- sym:56305 SOCKET_OPTION (httpx/_transports/default.py:63-67)
  SOCKET_OPTION = typing.Union[
- sym:56364 ParseResult.copy_with (httpx/_urlparse.py:186-198)
  def copy_with(self, **kwargs: str | None) -> ParseResult:
- sym:56374 URL (httpx/_urls.py:15-417)
  class URL:
- sym:56376 URL.scheme (httpx/_urls.py:127-132)
  def scheme(self) -> str:
- sym:56377 URL.raw_scheme (httpx/_urls.py:135-140)
  def raw_scheme(self) -> bytes:
- sym:56379 URL.username (httpx/_urls.py:151-157)
  def username(self) -> str:
- sym:56605 PROXY_URL (tests/client/test_proxies.py:31-31)
  PROXY_URL = "http://[::1]"
- sym:57010 test_errors (tests/test_main.py:181-187)
  def test_errors():
- sym:56512 test_sync_digest_auth_raises_protocol_error_on_malformed_header (tests/client/test_auth.py:623-632)
  def test_sync_digest_auth_raises_protocol_error_on_malformed_header( auth_header: str, ) -> None:
- sym:56511 test_async_digest_auth_raises_protocol_error_on_malformed_header (tests/client/test_auth.py:604-613)
  async def test_async_digest_auth_raises_protocol_error_on_malformed_header( auth_header: str, ) -> None:
- sym:56691 test_headers_dict_repr (tests/models/test_headers.py:90-95)
  def test_headers_dict_repr():
- sym:56692 test_headers_encoding_in_repr (tests/models/test_headers.py:98-103)
  def test_headers_encoding_in_repr():
- sym:56693 test_headers_list_repr (tests/models/test_headers.py:106-113)
  def test_headers_list_repr():
- sym:56879 test_urlparse (tests/models/test_whatwg.py:23-52)
  def test_urlparse(test_case):
- sym:56611 test_for_deprecated_proxy_params (tests/client/test_proxies.py:250-257)
  def test_for_deprecated_proxy_params(proxies, is_valid):
- sym:56524 test_get_invalid_url (tests/client/test_client.py:41-44)
  def test_get_invalid_url(server, url):
- sym:57007 test_verbose (tests/test_main.py:121-142)
  def test_verbose(server):
- sym:56443 test_get_invalid_url (tests/client/test_async_client.py:33-36)
  async def test_get_invalid_url(server, url):
- sym:56617 redirects (tests/client/test_redirects.py:8-113)
  def redirects(request: httpx.Request) -> httpx.Response:
- sym:57068 test_wsgi_server_protocol (tests/test_wsgi.py:188-203)
  def test_wsgi_server_protocol():

---
tokens: 3243/8000
