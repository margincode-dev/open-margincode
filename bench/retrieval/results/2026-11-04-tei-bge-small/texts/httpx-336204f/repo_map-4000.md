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
