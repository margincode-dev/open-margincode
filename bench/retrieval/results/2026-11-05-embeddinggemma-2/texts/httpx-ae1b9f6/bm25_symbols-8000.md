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
        auth = (self.auth[0], "********") if self.auth else None

        # Build a nice concise representation.
        url_str = f"{str(self.url)!r}"
        auth_str = f", auth={auth!r}" if auth else ""
        headers_str = f", headers={dict(self.headers)!r}" if self.headers else ""
        return f"Proxy({url_str}{auth_str}{headers_str})"
```
- test_auth_disable_per_request (tests/client/test_auth.py:289-300)
```python
async def test_auth_disable_per_request() -> None:
    url = "https://example.org/"
    auth = ("user", "password123")
    app = App()

    async with httpx.AsyncClient(
        transport=httpx.MockTransport(app), auth=auth
    ) as client:
        response = await client.get(url, auth=None)

    assert response.status_code == 200
    assert response.json() == {"auth": None}
```
- test_digest_auth_returns_no_auth_if_no_digest_header_in_response (tests/client/test_auth.py:357-367)
```python
async def test_digest_auth_returns_no_auth_if_no_digest_header_in_response() -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    app = App()

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response = await client.get(url, auth=auth)

    assert response.status_code == 200
    assert response.json() == {"auth": None}
    assert len(response.history) == 0
```
- test_async_auth_reads_response_body (tests/client/test_auth.py:710-723)
```python
async def test_async_auth_reads_response_body() -> None:
    """
    Test that we can read the response body in an auth flow if `requires_response_body`
    is set.
    """
    url = "https://example.org/"
    auth = ResponseBodyAuth("xyz")
    app = App()

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response = await client.get(url, auth=auth)

    assert response.status_code == 200
    assert response.json() == {"auth": '{"auth":"xyz"}'}
```
- test_async_auth (tests/client/test_auth.py:743-757)
```python
async def test_async_auth() -> None:
    """
    Test that we can use an auth implementation specific to the async case, to
    support cases that require performing I/O or using concurrency primitives (such
    as checking a disk-based cache or fetching a token from a remote auth server).
    """
    url = "https://example.org/"
    auth = SyncOrAsyncAuth()
    app = App()

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response = await client.get(url, auth=auth)

    assert response.status_code == 200
    assert response.json() == {"auth": "async-auth"}
```
- Client._send_handling_auth (httpx/_client.py:930-962)
```python
    def _send_handling_auth(
        self,
        request: Request,
        auth: Auth,
        follow_redirects: bool,
        history: list[Response],
    ) -> Response:
        auth_flow = auth.sync_auth_flow(request)
        try:
            request = next(auth_flow)

            while True:
                response = self._send_handling_redirects(
                    request,
                    follow_redirects=follow_redirects,
                    history=history,
                )
                try:
                    try:
                        next_request = auth_flow.send(response)
                    except StopIteration:
                        return response

                    response.history = list(history)
                    response.read()
                    request = next_request
                    history.append(response)

                except BaseException as exc:
                    response.close()
                    raise exc
        finally:
            auth_flow.close()
```
- BasicAuth (httpx/_auth.py:126-142)
```python
class BasicAuth(Auth):
    """
    Allows the 'auth' argument to be passed as a (username, password) pair,
    and uses HTTP Basic authentication.
    """

    def __init__(self, username: str | bytes, password: str | bytes) -> None:
        self._auth_header = self._build_auth_header(username, password)

    def auth_flow(self, request: Request) -> typing.Generator[Request, Response, None]:
        request.headers["Authorization"] = self._auth_header
        yield request

    def _build_auth_header(self, username: str | bytes, password: str | bytes) -> str:
        userpass = b":".join((to_bytes(username), to_bytes(password)))
        token = b64encode(userpass).decode()
        return f"Basic {token}"
```
- test_digest_auth_reuses_challenge (tests/client/test_auth.py:540-553)
```python
async def test_digest_auth_reuses_challenge() -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    app = DigestApp()

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response_1 = await client.get(url, auth=auth)
        response_2 = await client.get(url, auth=auth)

        assert response_1.status_code == 200
        assert response_2.status_code == 200

        assert len(response_1.history) == 1
        assert len(response_2.history) == 0
```
- test_sync_auth_history (tests/client/test_auth.py:662-685)
```python
def test_sync_auth_history() -> None:
    """
    Test that intermediate requests sent as part of an authentication flow
    are recorded in the response history.
    """
    url = "https://example.org/"
    auth = RepeatAuth(repeat=2)
    app = App(auth_header="abc")

    with httpx.Client(transport=httpx.MockTransport(app)) as client:
        response = client.get(url, auth=auth)

    assert response.status_code == 200
    assert response.json() == {"auth": "Repeat abc.abc"}

    assert len(response.history) == 2
    resp1, resp2 = response.history
    assert resp1.json() == {"auth": "Repeat 0"}
    assert resp2.json() == {"auth": "Repeat 1"}

    assert len(resp2.history) == 1
    assert resp2.history == [resp1]

    assert len(resp1.history) == 0
```
- App (tests/client/test_auth.py:23-35)
```python
class App:
    """
    A mock app to test auth credentials.
    """

    def __init__(self, auth_header: str = "", status_code: int = 200) -> None:
        self.auth_header = auth_header
        self.status_code = status_code

    def __call__(self, request: httpx.Request) -> httpx.Response:
        headers = {"www-authenticate": self.auth_header} if self.auth_header else {}
        data = {"auth": request.headers.get("Authorization")}
        return httpx.Response(self.status_code, headers=headers, json=data)
```
- NetRCAuth (httpx/_auth.py:145-172)
```python
class NetRCAuth(Auth):
    """
    Use a 'netrc' file to lookup basic auth credentials based on the url host.
    """

    def __init__(self, file: str | None = None) -> None:
        # Lazily import 'netrc'.
        # There's no need for us to load this module unless 'NetRCAuth' is being used.
        import netrc

        self._netrc_info = netrc.netrc(file)

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

    def _build_auth_header(self, username: str | bytes, password: str | bytes) -> str:
        userpass = b":".join((to_bytes(username), to_bytes(password)))
        token = b64encode(userpass).decode()
        return f"Basic {token}"
```
- Proxy (httpx/_config.py:201-243)
```python
class Proxy:
    def __init__(
        self,
        url: URL | str,
        *,
        ssl_context: ssl.SSLContext | None = None,
        auth: tuple[str, str] | None = None,
        headers: HeaderTypes | None = None,
    ) -> None:
        url = URL(url)
        headers = Headers(headers)

        if url.scheme not in ("http", "https", "socks5", "socks5h"):
            raise ValueError(f"Unknown scheme for proxy URL {url!r}")

        if url.username or url.password:
            # Remove any auth credentials from the URL.
            auth = (url.username, url.password)
            url = url.copy_with(username=None, password=None)

        self.url = url
        self.auth = auth
        self.headers = headers
        self.ssl_context = ssl_context

    @property
    def raw_auth(self) -> tuple[bytes, bytes] | None:
        # The proxy authentication as raw bytes.
        return (
            None
            if self.auth is None
            else (self.auth[0].encode("utf-8"), self.auth[1].encode("utf-8"))
        )

    def __repr__(self) -> str:
        # The authentication is represented with the password component masked.
        auth = (self.auth[0], "********") if self.auth else None

        # Build a nice concise representation.
        url_str = f"{str(self.url)!r}"
        auth_str = f", auth={auth!r}" if auth else ""
        headers_str = f", headers={dict(self.headers)!r}" if self.headers else ""
        return f"Proxy({url_str}{auth_str}{headers_str})"
```
- test_async_auth_history (tests/client/test_auth.py:636-659)
```python
async def test_async_auth_history() -> None:
    """
    Test that intermediate requests sent as part of an authentication flow
    are recorded in the response history.
    """
    url = "https://example.org/"
    auth = RepeatAuth(repeat=2)
    app = App(auth_header="abc")

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response = await client.get(url, auth=auth)

    assert response.status_code == 200
    assert response.json() == {"auth": "Repeat abc.abc"}

    assert len(response.history) == 2
    resp1, resp2 = response.history
    assert resp1.json() == {"auth": "Repeat 0"}
    assert resp2.json() == {"auth": "Repeat 1"}

    assert len(resp2.history) == 1
    assert resp2.history == [resp1]

    assert len(resp1.history) == 0
```
- test_auth_property (tests/client/test_auth.py:323-335)
```python
async def test_auth_property() -> None:
    app = App()

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        assert client.auth is None

        client.auth = ("user", "password123")
        assert isinstance(client.auth, httpx.BasicAuth)

        url = "https://example.org/"
        response = await client.get(url)
        assert response.status_code == 200
        assert response.json() == {"auth": "Basic dXNlcjpwYXNzd29yZDEyMw=="}
```
- test_basic_auth (tests/client/test_auth.py:163-172)
```python
async def test_basic_auth() -> None:
    url = "https://example.org/"
    auth = ("user", "password123")
    app = App()

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response = await client.get(url, auth=auth)

    assert response.status_code == 200
    assert response.json() == {"auth": "Basic dXNlcjpwYXNzd29yZDEyMw=="}
```
- test_basic_auth (tests/test_auth.py:14-26)
```python
def test_basic_auth():
    auth = httpx.BasicAuth(username="user", password="pass")
    request = httpx.Request("GET", "https://www.example.com")

    # The initial request should include a basic auth header.
    flow = auth.sync_auth_flow(request)
    request = next(flow)
    assert request.headers["Authorization"].startswith("Basic")

    # No other requests are made.
    response = httpx.Response(content=b"Hello, world!", status_code=200)
    with pytest.raises(StopIteration):
        flow.send(response)
```
- AsyncClient._send_handling_auth (httpx/_client.py:1645-1677)
```python
    async def _send_handling_auth(
        self,
        request: Request,
        auth: Auth,
        follow_redirects: bool,
        history: list[Response],
    ) -> Response:
        auth_flow = auth.async_auth_flow(request)
        try:
            request = await auth_flow.__anext__()

            while True:
                response = await self._send_handling_redirects(
                    request,
                    follow_redirects=follow_redirects,
                    history=history,
                )
                try:
                    try:
                        next_request = await auth_flow.asend(response)
                    except StopAsyncIteration:
                        return response

                    response.history = list(history)
                    await response.aread()
                    request = next_request
                    history.append(response)

                except BaseException as exc:
                    await response.aclose()
                    raise exc
        finally:
            await auth_flow.aclose()
```
- SyncOrAsyncAuth.sync_auth_flow (tests/client/test_auth.py:147-152)
```python
    def sync_auth_flow(
        self, request: httpx.Request
    ) -> typing.Generator[httpx.Request, httpx.Response, None]:
        with self._lock:
            request.headers["Authorization"] = "sync-auth"
        yield request
```
- test_basic_auth_on_session (tests/client/test_auth.py:207-218)
```python
async def test_basic_auth_on_session() -> None:
    url = "https://example.org/"
    auth = ("user", "password123")
    app = App()

    async with httpx.AsyncClient(
        transport=httpx.MockTransport(app), auth=auth
    ) as client:
        response = await client.get(url)

    assert response.status_code == 200
    assert response.json() == {"auth": "Basic dXNlcjpwYXNzd29yZDEyMw=="}
```
- test_custom_auth (tests/client/test_auth.py:222-234)
```python
async def test_custom_auth() -> None:
    url = "https://example.org/"
    app = App()

    def auth(request: httpx.Request) -> httpx.Request:
        request.headers["Authorization"] = "Token 123"
        return request

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response = await client.get(url, auth=auth)

    assert response.status_code == 200
    assert response.json() == {"auth": "Token 123"}
```
- test_netrc_auth_credentials_do_not_exist (tests/client/test_auth.py:256-270)
```python
def test_netrc_auth_credentials_do_not_exist() -> None:
    """
    When netrc auth is being used and a request is made to a host that is
    not in the netrc file, then no credentials should be applied.
    """
    netrc_file = str(FIXTURES_DIR / ".netrc")
    url = "http://example.org"
    app = App()
    auth = httpx.NetRCAuth(netrc_file)

    with httpx.Client(transport=httpx.MockTransport(app), auth=auth) as client:
        response = client.get(url)

    assert response.status_code == 200
    assert response.json() == {"auth": None}
```
- DigestAuth._resolve_qop (httpx/_auth.py:329-340)
```python
    def _resolve_qop(self, qop: bytes | None, request: Request) -> bytes | None:
        if qop is None:
            return None
        qops = re.split(b", ?", qop)
        if b"auth" in qops:
            return b"auth"

        if qops == [b"auth-int"]:
            raise NotImplementedError("Digest auth-int support is not yet implemented")

        message = f'Unexpected qop value "{qop!r}" in digest auth'
        raise ProtocolError(message, request=request)
```
- test_digest_auth_with_401 (tests/test_auth.py:44-66)
```python
def test_digest_auth_with_401():
    auth = httpx.DigestAuth(username="user", password="pass")
    request = httpx.Request("GET", "https://www.example.com")

    # The initial request should not include an auth header.
    flow = auth.sync_auth_flow(request)
    request = next(flow)
    assert "Authorization" not in request.headers

    # If a 401 response is returned, then a digest auth request is made.
    headers = {
        "WWW-Authenticate": 'Digest realm="...", qop="auth", nonce="...", opaque="..."'
    }
    response = httpx.Response(
        content=b"Auth required", status_code=401, headers=headers, request=request
    )
    request = flow.send(response)
    assert request.headers["Authorization"].startswith("Digest")

    # No other requests are made.
    response = httpx.Response(content=b"Hello, world!", status_code=200)
    with pytest.raises(StopIteration):
        flow.send(response)
```
- test_digest_auth_incorrect_credentials (tests/client/test_auth.py:527-536)
```python
async def test_digest_auth_incorrect_credentials() -> None:
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    app = DigestApp(send_response_after_attempt=2)

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response = await client.get(url, auth=auth)

    assert response.status_code == 401
    assert len(response.history) == 1
```
- SyncOrAsyncAuth.async_auth_flow (tests/client/test_auth.py:154-159)
```python
    async def async_auth_flow(
        self, request: httpx.Request
    ) -> typing.AsyncGenerator[httpx.Request, httpx.Response]:
        async with self._async_lock:
            request.headers["Authorization"] = "async-auth"
        yield request
```
- test_auth_hidden_header (tests/client/test_auth.py:311-319)
```python
async def test_auth_hidden_header() -> None:
    url = "https://example.org/"
    auth = ("example-username", "example-password")
    app = App()

    async with httpx.AsyncClient(transport=httpx.MockTransport(app)) as client:
        response = await client.get(url, auth=auth)

    assert "'authorization': '[secure]'" in str(response.request.headers)
```
- test_digest_auth_with_200 (tests/test_auth.py:29-41)
```python
def test_digest_auth_with_200():
    auth = httpx.DigestAuth(username="user", password="pass")
    request = httpx.Request("GET", "https://www.example.com")

    # The initial request should not include an auth header.
    flow = auth.sync_auth_flow(request)
    request = next(flow)
    assert "Authorization" not in request.headers

    # If a 200 response is returned, then no other requests are made.
    response = httpx.Response(content=b"Hello, world!", status_code=200)
    with pytest.raises(StopIteration):
        flow.send(response)
```
- test_netrc_auth_credentials_exist (tests/client/test_auth.py:237-253)
```python
def test_netrc_auth_credentials_exist() -> None:
    """
    When netrc auth is being used and a request is made to a host that is
    in the netrc file, then the relevant credentials should be applied.
    """
    netrc_file = str(FIXTURES_DIR / ".netrc")
    url = "http://netrcexample.org"
    app = App()
    auth = httpx.NetRCAuth(netrc_file)

    with httpx.Client(transport=httpx.MockTransport(app), auth=auth) as client:
        response = client.get(url)

    assert response.status_code == 200
    assert response.json() == {
        "auth": "Basic ZXhhbXBsZS11c2VybmFtZTpleGFtcGxlLXBhc3N3b3Jk"
    }
```
- BaseClient.auth (httpx/_client.py:273-281)
```python
    def auth(self) -> Auth | None:
        """
        Authentication class used when none is passed at the request-level.

        See also [Authentication][0].

        [0]: /quickstart/#authentication
        """
        return self._auth
```
- test_basic_auth_with_stream (tests/client/test_auth.py:176-191)
```python
async def test_basic_auth_with_stream() -> None:
    """
    See: https://github.com/encode/httpx/pull/1312
    """
    url = "https://example.org/"
    auth = ("user", "password123")
    app = App()

    async with httpx.AsyncClient(
        transport=httpx.MockTransport(app), auth=auth
    ) as client:
        async with client.stream("GET", url) as response:
            await response.aread()

    assert response.status_code == 200
    assert response.json() == {"auth": "Basic dXNlcjpwYXNzd29yZDEyMw=="}
```
- test_digest_auth_unavailable_streaming_body (tests/client/test_auth.py:696-706)
```python
async def test_digest_auth_unavailable_streaming_body():
    url = "https://example.org/"
    auth = httpx.DigestAuth(username="user", password="password123")
    app = DigestApp()

    async def streaming_body() -> typing.AsyncIterator[bytes]:
        yield b"Example request body"  # pragma: no cover

    async with httpx.AsyncClient(transport=ConsumeBodyTransport(app)) as client:
        with pytest.raises(httpx.StreamConsumed):
            await client.post(url, content=streaming_body(), auth=auth)
```
- SyncOrAsyncAuth (tests/client/test_auth.py:137-159)
```python
class SyncOrAsyncAuth(httpx.Auth):
    """
    A mock authentication scheme that uses a different implementation for the
    sync and async cases.
    """

    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._async_lock = anyio.Lock()

    def sync_auth_flow(
        self, request: httpx.Request
    ) -> typing.Generator[httpx.Request, httpx.Response, None]:
        with self._lock:
            request.headers["Authorization"] = "sync-auth"
        yield request

    async def async_auth_flow(
        self, request: httpx.Request
    ) -> typing.AsyncGenerator[httpx.Request, httpx.Response]:
        async with self._async_lock:
            request.headers["Authorization"] = "async-auth"
        yield request
```
- ResponseBodyAuth.auth_flow (tests/client/test_auth.py:127-134)
```python
    def auth_flow(
        self, request: httpx.Request
    ) -> typing.Generator[httpx.Request, httpx.Response, None]:
        request.headers["Authorization"] = self.token
        response = yield request
        data = response.text
        request.headers["Authorization"] = data
        yield request
```
- test_digest_auth_with_401_nonce_counting (tests/test_auth.py:69-104)
```python
def test_digest_auth_with_401_nonce_counting():
    auth = httpx.DigestAuth(username="user", password="pass")
    request = httpx.Request("GET", "https://www.example.com")

    # The initial request should not include an auth header.
    flow = auth.sync_auth_flow(request)
    request = next(flow)
    assert "Authorization" not in request.headers

    # If a 401 response is returned, then a digest auth request is made.
    headers = {
        "WWW-Authenticate": 'Digest realm="...", qop="auth", nonce="...", opaque="..."'
    }
    response = httpx.Response(
        content=b"Auth required", status_code=401, headers=headers, request=request
    )
    first_request = flow.send(response)
    assert first_request.headers["Authorization"].startswith("Digest")

    # Each subsequent request contains the digest header by default...
    request = httpx.Request("GET", "https://www.example.com")
    flow = auth.sync_auth_flow(request)
    second_request = next(flow)
    assert second_request.headers["Authorization"].startswith("Digest")

    # ... and the client nonce count (nc) is increased
    first_nc = parse_keqv_list(first_request.headers["Authorization"].split(", "))["nc"]
    second_nc = parse_keqv_list(second_request.headers["Authorization"].split(", "))[
        "nc"
    ]
    assert int(first_nc, 16) + 1 == int(second_nc, 16)

    # No other requests are made.
    response = httpx.Response(content=b"Hello, world!", status_code=200)
    with pytest.raises(StopIteration):
        flow.send(response)
```
- _DigestAuthChallenge (httpx/_auth.py:343-348)
```python
class _DigestAuthChallenge(typing.NamedTuple):
    realm: bytes
    nonce: bytes
    algorithm: str
    opaque: bytes | None
    qop: bytes | None
```
- Auth.sync_auth_flow (httpx/_auth.py:62-85)
```python
    def sync_auth_flow(
        self, request: Request
    ) -> typing.Generator[Request, Response, None]:
        """
        Execute the authentication flow synchronously.

        By default, this defers to `.auth_flow()`. You should override this method
        when the authentication scheme does I/O and/or uses concurrency primitives.
        """
        if self.requires_request_body:
            request.read()

        flow = self.auth_flow(request)
        request = next(flow)

        while True:
            response = yield request
            if self.requires_response_body:
                response.read()

            try:
                request = flow.send(response)
            except StopIteration:
                break
```
- BaseClient (httpx/_client.py:188-591)
```python
class BaseClient:
    def __init__(
        self,
        *,
        auth: AuthTypes | None = None,
        params: QueryParamTypes | None = None,
        headers: HeaderTypes | None = None,
        cookies: CookieTypes | None = None,
        timeout: TimeoutTypes = DEFAULT_TIMEOUT_CONFIG,
        follow_redirects: bool = False,
        max_redirects: int = DEFAULT_MAX_REDIRECTS,
        event_hooks: None | (typing.Mapping[str, list[EventHook]]) = None,
        base_url: URL | str = "",
        trust_env: bool = True,
        default_encoding: str | typing.Callable[[bytes], str] = "utf-8",
    ) -> None:
        event_hooks = {} if event_hooks is None else event_hooks

        self._base_url = self._enforce_trailing_slash(URL(base_url))

        self._auth = self._build_auth(auth)
        self._params = QueryParams(params)
        self.headers = Headers(headers)
        self._cookies = Cookies(cookies)
        self._timeout = Timeout(timeout)
        self.follow_redirects = follow_redirects
