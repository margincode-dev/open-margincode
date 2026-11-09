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
