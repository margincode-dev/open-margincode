httpx/_models.py:
  L751: def is_client_error(self) -> bool:
  L758: def is_server_error(self) -> bool:
  L765: def is_error(self) -> bool:
  L53: SENSITIVE_HEADERS = {"authorization", "proxy-authorization"}
  L56: def _is_known_encoding(encoding: str) -> bool:
  L67: def _normalize_header_key(key: str | bytes, encoding: str | None = None) -> bytes:
  L74: def _normalize_header_value(value: str | bytes, encoding: str | None = None) -> bytes:
  L85: def _parse_content_type_charset(content_type: str) -> str | None:
  L93: def _parse_header_links(value: str) -> list[dict[str, str]]:
  L130: def _obfuscate_sensitive_headers(
  L139: class Headers(typing.MutableMapping[str, str]):
  L144: def __init__(
  L167: def encoding(self) -> str:
  L192: def encoding(self, value: str) -> None:
  L196: def raw(self) -> list[tuple[bytes, bytes]]:
  L202: def keys(self) -> typing.KeysView[str]:
  L205: def values(self) -> typing.ValuesView[str]:
  L216: def items(self) -> typing.ItemsView[str, str]:
  L231: def multi_items(self) -> list[tuple[str, str]]:
  L242: def get(self, key: str, default: typing.Any = None) -> typing.Any:
  L252: def get_list(self, key: str, split_commas: bool = False) -> list[str]:
  L274: def update(self, headers: HeaderTypes | None = None) -> None:  # type: ignore
  L281: def copy(self) -> Headers:
  L284: def __getitem__(self, key: str) -> str:
  L304: def __setitem__(self, key: str, value: str) -> None:
  L328: def __delitem__(self, key: str) -> None:
  L346: def __contains__(self, key: typing.Any) -> bool:
  L350: def __iter__(self) -> typing.Iterator[typing.Any]:
  L353: def __len__(self) -> int:
  L356: def __eq__(self, other: typing.Any) -> bool:
  L366: def __repr__(self) -> str:
  L382: class Request:
  L383: def __init__(
  L441: def _prepare(self, default_headers: dict[str, str]) -> None:
  L463: def content(self) -> bytes:
  L468: def read(self) -> bytes:
  L482: async def aread(self) -> bytes:
  L496: def __repr__(self) -> str:
  L501: def __getstate__(self) -> dict[str, typing.Any]:
  L508: def __setstate__(self, state: dict[str, typing.Any]) -> None:
  L515: class Response:
  L516: def __init__(
  L571: def _prepare(self, default_headers: dict[str, str]) -> None:
  L579: def elapsed(self) -> datetime.timedelta:
  L592: def elapsed(self, elapsed: datetime.timedelta) -> None:
  L596: def request(self) -> Request:
  L607: def request(self, value: Request) -> None:
  L611: def http_version(self) -> str:
  L620: def reason_phrase(self) -> str:
  L629: def url(self) -> URL:
  L636: def content(self) -> bytes:
  L642: def text(self) -> str:
  L653: def encoding(self) -> str | None:
  L675: def encoding(self, value: str) -> None:
  L689: def charset_encoding(self) -> str | None:
  L699: def _get_content_decoder(self) -> ContentDecoder:
  L725: def is_informational(self) -> bool:
  L732: def is_success(self) -> bool:
  L739: def is_redirect(self) -> bool:
  L772: def has_redirect_location(self) -> bool:
  L794: def raise_for_status(self) -> Response:
  L831: def json(self, **kwargs: typing.Any) -> typing.Any:
  L835: def cookies(self) -> Cookies:
  L842: def links(self) -> dict[str | None, dict[str, str]]:
  L856: def num_bytes_downloaded(self) -> int:
  L859: def __repr__(self) -> str:
  L862: def __getstate__(self) -> dict[str, typing.Any]:
  L869: def __setstate__(self, state: dict[str, typing.Any]) -> None:
  L876: def read(self) -> bytes:
  L884: def iter_bytes(self, chunk_size: int | None = None) -> typing.Iterator[bytes]:
  L907: def iter_text(self, chunk_size: int | None = None) -> typing.Iterator[str]:
  L926: def iter_lines(self) -> typing.Iterator[str]:
  L935: def iter_raw(self, chunk_size: int | None = None) -> typing.Iterator[bytes]:
  L961: def close(self) -> None:
  L974: async def aread(self) -> bytes:
  L982: async def aiter_bytes(
  L1007: async def aiter_text(
  L1028: async def aiter_lines(self) -> typing.AsyncIterator[str]:
  L1037: async def aiter_raw(
  L1065: async def aclose(self) -> None:
  L1079: class Cookies(typing.MutableMapping[str, str]):
  L1084: def __init__(self, cookies: CookieTypes | None = None) -> None:
  L1101: def extract_cookies(self, response: Response) -> None:
  L1110: def set_cookie_header(self, request: Request) -> None:
  L1117: def set(self, name: str, value: str, domain: str = "", path: str = "/") -> None:
  L1143: def get(  # type: ignore
  L1168: def delete(
  L1192: def clear(self, domain: str | None = None, path: str | None = None) -> None:
  L1205: def update(self, cookies: CookieTypes | None = None) -> None:  # type: ignore
  L1210: def __setitem__(self, name: str, value: str) -> None:
  L1213: def __getitem__(self, name: str) -> str:
  L1219: def __delitem__(self, name: str) -> None:
  L1222: def __len__(self) -> int:
  L1225: def __iter__(self) -> typing.Iterator[str]:
  L1228: def __bool__(self) -> bool:
  L1233: def __repr__(self) -> str:
  L1243: class _CookieCompatRequest(urllib.request.Request):
  L1249: def __init__(self, request: Request) -> None:
  L1257: def add_unredirected_header(self, key: str, value: str) -> None:
  L1261: class _CookieCompatResponse:
  L1267: def __init__(self, response: Response) -> None:
  L1270: def info(self) -> email.message.Message:
httpx/_exceptions.py:
  L202: class ProxyError(TransportError):
  L216: class ProtocolError(TransportError):
  L222: class LocalProtocolError(ProtocolError):
  L232: class RemoteProtocolError(ProtocolError):
  L74: class HTTPError(Exception):
  L92: def __init__(self, message: str) -> None:
  L97: def request(self) -> Request:
  L103: def request(self, request: Request) -> None:
  L107: class RequestError(HTTPError):
  L112: def __init__(self, message: str, *, request: Request | None = None) -> None:
  L123: class TransportError(RequestError):
  L167: class NetworkError(TransportError):
  L175: class ReadError(NetworkError):
  L181: class WriteError(NetworkError):
  L187: class ConnectError(NetworkError):
  L193: class CloseError(NetworkError):
  L208: class UnsupportedProtocol(TransportError):
  L243: class DecodingError(RequestError):
  L258: class HTTPStatusError(HTTPError):
  L265: def __init__(self, message: str, *, request: Request, response: Response) -> None:
  L297: class StreamError(RuntimeError):
  L305: def __init__(self, message: str) -> None:
  L132: class TimeoutException(TransportError):
  L140: class ConnectTimeout(TimeoutException):
  L146: class ReadTimeout(TimeoutException):
  L152: class WriteTimeout(TimeoutException):
  L158: class PoolTimeout(TimeoutException):
  L249: class TooManyRedirects(RequestError):
  L271: class InvalidURL(Exception):
  L276: def __init__(self, message: str) -> None:
  L280: class CookieConflict(Exception):
  L287: def __init__(self, message: str) -> None:
  L309: class StreamConsumed(StreamError):
  L315: def __init__(self) -> None:
  L327: class StreamClosed(StreamError):
  L333: def __init__(self) -> None:
  L340: class ResponseNotRead(StreamError):
  L345: def __init__(self) -> None:
  L353: class RequestNotRead(StreamError):
  L358: def __init__(self) -> None:
  L367: def request_context(
httpx/_urls.py:
  L127: def scheme(self) -> str:
  L135: def raw_scheme(self) -> bytes:
  L15: class URL:
  L77: def __init__(self, url: URL | str = "", **kwargs: typing.Any) -> None:
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
  L404: def raw(self) -> tuple[bytes, bytes, int, bytes]:  # pragma: nocover
  L420: class QueryParams(typing.Mapping[str, str]):
  L425: def __init__(self, *args: QueryParamTypes | None, **kwargs: typing.Any) -> None:
  L463: def keys(self) -> typing.KeysView[str]:
  L474: def values(self) -> typing.ValuesView[str]:
  L486: def items(self) -> typing.ItemsView[str, str]:
  L498: def multi_items(self) -> list[tuple[str, str]]:
  L512: def get(self, key: typing.Any, default: typing.Any = None) -> typing.Any:
  L526: def get_list(self, key: str) -> list[str]:
  L537: def set(self, key: str, value: typing.Any = None) -> QueryParams:
  L552: def add(self, key: str, value: typing.Any = None) -> QueryParams:
  L567: def remove(self, key: str) -> QueryParams:
  L582: def merge(self, params: QueryParamTypes | None = None) -> QueryParams:
  L600: def __getitem__(self, key: typing.Any) -> str:
  L603: def __contains__(self, key: typing.Any) -> bool:
  L606: def __iter__(self) -> typing.Iterator[typing.Any]:
  L609: def __len__(self) -> int:
  L612: def __bool__(self) -> bool:
  L615: def __hash__(self) -> int:
  L618: def __eq__(self, other: typing.Any) -> bool:
  L623: def __str__(self) -> str:
  L626: def __repr__(self) -> str:
  L631: def update(self, params: QueryParamTypes | None = None) -> None:
  L637: def __setitem__(self, key: str, value: str) -> None:
httpx/_client.py:
  L239: def _get_proxy_map(
  L740: def _init_proxy_transport(
  L1454: def _init_proxy_transport(
  L62: def _is_https_redirect(url: URL, location: URL) -> bool:
  L77: def _port_or_default(url: URL) -> int | None:
  L83: def _same_origin(url: URL, other: URL) -> bool:
  L94: class UseClientDefault:
  L114: USE_CLIENT_DEFAULT = UseClientDefault()
  L119: USER_AGENT = f"python-httpx/{__version__}"
  L120: ACCEPT_ENCODING = ", ".join(
  L125: class ClientState(enum.Enum):
  L139: class BoundSyncStream(SyncByteStream):
  L145: def __init__(
  L152: def __iter__(self) -> typing.Iterator[bytes]:
  L156: def close(self) -> None:
  L162: class BoundAsyncStream(AsyncByteStream):
  L168: def __init__(
  L175: async def __aiter__(self) -> typing.AsyncIterator[bytes]:
  L179: async def aclose(self) -> None:
  L188: class BaseClient:
  L189: def __init__(
  L224: def is_closed(self) -> bool:
  L231: def trust_env(self) -> bool:
  L234: def _enforce_trailing_slash(self, url: URL) -> URL:
  L254: def timeout(self) -> Timeout:
  L258: def timeout(self, timeout: TimeoutTypes) -> None:
  L262: def event_hooks(self) -> dict[str, list[EventHook]]:
  L266: def event_hooks(self, event_hooks: dict[str, list[EventHook]]) -> None:
  L273: def auth(self) -> Auth | None:
  L284: def auth(self, auth: AuthTypes) -> None:
  L288: def base_url(self) -> URL:
  L295: def base_url(self, url: URL | str) -> None:
  L299: def headers(self) -> Headers:
  L306: def headers(self, headers: HeaderTypes) -> None:
  L319: def cookies(self) -> Cookies:
  L326: def cookies(self, cookies: CookieTypes) -> None:
  L330: def params(self) -> QueryParams:
  L337: def params(self, params: QueryParamTypes) -> None:
  L340: def build_request(
  L391: def _merge_url(self, url: URL | str) -> URL:
  L413: def _merge_cookies(self, cookies: CookieTypes | None = None) -> CookieTypes | None:
  L424: def _merge_headers(self, headers: HeaderTypes | None = None) -> HeaderTypes | None:
  L433: def _merge_queryparams(
  L445: def _build_auth(self, auth: AuthTypes | None) -> Auth | None:
  L457: def _build_request_auth(
  L475: def _build_redirect_request(self, request: Request, response: Response) -> Request:
  L494: def _redirect_method(self, request: Request, response: Response) -> str:
  L517: def _redirect_url(self, request: Request, response: Response) -> URL:
  L546: def _redirect_headers(self, request: Request, url: URL, method: str) -> Headers:
  L573: def _redirect_stream(
  L584: def _set_timeout(self, request: Request) -> None:
  L594: class Client(BaseClient):
  L639: def __init__(
  L718: def _init_transport(
  L760: def _transport_for_url(self, url: URL) -> BaseTransport:
  L771: def request(
  L828: def stream(
  L879: def send(
  L930: def _send_handling_auth(
  L964: def _send_handling_redirects(
  L1001: def _send_single_request(self, request: Request) -> Response:
  L1036: def get(
  L1065: def options(
  L1094: def head(
  L1123: def post(
  L1160: def put(
  L1197: def patch(
  L1234: def delete(
  L1263: def close(self) -> None:
  L1275: def __enter__(self: T) -> T:
  L1293: def __exit__(
  L1307: class AsyncClient(BaseClient):
  L1353: def __init__(
  L1432: def _init_transport(
  L1474: def _transport_for_url(self, url: URL) -> AsyncBaseTransport:
  L1485: async def request(
  L1543: async def stream(
  L1594: async def send(
  L1645: async def _send_handling_auth(
  L1679: async def _send_handling_redirects(
  L1717: async def _send_single_request(self, request: Request) -> Response:
  L1751: async def get(
  L1780: async def options(
  L1809: async def head(
  L1838: async def post(
  L1875: async def put(
  L1912: async def patch(
  L1949: async def delete(
  L1978: async def aclose(self) -> None:
  L1990: async def __aenter__(self: U) -> U:
  L2008: async def __aexit__(
httpx/_decoders.py:
  L36: class ContentDecoder:
  L37: def decode(self, data: bytes) -> bytes:
  L40: def flush(self) -> bytes:
  L44: class IdentityDecoder(ContentDecoder):
  L49: def decode(self, data: bytes) -> bytes:
  L52: def flush(self) -> bytes:
  L56: class DeflateDecoder(ContentDecoder):
  L63: def __init__(self) -> None:
  L67: def decode(self, data: bytes) -> bytes:
  L78: def flush(self) -> bytes:
  L85: class GZipDecoder(ContentDecoder):
  L92: def __init__(self) -> None:
  L95: def decode(self, data: bytes) -> bytes:
  L101: def flush(self) -> bytes:
  L108: class BrotliDecoder(ContentDecoder):
  L118: def __init__(self) -> None:
  L136: def decode(self, data: bytes) -> bytes:
  L145: def flush(self) -> bytes:
  L161: class ZStandardDecoder(ContentDecoder):
  L170: def __init__(self) -> None:
  L180: def decode(self, data: bytes) -> bytes:
  L194: def flush(self) -> bytes:
  L203: class MultiDecoder(ContentDecoder):
  L208: def __init__(self, children: typing.Sequence[ContentDecoder]) -> None:
  L216: def decode(self, data: bytes) -> bytes:
  L221: def flush(self) -> bytes:
  L228: class ByteChunker:
  L233: def __init__(self, chunk_size: int | None = None) -> None:
  L237: def decode(self, content: bytes) -> list[bytes]:
  L260: def flush(self) -> list[bytes]:
  L267: class TextChunker:
  L272: def __init__(self, chunk_size: int | None = None) -> None:
  L276: def decode(self, content: str) -> list[str]:
  L299: def flush(self) -> list[str]:
  L306: class TextDecoder:
  L311: def __init__(self, encoding: str = "utf-8") -> None:
  L314: def decode(self, data: bytes) -> str:
  L317: def flush(self) -> str:
  L321: class LineDecoder:
  L329: def __init__(self) -> None:
  L333: def decode(self, text: str) -> list[str]:
  L371: def flush(self) -> list[str]:
  L381: SUPPORTED_DECODERS = {
httpx/_api.py:
  L39: def request(
  L124: def stream(
  L174: def get(
  L210: def options(
  L246: def head(
  L282: def post(
  L323: def put(
  L364: def patch(
  L405: def delete(
tests/conftest.py:
  L23: ENVIRONMENT_VARIABLES = {
  L35: def clean_environ():
  L59: async def app(scope: Scope, receive: Receive, send: Send) -> None:
  L79: async def hello_world(scope: Scope, receive: Receive, send: Send) -> None:
  L90: async def hello_world_json(scope: Scope, receive: Receive, send: Send) -> None:
  L101: async def slow_response(scope: Scope, receive: Receive, send: Send) -> None:
  L113: async def status_code(scope: Scope, receive: Receive, send: Send) -> None:
  L125: async def echo_body(scope: Scope, receive: Receive, send: Send) -> None:
  L144: async def echo_binary(scope: Scope, receive: Receive, send: Send) -> None:
  L163: async def echo_headers(scope: Scope, receive: Receive, send: Send) -> None:
  L178: async def redirect_301(scope: Scope, receive: Receive, send: Send) -> None:
  L186: def cert_authority():
  L191: def localhost_cert(cert_authority):
  L196: def cert_pem_file(localhost_cert):
  L202: def cert_private_key_file(localhost_cert):
  L208: def cert_encrypted_private_key_file(localhost_cert):
  L224: class TestServer(Server):
  L226: def url(self) -> httpx.URL:
  L230: def install_signal_handlers(self) -> None:
  L235: async def serve(self, sockets=None):
  L245: async def restart(self) -> None:  # pragma: no cover
  L256: async def watch_restarts(self) -> None:  # pragma: no cover
  L271: def serve_in_thread(server: TestServer) -> typing.Iterator[TestServer]:
  L284: def server() -> typing.Iterator[TestServer]:
httpx/_config.py:
  L201: class Proxy:
  L202: def __init__(
  L227: def raw_auth(self) -> tuple[bytes, bytes] | None:
  L235: def __repr__(self) -> str:
  L16: class UnsetType:
  L20: UNSET = UnsetType()
  L23: def create_ssl_context(
  L72: class Timeout:
  L86: def __init__(
  L132: def as_dict(self) -> dict[str, float | None]:
  L140: def __eq__(self, other: typing.Any) -> bool:
  L149: def __repr__(self) -> str:
  L159: class Limits:
  L173: def __init__(
  L184: def __eq__(self, other: typing.Any) -> bool:
  L192: def __repr__(self) -> str:
  L246: DEFAULT_TIMEOUT_CONFIG = Timeout(timeout=5.0)
  L247: DEFAULT_LIMITS = Limits(max_connections=100, max_keepalive_connections=20)
  L248: DEFAULT_MAX_REDIRECTS = 20
httpx/_utils.py:
  L15: def primitive_value_to_str(value: PrimitiveData) -> str:
  L30: def get_environment_proxies() -> dict[str, str | None]:
  L79: def to_bytes(value: str | bytes, encoding: str = "utf-8") -> bytes:
  L83: def to_str(value: str | bytes, encoding: str = "utf-8") -> str:
  L87: def to_bytes_or_str(value: str, match_type_of: typing.AnyStr) -> typing.AnyStr:
  L91: def unquote(value: str) -> str:
  L95: def peek_filelike_length(stream: typing.Any) -> int | None:
  L120: class URLPattern:
  L162: def __init__(self, pattern: str) -> None:
  L192: def matches(self, other: URL) -> bool:
  L206: def priority(self) -> tuple[int, int, int]:
  L219: def __hash__(self) -> int:
  L222: def __lt__(self, other: URLPattern) -> bool:
  L225: def __eq__(self, other: typing.Any) -> bool:
  L229: def is_ipv4_hostname(hostname: str) -> bool:
  L237: def is_ipv6_hostname(hostname: str) -> bool:
httpx/_urlparse.py:
  L29: MAX_URL_LENGTH = 65536
  L32: UNRESERVED_CHARACTERS = (
  L35: SUB_DELIMS = "!$&'()*+,;="
  L37: PERCENT_ENCODED_REGEX = re.compile("%[A-Fa-f0-9]{2}")
  L43: FRAG_SAFE = "".join(
  L49: QUERY_SAFE = "".join(
  L55: PATH_SAFE = "".join(
  L66: USERNAME_SAFE = "".join(
  L76: PASSWORD_SAFE = "".join(
  L89: USERINFO_SAFE = "".join(
  L106: URL_REGEX = re.compile(
  L125: AUTHORITY_REGEX = re.compile(
  L140: COMPONENT_REGEX = {
  L158: class ParseResult(typing.NamedTuple):
  L168: def authority(self) -> str:
  L178: def netloc(self) -> str:
  L186: def copy_with(self, **kwargs: str | None) -> ParseResult:
  L200: def __str__(self) -> str:
  L213: def urlparse(url: str = "", **kwargs: str | None) -> ParseResult:
  L348: def encode_host(host: str) -> str:
  L395: def normalize_port(port: str | int | None, scheme: str) -> int | None:
  L422: def validate_path(path: str, has_scheme: bool, has_authority: bool) -> None:
  L447: def normalize_path(path: str) -> str:
  L478: def PERCENT(string: str) -> str:
  L482: def percent_encoded(string: str, safe: str) -> str:
  L497: def quote(string: str, safe: str) -> str:
tests/client/test_proxies.py:
  L123: def test_unsupported_proxy_scheme():
  L16: def test_socks_proxy():
  L31: PROXY_URL = "http://[::1]"
  L104: async def test_async_proxy_close():
  L114: def test_sync_proxy_close():
  L250: def test_for_deprecated_proxy_params(proxies, is_valid):
  L260: def test_proxy_with_mounts():
  L7: def url_to_origin(url: str) -> httpcore.URL:
  L88: def test_transport_for_request(url, proxies, expected):
  L226: def test_proxies_environ(monkeypatch, client_class, url, env, expected):
httpx/_status_codes.py:
  L67: def is_client_error(cls, value: int) -> bool:
  L74: def is_server_error(cls, value: int) -> bool:
  L81: def is_error(cls, value: int) -> bool:
  L8: class codes(IntEnum):
  L28: def __new__(cls, value: int, phrase: str = "") -> codes:
  L35: def __str__(self) -> str:
  L39: def get_reason_phrase(cls, value: int) -> str:
  L46: def is_informational(cls, value: int) -> bool:
  L53: def is_success(cls, value: int) -> bool:
  L60: def is_redirect(cls, value: int) -> bool:
tests/client/test_auth.py:
  L604: async def test_async_digest_auth_raises_protocol_error_on_malformed_header(
  L623: def test_sync_digest_auth_raises_protocol_error_on_malformed_header(
  L277: def test_netrc_auth_nopassword_parse_error() -> None:  # pragma: no cover
  L370: def test_digest_auth_returns_no_auth_if_alternate_auth_scheme() -> None:
  L23: class App:
  L28: def __init__(self, auth_header: str = "", status_code: int = 200) -> None:
  L32: def __call__(self, request: httpx.Request) -> httpx.Response:
  L38: class DigestApp:
  L39: def __init__(
  L52: def __call__(self, request: httpx.Request) -> httpx.Response:
  L59: def challenge_send(self, request: httpx.Request) -> httpx.Response:
  L87: class RepeatAuth(httpx.Auth):
  L97: def __init__(self, repeat: int) -> None:
  L100: def auth_flow(
  L115: class ResponseBodyAuth(httpx.Auth):
  L124: def __init__(self, token: str) -> None:
  L127: def auth_flow(
  L137: class SyncOrAsyncAuth(httpx.Auth):
  L143: def __init__(self) -> None:
  L147: def sync_auth_flow(
  L154: async def async_auth_flow(
  L163: async def test_basic_auth() -> None:
  L176: async def test_basic_auth_with_stream() -> None:
  L195: async def test_basic_auth_in_url() -> None:
  L207: async def test_basic_auth_on_session() -> None:
  L222: async def test_custom_auth() -> None:
  L237: def test_netrc_auth_credentials_exist() -> None:
  L256: def test_netrc_auth_credentials_do_not_exist() -> None:
  L289: async def test_auth_disable_per_request() -> None:
  L303: def test_auth_hidden_url() -> None:
  L311: async def test_auth_hidden_header() -> None:
  L323: async def test_auth_property() -> None:
  L339: async def test_auth_invalid_type() -> None:
  L357: async def test_digest_auth_returns_no_auth_if_no_digest_header_in_response() -> None:
  L385: async def test_digest_auth_200_response_including_digest_auth_header() -> None:
  L400: async def test_digest_auth_401_response_without_digest_auth_header() -> None:
  L427: async def test_digest_auth(
  L460: async def test_digest_auth_no_specified_qop() -> None:
  L492: async def test_digest_auth_qop_including_spaces_and_auth_returns_auth(qop: str) -> None:
  L505: async def test_digest_auth_qop_auth_int_not_implemented() -> None:
  L516: async def test_digest_auth_qop_must_be_auth_or_auth_int() -> None:
  L527: async def test_digest_auth_incorrect_credentials() -> None:
  L540: async def test_digest_auth_reuses_challenge() -> None:
  L557: async def test_digest_auth_resets_nonce_count_after_401() -> None:
  L636: async def test_async_auth_history() -> None:
  L662: def test_sync_auth_history() -> None:
  L688: class ConsumeBodyTransport(httpx.MockTransport):
  L689: async def handle_async_request(self, request: httpx.Request) -> httpx.Response:
  L696: async def test_digest_auth_unavailable_streaming_body():
  L710: async def test_async_auth_reads_response_body() -> None:
  L726: def test_sync_auth_reads_response_body() -> None:
  L743: async def test_async_auth() -> None:
  L760: def test_sync_auth() -> None:
httpx/_types.py:
  L92: class SyncByteStream:
  L93: def __iter__(self) -> Iterator[bytes]:
  L99: def close(self) -> None:
  L106: class AsyncByteStream:
  L107: async def __aiter__(self) -> AsyncIterator[bytes]:
  L113: async def aclose(self) -> None:
tests/test_config.py:
  L182: def test_invalid_proxy_scheme():
  L164: def test_proxy_from_url():
  L173: def test_proxy_with_auth_from_url():
  L11: def test_load_ssl_config():
  L17: def test_load_ssl_config_verify_non_existing_file():
  L23: def test_load_ssl_with_keylog(monkeypatch: typing.Any) -> None:
  L29: def test_load_ssl_config_verify_existing_file():
  L36: def test_load_ssl_config_verify_directory():
  L43: def test_load_ssl_config_cert_and_key(cert_pem_file, cert_private_key_file):
  L51: def test_load_ssl_config_cert_and_encrypted_key(
  L60: def test_load_ssl_config_cert_and_key_invalid_password(
  L70: def test_load_ssl_config_cert_without_key_raises(cert_pem_file):
  L76: def test_load_ssl_config_no_verify():
  L82: def test_SSLContext_with_get_request(server, cert_pem_file):
  L89: def test_limits_repr():
  L98: def test_limits_eq():
  L103: def test_timeout_eq():
  L108: def test_timeout_all_parameters_set():
  L113: def test_timeout_from_nothing():
  L121: def test_timeout_from_none():
  L126: def test_timeout_from_one_none_value():
  L131: def test_timeout_from_one_value():
  L136: def test_timeout_from_one_value_and_default():
  L141: def test_timeout_missing_default():
  L146: def test_timeout_from_tuple():
  L151: def test_timeout_from_config_instance():
  L156: def test_timeout_repr():
tests/models/test_url.py:
  L56: def test_url_no_scheme():
  L403: def test_urlparse_with_invalid_scheme():
  L557: def test_resolution_error_1833():
  L8: def test_basic_url():
  L24: def test_complete_url():
  L40: def test_url_with_empty_query():
  L63: def test_url_no_authority():
  L135: def test_path_query_fragment(url, raw_path, path, query, fragment):
  L143: def test_url_query_encoding():
  L154: def test_url_params():
  L207: def test_url_username_and_password(url, userinfo, username, password):
  L217: def test_url_valid_host():
  L222: def test_url_normalized_host():
  L227: def test_url_percent_escape_host():
  L232: def test_url_ipv4_like_host():
  L241: def test_url_valid_port():
  L246: def test_url_normalized_port():
  L252: def test_url_invalid_port():
  L261: def test_url_normalized_path():
  L266: def test_url_escaped_path():
  L271: def test_url_leading_dot_prefix_on_absolute_url():
  L276: def test_url_leading_dot_prefix_on_relative_url():
  L286: def test_param_with_space():
  L293: def test_param_requires_encoding():
  L299: def test_param_with_percent_encoded():
  L307: def test_param_with_existing_escape_requires_encoding():
  L320: def test_query_with_existing_percent_encoding():
