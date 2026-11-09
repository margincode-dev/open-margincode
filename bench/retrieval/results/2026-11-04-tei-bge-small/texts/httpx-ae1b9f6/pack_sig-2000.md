## Task
# httpx-ae1b9f6 · httpx-ae1b9f6 · open
Goal: Expose `FunctionAuth` in `__all__` (#3699)
Workspace: /root/bench/av_tei/ws/rec09fc-httpx-ae1b9f6/httpx-ae1b9f6
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
- httpx/_urls.py
- tests/client/test_auth.py
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
- sym:54716 FunctionAuth (httpx/_auth.py:113-123)
  class FunctionAuth(Auth):
- sym:54777 BaseClient._build_auth (httpx/_client.py:445-455)
  def _build_auth(self, auth: AuthTypes | None) -> Auth | None:
- sym:54717 FunctionAuth.__init__ (httpx/_auth.py:119-120)
  def __init__(self, func: typing.Callable[[Request], Request]) -> None:
- sym:54718 FunctionAuth.auth_flow (httpx/_auth.py:122-123)
  def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
- sym:54778 BaseClient._build_request_auth (httpx/_client.py:457-473)
  def _build_request_auth( self, request: Request, auth: AuthTypes | UseClientDefault | None = USE_CLIENT_DEFAULT, ) -> Auth:
- sym:54719 BasicAuth (httpx/_auth.py:126-142)
  class BasicAuth(Auth):
- sym:54951 print_help (httpx/_main.py:26-100)
  def print_help() -> None:
- sym:55218 URL.__repr__ (httpx/_urls.py:377-401)
  def __repr__(self) -> str:
- sym:55176 AUTHORITY_REGEX (httpx/_urlparse.py:125-134)
  AUTHORITY_REGEX = re.compile(
- sym:54842 Proxy.__repr__ (httpx/_config.py:235-243)
  def __repr__(self) -> str: # The authentication is represented with the password component masked.
- sym:54712 Auth (httpx/_auth.py:22-110)
  class Auth:
- sym:54723 NetRCAuth (httpx/_auth.py:145-172)
  class NetRCAuth(Auth):
- sym:54727 DigestAuth (httpx/_auth.py:175-340)
  class DigestAuth(Auth):
- sym:54753 BaseClient.__init__ (httpx/_client.py:189-221)
  def __init__( self, *, auth: AuthTypes | None = None, params: QueryParamTypes | None = None, headers: HeaderTypes | None = None, cookies: CookieTypes | None = None, timeout: TimeoutTypes = DEFAULT_TIMEOUT_CONFIG, follow_redirects: bool = False, max_redirects: int = DEFAULT_MAX_REDIRECTS, event_hooks: None | (typing.Mapping[str, list[EventHook]]) = None, base_url: URL | str = "", trust_env: bool = True, default_encoding: str | typing.Callable[[bytes], str] = "utf-8", ) -> None:
- sym:54762 BaseClient.auth (httpx/_client.py:273-281)
  def auth(self) -> Auth | None:
- sym:54793 Client._send_handling_auth (httpx/_client.py:930-962)
  def _send_handling_auth( self, request: Request, auth: Auth, follow_redirects: bool, history: list[Response], ) -> Response:
- sym:54814 AsyncClient._send_handling_auth (httpx/_client.py:1645-1677)
  async def _send_handling_auth( self, request: Request, auth: Auth, follow_redirects: bool, history: list[Response], ) -> Response:
- sym:55196 URL.username (httpx/_urls.py:151-157)
  def username(self) -> str:
- sym:55197 URL.password (httpx/_urls.py:160-166)
  def password(self) -> str:
- sym:55317 test_digest_auth_returns_no_auth_if_alternate_auth_scheme (tests/client/test_auth.py:370-381)
  def test_digest_auth_returns_no_auth_if_alternate_auth_scheme() -> None:
- sym:55324 test_digest_auth_qop_must_be_auth_or_auth_int (tests/client/test_auth.py:516-523)
  async def test_digest_auth_qop_must_be_auth_or_auth_int() -> None:
- sym:55293 RepeatAuth (tests/client/test_auth.py:87-112)
  class RepeatAuth(httpx.Auth):
- sym:55296 ResponseBodyAuth (tests/client/test_auth.py:115-134)
  class ResponseBodyAuth(httpx.Auth):
- sym:55299 SyncOrAsyncAuth (tests/client/test_auth.py:137-159)
  class SyncOrAsyncAuth(httpx.Auth):

---
tokens: 1679/2000
