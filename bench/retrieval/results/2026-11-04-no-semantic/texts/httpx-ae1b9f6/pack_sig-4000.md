## Task
# httpx-ae1b9f6 · httpx-ae1b9f6-2 · open
Goal: Expose `FunctionAuth` in `__all__` (#3699)
Workspace: /root/bench/ws/rebf815-httpx-ae1b9f6/httpx-ae1b9f6-2
Changed (0): nothing yet

## Map
- httpx/_auth.py
  L22: class Auth:
  L38: def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
  L62: def sync_auth_flow( self, request: Request ) -> typing.Generator[Request, Response, None]:
  L87: async def async_auth_flow( self, request: Request ) -> typing.AsyncGenerator[Request, Response]:
  L113: class FunctionAuth(Auth):
  L119: def __init__(self, func: typing.Callable[[Request], Request]) -> None:
  L122: def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
  L126: class BasicAuth(Auth):
  L132: def __init__(self, username: str | bytes, password: str | bytes) -> None:
  L135: def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
  L139: def _build_auth_header(self, username: str | bytes, password: str | bytes) -> str:
  L145: class NetRCAuth(Auth):
  L150: def __init__(self, file: str | None = None) -> None: # Lazily import 'netrc'. # There's no need for us to load this module unless 'NetRCAuth' is being used.
  L157: def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
  L169: def _build_auth_header(self, username: str | bytes, password: str | bytes) -> str:
  L175: class DigestAuth(Auth):
  L187: def __init__(self, username: str | bytes, password: str | bytes) -> None:
  L193: def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
  L224: def _parse_challenge( self, request: Request, response: Response, auth_header: str ) -> _DigestAuthChallenge:
  L255: def _build_auth_header( self, request: Request, challenge: _DigestAuthChallenge ) -> str:
  L303: def _get_client_nonce(self, nonce_count: int, nonce: bytes) -> bytes:
  L311: def _get_header_value(self, header_fields: dict[str, bytes]) -> str:
  L329: def _resolve_qop(self, qop: bytes | None, request: Request) -> bytes | None:
  L343: class _DigestAuthChallenge(typing.NamedTuple):
- httpx/_client.py
  L62: def _is_https_redirect(url: URL, location: URL) -> bool:
  L77: def _port_or_default(url: URL) -> int | None:
  L83: def _same_origin(url: URL, other: URL) -> bool:
  L94: class UseClientDefault:
  L114: USE_CLIENT_DEFAULT = UseClientDefault()
  L119: USER_AGENT = f"python-httpx/{__version__}"
  L120: ACCEPT_ENCODING = ", ".join(
  L125: class ClientState(enum.Enum): # UNOPENED: # The client has been instantiated, but has not been used to send a request, # or been opened by entering the context of a `with` block.
  L139: class BoundSyncStream(SyncByteStream):
  L145: def __init__( self, stream: SyncByteStream, response: Response, start: float ) -> None:
  L152: def __iter__(self) -> typing.Iterator[bytes]:
  L156: def close(self) -> None:
  L162: class BoundAsyncStream(AsyncByteStream):
  L168: def __init__( self, stream: AsyncByteStream, response: Response, start: float ) -> None:
  L175: async def __aiter__(self) -> typing.AsyncIterator[bytes]:
  L179: async def aclose(self) -> None:
  L188: class BaseClient:
  L189: def __init__( self, *, auth: AuthTypes | None = None, params: QueryParamTypes | None = None, headers: HeaderTypes | None = None, cookies: CookieTypes | None = None, timeout: TimeoutTypes = DEFAULT_TIMEOUT_CONFIG, follow_redirects: bool = False, max_redirects: int = DEFAULT_MAX_REDIRECTS, event_hooks: None | (typing.Mapping[str, list[EventHook]]) = None, base_url: URL | str = "", trust_env: bool = True, default_encoding: str | typing.Callable[[bytes], str] = "utf-8", ) -> None:
  L224: def is_closed(self) -> bool:
  L231: def trust_env(self) -> bool:
  L234: def _enforce_trailing_slash(self, url: URL) -> URL:
  L239: def _get_proxy_map( self, proxy: ProxyTypes | None, allow_env_proxies: bool ) -> dict[str, Proxy | None]:
  L254: def timeout(self) -> Timeout:
  L262: def event_hooks(self) -> dict[str, list[EventHook]]:
  L273: def auth(self) -> Auth | None:
  L288: def base_url(self) -> URL:
  L299: def headers(self) -> Headers:
  L319: def cookies(self) -> Cookies:
  L330: def params(self) -> QueryParams:
  L340: def build_request( self, method: str, url: URL | str, *, content: RequestContent | None = None, data: RequestData | None = None, files: RequestFiles | None = None, json: typing.Any | None = None, params: QueryParamTypes | None = None, headers: HeaderTypes | None = None, cookies: CookieTypes | None = None, timeout: TimeoutTypes | UseClientDefault = USE_CLIENT_DEFAULT, extensions: RequestExtensions | None = None, ) -> Request:
- httpx/_urls.py
  L15: class URL:
  L77: def __init__(self, url: URL | str = "", **kwargs: typing.Any) -> None:
  L127: def scheme(self) -> str:
  L135: def raw_scheme(self) -> bytes:
  L143: def userinfo(self) -> bytes:
  L151: def username(self) -> str:
  L160: def password(self) -> str:
  L169: def host(self) -> str:
  L196: def raw_host(self) -> bytes:
  L218: def port(self) -> int | None:
  L234: def netloc(self) -> bytes:
  L245: def path(self) -> str:
  L258: def query(self) -> bytes:
  L275: def params(self) -> QueryParams:
  L283: def raw_path(self) -> bytes:
  L300: def fragment(self) -> str:
  L308: def is_absolute_url(self) -> bool:
  L320: def is_relative_url(self) -> bool:
  L327: def copy_with(self, **kwargs: typing.Any) -> URL:
  L342: def copy_set_param(self, key: str, value: typing.Any = None) -> URL:
  L345: def copy_add_param(self, key: str, value: typing.Any = None) -> URL:
  L348: def copy_remove_param(self, key: str) -> URL:
  L351: def copy_merge_params(self, params: QueryParamTypes) -> URL:
  L354: def join(self, url: URL | str) -> URL:
  L368: def __hash__(self) -> int:
  L371: def __eq__(self, other: typing.Any) -> bool:
  L374: def __str__(self) -> str:
  L377: def __repr__(self) -> str:
  L404: def raw(self) -> tuple[bytes, bytes, int, bytes]: # pragma: nocover
  L420: class QueryParams(typing.Mapping[str, str]):
- tests/client/test_auth.py
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
- sym:54716 FunctionAuth (httpx/_auth.py:113-123)
  class FunctionAuth(Auth):
- sym:54777 BaseClient._build_auth (httpx/_client.py:445-455)
  def _build_auth(self, auth: AuthTypes | None) -> Auth | None:
- sym:54778 BaseClient._build_request_auth (httpx/_client.py:457-473)
  def _build_request_auth( self, request: Request, auth: AuthTypes | UseClientDefault | None = USE_CLIENT_DEFAULT, ) -> Auth:
- sym:54793 Client._send_handling_auth (httpx/_client.py:930-962)
  def _send_handling_auth( self, request: Request, auth: Auth, follow_redirects: bool, history: list[Response], ) -> Response:
- sym:54718 FunctionAuth.auth_flow (httpx/_auth.py:122-123)
  def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
- sym:54717 FunctionAuth.__init__ (httpx/_auth.py:119-120)
  def __init__(self, func: typing.Callable[[Request], Request]) -> None:
- sym:54814 AsyncClient._send_handling_auth (httpx/_client.py:1645-1677)
  async def _send_handling_auth( self, request: Request, auth: Auth, follow_redirects: bool, history: list[Response], ) -> Response:
- sym:54951 print_help (httpx/_main.py:26-100)
  def print_help() -> None:
- sym:54712 Auth (httpx/_auth.py:22-110)
  class Auth:
- sym:54719 BasicAuth (httpx/_auth.py:126-142)
  class BasicAuth(Auth):
- sym:54723 NetRCAuth (httpx/_auth.py:145-172)
  class NetRCAuth(Auth):
- sym:54727 DigestAuth (httpx/_auth.py:175-340)
  class DigestAuth(Auth):
- sym:54753 BaseClient.__init__ (httpx/_client.py:189-221)
  def __init__( self, *, auth: AuthTypes | None = None, params: QueryParamTypes | None = None, headers: HeaderTypes | None = None, cookies: CookieTypes | None = None, timeout: TimeoutTypes = DEFAULT_TIMEOUT_CONFIG, follow_redirects: bool = False, max_redirects: int = DEFAULT_MAX_REDIRECTS, event_hooks: None | (typing.Mapping[str, list[EventHook]]) = None, base_url: URL | str = "", trust_env: bool = True, default_encoding: str | typing.Callable[[bytes], str] = "utf-8", ) -> None:
- sym:54762 BaseClient.auth (httpx/_client.py:273-281)
  def auth(self) -> Auth | None:
- sym:55196 URL.username (httpx/_urls.py:151-157)
  def username(self) -> str:
- sym:55197 URL.password (httpx/_urls.py:160-166)
  def password(self) -> str:
- sym:55317 test_digest_auth_returns_no_auth_if_alternate_auth_scheme (tests/client/test_auth.py:370-381)
  def test_digest_auth_returns_no_auth_if_alternate_auth_scheme() -> None:
- sym:55324 test_digest_auth_qop_must_be_auth_or_auth_int (tests/client/test_auth.py:516-523)
  async def test_digest_auth_qop_must_be_auth_or_auth_int() -> None:
- sym:55318 test_digest_auth_200_response_including_digest_auth_header (tests/client/test_auth.py:385-396)
  async def test_digest_auth_200_response_including_digest_auth_header() -> None:
- sym:55319 test_digest_auth_401_response_without_digest_auth_header (tests/client/test_auth.py:400-410)
  async def test_digest_auth_401_response_without_digest_auth_header() -> None:
- sym:55322 test_digest_auth_qop_including_spaces_and_auth_returns_auth (tests/client/test_auth.py:492-501)
  async def test_digest_auth_qop_including_spaces_and_auth_returns_auth(qop: str) -> None:
- sym:55323 test_digest_auth_qop_auth_int_not_implemented (tests/client/test_auth.py:505-512)
  async def test_digest_auth_qop_auth_int_not_implemented() -> None:
- sym:55329 test_sync_digest_auth_raises_protocol_error_on_malformed_header (tests/client/test_auth.py:623-632)
  def test_sync_digest_auth_raises_protocol_error_on_malformed_header( auth_header: str, ) -> None:
- sym:55328 test_async_digest_auth_raises_protocol_error_on_malformed_header (tests/client/test_auth.py:604-613)
  async def test_async_digest_auth_raises_protocol_error_on_malformed_header( auth_header: str, ) -> None:
- sym:55293 RepeatAuth (tests/client/test_auth.py:87-112)
  class RepeatAuth(httpx.Auth):
- sym:55296 ResponseBodyAuth (tests/client/test_auth.py:115-134)
  class ResponseBodyAuth(httpx.Auth):
- sym:55299 SyncOrAsyncAuth (tests/client/test_auth.py:137-159)
  class SyncOrAsyncAuth(httpx.Auth):

---
tokens: 3382/4000
