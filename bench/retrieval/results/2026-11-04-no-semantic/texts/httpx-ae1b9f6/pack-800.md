## Task
# httpx-ae1b9f6 · httpx-ae1b9f6-2 · open
Goal: Expose `FunctionAuth` in `__all__` (#3699)

## Map
- httpx/_auth.py
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
- sym:54778 BaseClient._build_request_auth (httpx/_client.py:457-473)
  def _build_request_auth( self, request: Request, auth: AuthTypes | UseClientDefault | None = USE_CLIENT_DEFAULT, ) -> Auth:
- sym:54718 FunctionAuth.auth_flow (httpx/_auth.py:122-123)
  def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
- sym:54717 FunctionAuth.__init__ (httpx/_auth.py:119-120)
- sym:54951 print_help (httpx/_main.py:26-100)
  def print_help() -> None:
- sym:54712 Auth (httpx/_auth.py:22-110)
  class Auth:
- sym:54719 BasicAuth (httpx/_auth.py:126-142)
- sym:54723 NetRCAuth (httpx/_auth.py:145-172)
- sym:54727 DigestAuth (httpx/_auth.py:175-340)
- sym:54753 BaseClient.__init__ (httpx/_client.py:189-221)
- sym:54762 BaseClient.auth (httpx/_client.py:273-281)
- sym:54793 Client._send_handling_auth (httpx/_client.py:930-962)
- sym:54814 AsyncClient._send_handling_auth (httpx/_client.py:1645-1677)
- sym:55196 URL.username (httpx/_urls.py:151-157)
- sym:55197 URL.password (httpx/_urls.py:160-166)
- sym:55317 test_digest_auth_returns_no_auth_if_alternate_auth_scheme (tests/client/test_auth.py:370-381)
- sym:55324 test_digest_auth_qop_must_be_auth_or_auth_int (tests/client/test_auth.py:516-523)
- sym:55293 RepeatAuth (tests/client/test_auth.py:87-112)
- sym:55296 ResponseBodyAuth (tests/client/test_auth.py:115-134)
- sym:55299 SyncOrAsyncAuth (tests/client/test_auth.py:137-159)

---
tokens: 681/800
