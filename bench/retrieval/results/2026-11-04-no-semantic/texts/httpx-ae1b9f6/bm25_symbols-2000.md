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
        response = await client.get(url, auth=auth)

    assert response.status_code == 200
    assert response.json() == {"auth": None}
    assert len(response.history) == 0
```
- test_sync_auth (tests/client/test_auth.py:760-772)
```python
def test_sync_auth() -> None:
    """
    Test that we can use an auth implementation specific to the sync case.
    """
    url = "https://example.org/"
    auth = SyncOrAsyncAuth()
    app = App()

    with httpx.Client(transport=httpx.MockTransport(app)) as client:
        response = client.get(url, auth=auth)

    assert response.status_code == 200
    assert response.json() == {"auth": "sync-auth"}
```
- NetRCAuth.auth_flow (httpx/_auth.py:157-167)
```python
    def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
        auth_info = self._netrc_info.authenticators(request.url.host)
        if auth_info is None or not auth_info[2]:
            # The netrc file did not have authentication credentials for this host.
            yield request
        else:
            # Build a basic auth header with credentials from the netrc file.
            request.headers["Authorization"] = self._build_auth_header(
                username=auth_info[0], password=auth_info[2]
            )
            yield request
```
- test_digest_auth_qop_including_spaces_and_auth_returns_auth (tests/client/test_auth.py:492-501)
```python
async def test_digest_auth_qop_including_spaces_and_auth_returns_auth(qop: str) -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    app = DigestApp(qop=qop)

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response = await client.get(url, auth=auth)

    assert response.status_code == 200
    assert len(response.history) == 1
```
- test_digest_auth_qop_auth_int_not_implemented (tests/client/test_auth.py:505-512)
```python
async def test_digest_auth_qop_auth_int_not_implemented() -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    app = DigestApp(qop="auth-int")

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        with pytest.raises(NotImplementedError):
            await client.get(url, auth=auth)
```
- test_digest_auth_401_response_without_digest_auth_header (tests/client/test_auth.py:400-410)
```python
async def test_digest_auth_401_response_without_digest_auth_header() -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    app = App(auth_header="", status_code=401)

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response = await client.get(url, auth=auth)

    assert response.status_code == 401
    assert response.json() == {"auth": None}
    assert len(response.history) == 0
```
- test_sync_digest_auth_raises_protocol_error_on_malformed_header (tests/client/test_auth.py:623-632)
```python
def test_sync_digest_auth_raises_protocol_error_on_malformed_header(
    auth_header: str,
) -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    app = App(auth_header=auth_header, status_code=401)

    with httpx.Client(transport=httpx.MockTransport(app)) as client:
        with pytest.raises(httpx.ProtocolError):
            client.get(url, auth=auth)
```
- test_sync_auth_reads_response_body (tests/client/test_auth.py:726-739)
```python
def test_sync_auth_reads_response_body() -> None:
    """
    Test that we can read the response body in an auth flow if `requires_response_body`
    is set.
    """
    url = "https://example.org/"
    auth = ResponseBodyAuth("xyz")
    app = App()

    with httpx.Client(transport=httpx.MockTransport(app)) as client:
        response = client.get(url, auth=auth)

    assert response.status_code == 200
    assert response.json() == {"auth": '{"auth":"xyz"}'}
```
- BasicAuth.auth_flow (httpx/_auth.py:135-137)
```python
    def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
        request.headers["Authorization"] = self._auth_header
        yield request
```
- test_async_digest_auth_raises_protocol_error_on_malformed_header (tests/client/test_auth.py:604-613)
```python
async def test_async_digest_auth_raises_protocol_error_on_malformed_header(
    auth_header: str,
) -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    app = App(auth_header=auth_header, status_code=401)

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        with pytest.raises(httpx.ProtocolError):
            await client.get(url, auth=auth)
```
- Proxy.__repr__ (httpx/_config.py:235-243)
```python
    def __repr__(self) -> str:
        # The authentication is represented with the password component masked.
