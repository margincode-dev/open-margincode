- FunctionAuth (httpx/_auth.py:113-123)
```python
class FunctionAuth(Auth):
    """
    Allows the 'auth' argument to be passed as a simple callable function,
    that takes the request, and returns a new, modified request.
    """

    def __init__(self, func: typing.Callable[[Request], Request]) -> None:
        self._func = func

    def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
        yield self._func(request)
```
- BaseClient._build_auth (httpx/_client.py:445-455)
```python
    def _build_auth(self, auth: AuthTypes | None) -> Auth | None:
        if auth is None:
            return None
        elif isinstance(auth, tuple):
            return BasicAuth(username=auth[0], password=auth[1])
        elif isinstance(auth, Auth):
            return auth
        elif callable(auth):
            return FunctionAuth(func=auth)
        else:
            raise TypeError(f'Invalid "auth" argument: {auth!r}')
```
- BaseClient.auth (httpx/_client.py:284-285)
```python
    def auth(self, auth: AuthTypes) -> None:
        self._auth = self._build_auth(auth)
```
- BaseClient._build_request_auth (httpx/_client.py:457-473)
```python
    def _build_request_auth(
        self,
        request: Request,
        auth: AuthTypes | UseClientDefault | None = USE_CLIENT_DEFAULT,
    ) -> Auth:
        auth = (
            self._auth if isinstance(auth, UseClientDefault) else self._build_auth(auth)
        )

        if auth is not None:
            return auth

        username, password = request.url.username, request.url.password
        if username or password:
            return BasicAuth(username=username, password=password)

        return Auth()
```
- test_digest_auth_returns_no_auth_if_alternate_auth_scheme (tests/client/test_auth.py:370-381)
```python
def test_digest_auth_returns_no_auth_if_alternate_auth_scheme() -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    auth_header = "Token ..."
    app = App(auth_header=auth_header, status_code=401)

    client = httpx.Client(transport=httpx.MockTransport(app))
    response = client.get(url, auth=auth)

    assert response.status_code == 401
    assert response.json() == {"auth": None}
    assert len(response.history) == 0
```
- test_digest_auth_qop_must_be_auth_or_auth_int (tests/client/test_auth.py:516-523)
```python
async def test_digest_auth_qop_must_be_auth_or_auth_int() -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    app = DigestApp(qop="not-auth")

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        with pytest.raises(httpx.ProtocolError):
            await client.get(url, auth=auth)
```
- test_digest_auth_200_response_including_digest_auth_header (tests/client/test_auth.py:385-396)
```python
async def test_digest_auth_200_response_including_digest_auth_header() -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    auth_header = 'Digest realm="realm@host.com",qop="auth",nonce="abc",opaque="xyz"'
    app = App(auth_header=auth_header, status_code=200)

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
