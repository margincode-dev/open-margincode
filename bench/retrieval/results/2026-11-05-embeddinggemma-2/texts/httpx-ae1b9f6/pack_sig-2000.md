## Task
# httpx-ae1b9f6 · httpx-ae1b9f6 · open
Goal: Expose `FunctionAuth` in `__all__` (#3699)
Workspace: /root/bench/av_gemma/ws/rec728b-httpx-ae1b9f6/httpx-ae1b9f6
Changed (0): nothing yet

## Map
- httpx/_auth.py
- httpx/_client.py
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
- sym:54717 FunctionAuth.__init__ (httpx/_auth.py:119-120)
  def __init__(self, func: typing.Callable[[Request], Request]) -> None:
- sym:54777 BaseClient._build_auth (httpx/_client.py:445-455)
  def _build_auth(self, auth: AuthTypes | None) -> Auth | None:
- sym:54727 DigestAuth (httpx/_auth.py:175-340)
  class DigestAuth(Auth):
- sym:54778 BaseClient._build_request_auth (httpx/_client.py:457-473)
  def _build_request_auth( self, request: Request, auth: AuthTypes | UseClientDefault | None = USE_CLIENT_DEFAULT, ) -> Auth:
- sym:54718 FunctionAuth.auth_flow (httpx/_auth.py:122-123)
  def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
- sym:54951 print_help (httpx/_main.py:26-100)
  def print_help() -> None:
- sym:54712 Auth (httpx/_auth.py:22-110)
  class Auth:
- sym:54719 BasicAuth (httpx/_auth.py:126-142)
  class BasicAuth(Auth):
- sym:54723 NetRCAuth (httpx/_auth.py:145-172)
  class NetRCAuth(Auth):
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
- sym:55301 SyncOrAsyncAuth.sync_auth_flow (tests/client/test_auth.py:147-152)
  def sync_auth_flow( self, request: httpx.Request ) -> typing.Generator[httpx.Request, httpx.Response, None]:
- sym:55288 App.__call__ (tests/client/test_auth.py:32-35)
  def __call__(self, request: httpx.Request) -> httpx.Response:
- sym:55814 test_all_imports_are_exported (tests/test_exported_members.py:4-13)
  def test_all_imports_are_exported() -> None:
- sym:55317 test_digest_auth_returns_no_auth_if_alternate_auth_scheme (tests/client/test_auth.py:370-381)
  def test_digest_auth_returns_no_auth_if_alternate_auth_scheme() -> None:
- sym:55332 ConsumeBodyTransport (tests/client/test_auth.py:688-692)
  class ConsumeBodyTransport(httpx.MockTransport):
- sym:55302 SyncOrAsyncAuth.async_auth_flow (tests/client/test_auth.py:154-159)
  async def async_auth_flow( self, request: httpx.Request ) -> typing.AsyncGenerator[httpx.Request, httpx.Response]:
- sym:55324 test_digest_auth_qop_must_be_auth_or_auth_int (tests/client/test_auth.py:516-523)
  async def test_digest_auth_qop_must_be_auth_or_auth_int() -> None:
- sym:55293 RepeatAuth (tests/client/test_auth.py:87-112)
  class RepeatAuth(httpx.Auth):
- sym:55296 ResponseBodyAuth (tests/client/test_auth.py:115-134)
  class ResponseBodyAuth(httpx.Auth):
- sym:55299 SyncOrAsyncAuth (tests/client/test_auth.py:137-159)
  class SyncOrAsyncAuth(httpx.Auth):

---
tokens: 1690/2000
