src/click/testing.py:
  L203: def name(self) -> str:
  L382: def get_default_prog_name(self, cli: Command) -> str:
  L32: class EchoingStdin:
  L37: def __init__(self, input: t.BinaryIO, output: t.BinaryIO) -> None:
  L42: def __getattr__(self, x: str) -> t.Any:
  L45: def _echo(self, rv: bytes) -> bytes:
  L51: def read(self, n: int = -1) -> bytes:
  L54: def read1(self, n: int = -1) -> bytes:
  L57: def readline(self, n: int = -1) -> bytes:
  L60: def readlines(self) -> list[bytes]:
  L63: def __iter__(self) -> cabc.Iterator[bytes]:
  L66: def __repr__(self) -> str:
  L71: def _pause_echo(stream: EchoingStdin | None) -> cabc.Generator[None]:
  L80: class _FDCapture:
  L95: def __init__(self, targetfd: int) -> None:
  L100: def start(self) -> None:
  L105: def stop(self) -> bytes:
  L117: class BytesIOCopy(io.BytesIO):
  L125: def __init__(self, copy_to: io.BytesIO) -> None:
  L129: def flush(self) -> None:
  L133: def write(self, b: ReadableBuffer) -> int:
  L138: class StreamMixer:
  L150: def __init__(self) -> None:
  L156: class _NamedTextIOWrapper(io.TextIOWrapper):
  L172: def __init__(
  L184: def close(self) -> None:
  L192: def fileno(self) -> int:
  L207: def mode(self) -> str:
  L211: def make_input_stream(
  L231: class Result:
  L262: def __init__(
  L283: def output(self) -> str:
  L295: def stdout(self) -> str:
  L302: def stderr(self) -> str:
  L312: def __repr__(self) -> str:
  L317: class CliRunner:
  L360: def __init__(
  L389: def make_env(
  L399: def isolation(
  L596: def invoke(
  L742: def isolated_filesystem(
src/click/core.py:
  L2520: def human_readable_name(self) -> str:
  L3838: def human_readable_name(self) -> str:
  L2482: def _check_name_is_usable(self, name: str, decls: cabc.Sequence[str]) -> None:
  L61: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
  L195: class ParameterSource(enum.IntEnum):
  L957: def set_parameter_source(self, name: str, source: ParameterSource) -> None:
  L966: def get_parameter_source(self, name: str) -> ParameterSource | None:
  L2237: class Parameter(ABC):
  L2356: def __init__(
  L2441: def _hide_unset(value: t.Any) -> t.Any:
  L2448: def to_info_dict(self) -> dict[str, t.Any]:
  L2474: def __repr__(self) -> str:
  L2478: def _parse_decls(
  L2526: def make_metavar(self, ctx: Context) -> str:
  L2541: def get_default(
  L2546: def get_default(
  L2550: def get_default(
  L2584: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None: ...
  L2586: def consume_value(
  L2634: def type_cast_value(self, ctx: Context, value: t.Any) -> t.Any:
  L2690: def value_is_missing(self, value: t.Any) -> bool:
  L2708: def process_value(self, ctx: Context, value: t.Any) -> t.Any:
  L2774: def resolve_envvar_value(self, ctx: Context) -> str | None:
  L2819: def value_from_envvar(self, ctx: Context) -> str | cabc.Sequence[str] | None:
  L2835: def handle_parse_result(
  L2923: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L2926: def get_usage_pieces(self, ctx: Context) -> list[str]:
  L2929: def get_error_hint(self, ctx: Context | None) -> str:
  L2939: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L3335: def _check_name_is_normalized(self, name: str, decls: cabc.Sequence[str]) -> None:
  L3464: def get_help_spec(self, ctx: Context) -> str:
  L64: def _complete_visible_commands(
  L83: def _check_nested_chain(
  L103: def _echo_aborted() -> None:
  L108: def _outside_click_stacklevel() -> int:
  L128: def _format_deprecated_label(deprecated: bool | str) -> str:
  L136: def _format_deprecated_suffix(deprecated: bool | str) -> str:
  L145: def batch(iterable: cabc.Iterable[V], batch_size: int) -> list[tuple[V, ...]]:
  L150: def augment_usage_errors(
  L168: def iter_params_for_processing(
  L234: class Context:
  L366: def __init__(
  L543: def protected_args(self) -> list[str]:
  L554: def to_info_dict(self) -> dict[str, t.Any]:
  L575: def __enter__(self) -> Self:
  L580: def __exit__(
  L595: def scope(self, cleanup: bool = True) -> cabc.Generator[Context]:
  L633: def meta(self) -> dict[str, t.Any]:
  L660: def make_formatter(self) -> HelpFormatter:
  L674: def with_resource(self, context_manager: AbstractContextManager[V]) -> V:
  L703: def call_on_close(self, f: t.Callable[..., t.Any]) -> t.Callable[..., t.Any]:
  L715: def close(self) -> None:
  L722: def _close_with_exception_info(
  L741: def command_path(self) -> str:
  L759: def find_root(self) -> Context:
  L766: def find_object(self, object_type: type[V]) -> V | None:
  L778: def ensure_object(self, object_type: type[V]) -> V:
  L787: def _default_map_has(self, name: str | None) -> bool:
  L802: def lookup_default(
  L807: def lookup_default(
  L811: def lookup_default(self, name: str, call: bool = True) -> t.Any | None:
  L833: def fail(self, message: str) -> t.NoReturn:
  L841: def abort(self) -> t.NoReturn:
  L845: def exit(self, code: int = 0) -> t.NoReturn:
  L855: def get_usage(self) -> str:
  L861: def get_help(self) -> str:
  L867: def _make_sub_context(self, command: Command) -> Context:
  L876: def invoke(
  L881: def invoke(self, callback: Command, /, *args: t.Any, **kwargs: t.Any) -> t.Any: ...
  L883: def invoke(
  L938: def forward(self, cmd: Command, /, *args: t.Any, **kwargs: t.Any) -> t.Any:
  L985: class Command:
  L1061: def __init__(
  L1105: def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
  L1116: def __repr__(self) -> str:
  L1119: def get_usage(self, ctx: Context) -> str:
  L1128: def get_params(self, ctx: Context) -> list[Parameter]:
  L1184: def format_usage(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1192: def collect_usage_pieces(self, ctx: Context) -> list[str]:
  L1203: def get_help_option_names(self, ctx: Context) -> list[str]:
  L1218: def get_help_option(self, ctx: Context) -> Option | None:
  L1252: def make_parser(self, ctx: Context) -> _OptionParser:
  L1259: def get_help(self, ctx: Context) -> str:
  L1268: def get_short_help_str(self, limit: int = 45) -> str:
  L1284: def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1303: def format_help_text(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1321: def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1333: def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1346: def format_epilog(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1355: def make_context(
  L1392: def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
  L1428: def invoke(self, ctx: Context) -> t.Any:
  L1444: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L1492: def main(
  L1502: def main(
  L1511: def main(
  L1647: def _main_shell_completion(
  L1679: def __call__(self, *args: t.Any, **kwargs: t.Any) -> t.Any:
  L1684: class _FakeSubclassCheck(type):
  L1685: def __subclasscheck__(cls, subclass: type) -> bool:
  L1688: def __instancecheck__(cls, instance: t.Any) -> bool:
  L1692: class _BaseCommand(Command, metaclass=_FakeSubclassCheck):
  L1699: class Group(Command):
  L1758: def __init__(
  L1813: def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
  L1831: def add_command(self, cmd: Command, name: str | None = None) -> None:
  L1842: def command(self, __func: t.Callable[..., t.Any]) -> Command: ...
  L1845: def command(
  L1849: def command(
  L1891: def group(self, __func: t.Callable[..., t.Any]) -> Group: ...
  L1894: def group(
  L1898: def group(
  L1942: def result_callback(self, replace: bool = False) -> t.Callable[[F], F]:
  L1987: def get_command(self, ctx: Context, cmd_name: str) -> Command | None:
  L1993: def list_commands(self, ctx: Context) -> list[str]:
  L1997: def collect_usage_pieces(self, ctx: Context) -> list[str]:
  L2002: def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
  L2006: def format_commands(self, ctx: Context, formatter: HelpFormatter) -> None:
  L2034: def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
  L2048: def invoke(self, ctx: Context) -> t.Any:
  L2116: def resolve_command(
  L2142: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L2162: class _MultiCommand(Group, metaclass=_FakeSubclassCheck):
  L2169: class CommandCollection(Group):
  L2187: def __init__(
  L2197: def add_source(self, group: Group) -> None:
  L2201: def get_command(self, ctx: Context, cmd_name: str) -> Command | None:
  L2218: def list_commands(self, ctx: Context) -> list[str]:
  L2227: def _check_iter(value: cabc.Iterable[V]) -> cabc.Iterator[V]:
  L2963: class Option(Parameter):
  L3055: def __init__(
  L3134: def is_bool_flag(self) -> bool:
  L3143: def flag_activation_value(self) -> t.Any:
  L3155: def _infer_flag_kind(
  L3188: def _pick_type(
  L3227: def _resolve_lazy_default(self, value: t.Any) -> t.Any:
  L3251: def _validate(self, prompt: bool | str, deprecated: bool | str) -> None:
  L3273: def to_info_dict(self) -> dict[str, t.Any]:
  L3295: def get_default(
  L3329: def get_error_hint(self, ctx: Context | None) -> str:
  L3355: def _parse_decls(
  L3422: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
  L3495: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L3520: def get_help_extra(self, ctx: Context) -> types.OptionHelpExtra:
  L3604: def prompt_for_value(self, ctx: Context) -> t.Any:
  L3654: def resolve_envvar_value(self, ctx: Context) -> str | None:
  L3678: def value_from_envvar(self, ctx: Context) -> t.Any:
  L3726: def consume_value(
  L3782: def process_value(self, ctx: Context, value: t.Any) -> t.Any:
  L3800: class Argument(Parameter):
  L3815: def __init__(
  L3843: def make_metavar(self, ctx: Context) -> str:
  L3862: def _parse_decls(
  L3883: def get_usage_pieces(self, ctx: Context) -> list[str]:
  L3886: def get_help_record(self, ctx: Context) -> tuple[str, str]:
  L3899: def get_error_hint(self, ctx: Context | None) -> str:
  L3904: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
  L3908: def __getattr__(name: str) -> object:
src/click/types.py:
  L888: class UUIDParameterType(ParamType[uuid.UUID]):
  L891: def convert(
  L906: def __repr__(self) -> str:
  L1282: def name(self) -> str:  # type: ignore[override]
  L60: class ParamTypeInfoDict(t.TypedDict):
  L65: class ParamType(t.Generic[_ValueT_co, _InputT_contra], abc.ABC):
  L122: def to_info_dict(self) -> ParamTypeInfoDict:
  L144: def __call__(
  L152: def __call__(
  L159: def __call__(
  L169: def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
  L172: def get_missing_message(self, param: Parameter, ctx: Context | None) -> str | None:
  L179: def convert(
  L205: def split_envvar_value(self, rv: str) -> cabc.Sequence[str]:
  L215: def fail(
  L224: def shell_complete(
  L242: class CompositeParamType(ParamType[_ValueT_co]):
  L247: def arity(self) -> int: ...  # type: ignore[override]
  L264: class FuncParamType(ParamType[_ValueT_co], t.Generic[_ValueT_contra, _ValueT_co]):
  L268: def __init__(self, func: t.Callable[[_ValueT_contra], _ValueT_co]) -> None:
  L272: def to_info_dict(self) -> FuncParamTypeInfoDict[_ValueT_contra, _ValueT_co]:
  L275: def convert(
  L292: class UnprocessedParamType(ParamType[t.Any]):
  L295: def convert(
  L300: def __repr__(self) -> str:
  L304: class StringParamType(ParamType[str]):
  L307: def convert(
  L325: def __repr__(self) -> str:
  L342: class Choice(ParamType[_ValueT_co], t.Generic[_ValueT_co]):
  L375: def __init__(
  L381: def to_info_dict(self) -> ChoiceInfoDict[_ValueT_co]:
  L388: def _normalized_mapping(
  L406: def normalize_choice(self, choice: object, ctx: Context | None) -> str:
  L426: def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
  L444: def get_missing_message(self, param: Parameter, ctx: Context | None) -> str:
  L454: def convert(
  L478: def get_invalid_choice_message(self, value: t.Any, ctx: Context | None) -> str:
  L492: def __repr__(self) -> str:
  L495: def shell_complete(
  L518: class DateTimeInfoDict(ParamTypeInfoDict):
  L522: class DateTime(ParamType[datetime]):
  L547: def __init__(self, formats: cabc.Sequence[str] | None = None):
  L554: def to_info_dict(self) -> DateTimeInfoDict:
  L557: def get_metavar(self, param: Parameter, ctx: Context) -> str:
  L560: def _try_to_convert_date(self, value: t.Any, format: str) -> datetime | None:
  L566: def convert(
  L589: def __repr__(self) -> str:
  L593: class _NumberParamTypeBase(
  L598: def convert(
  L632: class _NumberRangeBase(
  L642: def __init__(
  L656: def to_info_dict(self) -> NumberRangeInfoDict[_FloatValueT_co]:
  L666: def convert(
  L700: def _clamp(
  L717: def _describe_range(self) -> str:
  L731: def __repr__(self) -> str:
  L736: class IntParamType(_NumberParamTypeBase[t.SupportsInt | t.SupportsIndex, int]):
  L740: def __repr__(self) -> str:
  L744: class IntRange(_NumberRangeBase[int, int], IntParamType):
  L761: def _clamp(self, bound: int, dir: t.Literal[1, -1], open: bool) -> int:
  L768: class FloatParamType(_NumberParamTypeBase[t.SupportsFloat | t.SupportsIndex, float]):
  L772: def __repr__(self) -> str:
  L776: class FloatRange(_NumberRangeBase[float, float], FloatParamType):
  L794: def __init__(
  L809: def _clamp(self, bound: float, dir: t.Literal[1, -1], open: bool) -> float:
  L819: class BoolParamType(ParamType[bool]):
  L856: def str_to_bool(value: str | bool) -> bool | None:
  L870: def convert(
  L884: def __repr__(self) -> str:
  L910: class FileInfoDict(ParamTypeInfoDict):
  L915: class File(ParamType[t.IO[t.Any]]):
  L954: def __init__(
  L968: def to_info_dict(self) -> FileInfoDict:
  L975: def resolve_lazy_flag(self, value: str | os.PathLike[str]) -> bool:
  L984: def convert(
  L1029: def shell_complete(
  L1046: def _is_file_like(value: t.Any) -> te.TypeIs[t.IO[t.Any]]:
  L1050: class PathInfoDict(ParamTypeInfoDict):
  L1059: class Path(ParamType[_PathT_co, str | os.PathLike[str]], t.Generic[_PathT_co]):
  L1107: def __init__(
  L1136: def to_info_dict(self) -> PathInfoDict:
  L1147: def coerce_path_result(self, value: str | os.PathLike[str]) -> _PathT_co:
  L1161: def convert(
  L1235: def shell_complete(
  L1254: class TupleInfoDict(ParamTypeInfoDict):
  L1258: class Tuple(CompositeParamType[tuple[t.Any, ...]]):
  L1272: def __init__(self, types: cabc.Sequence[type[t.Any] | ParamType[t.Any]]) -> None:
  L1275: def to_info_dict(self) -> TupleInfoDict:
  L1286: def arity(self) -> int:  # type: ignore[override]
  L1289: def convert(
  L1311: def _guess_type(
  L1347: def convert_type(ty: None, default: None = None) -> StringParamType: ...
  L1349: def convert_type(
  L1353: def convert_type(
  L1356: def convert_type(
  L1411: UNPROCESSED: t.Final[UnprocessedParamType] = UnprocessedParamType()
  L1415: STRING: t.Final[StringParamType] = StringParamType()
  L1419: INT: t.Final[IntParamType] = IntParamType()
  L1423: FLOAT: t.Final[FloatParamType] = FloatParamType()
  L1427: BOOL: t.Final[BoolParamType] = BoolParamType()
  L1430: UUID: t.Final[UUIDParameterType] = UUIDParameterType()
  L1433: class OptionHelpExtra(t.TypedDict, total=False):
src/click/_compat.py:
  L116: def readable(self) -> bool:
  L463: def name(self) -> str:
  L13: CYGWIN = sys.platform.startswith("cygwin")
  L14: WIN = sys.platform.startswith("win")
  L22: def _make_text_stream(
  L43: def is_ascii_encoding(encoding: str) -> bool:
  L51: def get_best_encoding(stream: t.IO[t.Any]) -> str:
  L59: class _NonClosingTextIOWrapper(io.TextIOWrapper):
  L60: def __init__(
  L74: def __del__(self) -> None:
  L80: def isatty(self) -> bool:
  L85: class _FixupStream:
  L95: def __init__(
  L105: def __getattr__(self, name: str) -> t.Any:
  L108: def read1(self, size: int) -> bytes:
  L128: def writable(self) -> bool:
  L143: def seekable(self) -> bool:
  L154: def _is_binary_reader(stream: t.IO[t.Any], default: bool = False) -> bool:
  L163: def _is_binary_writer(stream: t.IO[t.Any], default: bool = False) -> bool:
  L176: def _find_binary_reader(stream: t.IO[t.Any]) -> t.BinaryIO | None:
  L194: def _find_binary_writer(stream: t.IO[t.Any]) -> t.BinaryIO | None:
  L212: def _stream_is_misconfigured(stream: t.TextIO) -> bool:
  L221: def _is_compat_stream_attr(stream: t.TextIO, attr: str, value: str | None) -> bool:
  L230: def _is_compatible_text_stream(
  L241: def _force_correct_text_stream(
  L287: def _force_correct_text_reader(
  L303: def _force_correct_text_writer(
  L319: def get_binary_stdin() -> t.BinaryIO:
  L326: def get_binary_stdout() -> t.BinaryIO:
  L333: def get_binary_stderr() -> t.BinaryIO:
  L340: def get_text_stdin(encoding: str | None = None, errors: str | None = None) -> t.TextIO:
  L347: def get_text_stdout(encoding: str | None = None, errors: str | None = None) -> t.TextIO:
  L354: def get_text_stderr(encoding: str | None = None, errors: str | None = None) -> t.TextIO:
  L361: def _wrap_io_open(
  L374: def open_stream(
  L455: class _AtomicFile:
  L456: def __init__(self, f: t.IO[t.Any], tmp_filename: str, real_filename: str) -> None:
  L466: def close(self, delete: bool = False) -> None:
  L473: def __getattr__(self, name: str) -> t.Any:
  L476: def __enter__(self) -> _AtomicFile:
  L479: def __exit__(
  L487: def __repr__(self) -> str:
  L491: def strip_ansi(value: str) -> str:
  L495: def _is_jupyter_kernel_output(stream: t.IO[t.Any]) -> bool:
  L502: def should_strip_ansi(
  L536: def term_len(x: str) -> int:
  L540: def isatty(stream: t.IO[t.Any]) -> bool:
  L547: def _make_cached_stream_func(
src/click/_winconsole.py:
  L124: def readable(self) -> t.Literal[True]:
  L196: def name(self) -> str:
  L53: STDIN_HANDLE = GetStdHandle(-10)
  L54: STDOUT_HANDLE = GetStdHandle(-11)
  L55: STDERR_HANDLE = GetStdHandle(-12)
  L60: ERROR_SUCCESS = 0
  L61: ERROR_NOT_ENOUGH_MEMORY = 8
  L62: ERROR_OPERATION_ABORTED = 995
  L64: EOF = b"\x1a"
  L65: MAX_BYTES_WRITTEN = 32767
  L114: class _WindowsConsoleRawIOBase(io.RawIOBase):
  L115: def __init__(self, handle: int | None) -> None:
  L118: def isatty(self) -> t.Literal[True]:
  L123: class _WindowsConsoleReader(_WindowsConsoleRawIOBase):
  L127: def readinto(self, b: Buffer) -> int:
  L158: class _WindowsConsoleWriter(_WindowsConsoleRawIOBase):
  L159: def writable(self) -> t.Literal[True]:
  L163: def _get_error_message(errno: int) -> str:
  L170: def write(self, b: Buffer) -> int:
  L190: class ConsoleStream:
  L191: def __init__(self, text_stream: t.TextIO, byte_stream: t.BinaryIO) -> None:
  L199: def write(self, x: t.AnyStr) -> int:
  L208: def writelines(self, lines: cabc.Iterable[t.AnyStr]) -> None:
  L212: def __getattr__(self, name: str) -> t.Any:
  L215: def isatty(self) -> bool:
  L218: def __repr__(self) -> str:
  L222: def _get_text_stdin(buffer_stream: t.BinaryIO) -> t.TextIO:
  L232: def _get_text_stdout(buffer_stream: t.BinaryIO) -> t.TextIO:
  L242: def _get_text_stderr(buffer_stream: t.BinaryIO) -> t.TextIO:
  L259: def _is_console(f: t.TextIO) -> bool:
  L272: def _get_windows_console_stream(
src/click/utils.py:
  L550: def _detect_program_name(
  L32: def _posixify(name: str) -> str:
  L36: def _safecall(func: t.Callable[P, R]) -> t.Callable[P, R | None]:
  L52: def make_str(value: t.Any) -> str:
  L62: def _make_default_short_help(help: str, max_length: int = 45) -> str:
  L103: class _LazyFile:
  L120: def __init__(
  L145: def __getattr__(self, name: str) -> t.Any:
  L148: def __repr__(self) -> str:
  L153: def open(self) -> t.IO[t.Any]:
  L171: def close(self) -> None:
  L176: def close_intelligently(self) -> None:
  L183: def __enter__(self) -> _LazyFile:
  L186: def __exit__(
  L194: def __iter__(self) -> cabc.Iterator[t.AnyStr]:
  L199: class _KeepOpenFile:
  L216: def __init__(self, file: t.IO[t.Any]) -> None:
  L219: def __getattr__(self, name: str) -> t.Any:
  L222: def __enter__(self) -> _KeepOpenFile:
  L225: def __exit__(
  L233: def __repr__(self) -> str:
  L236: def __iter__(self) -> cabc.Iterator[t.AnyStr]:
  L240: def echo(
  L337: def _get_binary_stream(name: t.Literal["stdin", "stdout", "stderr"]) -> t.BinaryIO:
  L354: def _get_text_stream(
  L381: def open_file(
  L430: def format_filename(
  L472: def get_app_dir(app_name: str, roaming: bool = True, force_posix: bool = False) -> str:
  L521: class _PacifyFlushWrapper:
  L534: def __init__(self, wrapped: t.IO[t.Any]) -> None:
  L537: def flush(self) -> None:
  L546: def __getattr__(self, attr: str) -> t.Any:
  L605: def _expand_args(
  L657: def __getattr__(name: str) -> object:
src/click/exceptions.py:
  L114: class BadParameter(UsageError):
  L135: def __init__(
  L146: def format_message(self) -> str:
  L159: class MissingParameter(BadParameter):
  L173: def __init__(
  L184: def format_message(self) -> str:
  L224: def __str__(self) -> str:
  L19: def _join_param_hints(param_hint: cabc.Sequence[str] | str | None) -> str | None:
  L26: def _format_possibilities(possibilities: list[str]) -> str:
  L35: class ClickException(Exception):
  L44: def __init__(self, message: str) -> None:
  L51: def format_message(self) -> str:
  L54: def __str__(self) -> str:
  L57: def show(self, file: t.IO[t.Any] | None = None) -> None:
  L68: class UsageError(ClickException):
  L82: def __init__(self, message: str, ctx: Context | None = None) -> None:
  L87: def show(self, file: t.IO[t.Any] | None = None) -> None:
  L232: class NoSuchOption(UsageError):
  L241: def __init__(
  L262: def format_message(self) -> str:
  L268: class NoSuchCommand(UsageError):
  L277: def __init__(
  L298: def format_message(self) -> str:
  L304: class BadOptionUsage(UsageError):
  L316: def __init__(
  L323: class BadArgumentUsage(UsageError):
  L332: class NoArgsIsHelpError(UsageError):
  L335: def __init__(self, ctx: Context) -> None:
  L338: def show(self, file: t.IO[t.Any] | None = None) -> None:
  L342: class FileError(ClickException):
  L348: def __init__(self, filename: str, hint: str | None = None) -> None:
  L356: def format_message(self) -> str:
  L362: class Abort(RuntimeError):
  L366: class Exit(RuntimeError):
  L377: def __init__(self, code: int = 0) -> None:
tests/test_arguments.py:
  L166: def test_parameter_name_is_an_identifier_or_deprecated(count, expose_value):
  L237: def test_parameter_name_transform(cls, form, decl, expect):
  L278: def test_parameter_name_not_an_identifier_is_deprecated(cls, form, decl):
  L304: def test_parameter_name_identifier_check_follows_the_unicode_table(cls, form, char):
  L330: def test_parameter_name_is_not_nfkc_normalized(runner, decorator, decl, argv):
  L134: NAME_TRANSFORM_DECLS = UNICODE_CASE_DECLS + [
  L145: NAME_SWEEP_DECLS = [
  L201: def test_argument_name_check_applies_when_not_exposed():
  L245: def test_option_explicit_name_keeps_its_case(decl, expect):
  L353: def test_argument_name_keeps_its_normalization_form(runner):
  L375: def test_argument_name_case_transform_can_collide(runner):
  L389: def test_argument_name_can_collide_with_an_option(runner):
  L14: def test_nargs_star(runner):
  L27: def test_nargs_tup(runner):
  L50: def test_nargs_tup_composite(runner, opts):
  L62: def test_nargs_mismatch_with_tuple_type():
  L71: def test_nargs_err(runner):
  L98: def test_argument_names(runner, decl, expect):
  L111: def test_argument_normalizes_an_identifier_decl():
  L119: PARAM_KINDS = [
  L125: UNICODE_CASE_DECLS = [
  L192: def test_argument_requires_its_one_declaration():
  L207: def test_argument_metavar_renders_what_a_declaration_may_not(runner):
  L407: def test_argument_has_no_auto_envvar(runner):
  L430: def test_argument_explicit_envvar_case_sensitivity(runner, env, expect):
  L448: def test_bytes_args(runner, monkeypatch):
  L471: def test_file_args(runner, tmp_path):
  L493: def test_path_allow_dash(runner):
  L504: def test_file_atomics(runner, tmp_path):
  L522: def test_stdout_default(runner):
  L547: def test_nargs_envvar(runner, nargs, value, expect):
  L567: def test_nargs_envvar_only_if_values_empty(runner):
  L580: def test_empty_nargs(runner):
  L600: def test_missing_arg(runner):
  L635: def test_required_argument(value, expect_missing, processed_value):
  L650: def test_implicit_non_required(runner):
  L661: def test_argument_help(runner):
  L680: def test_argument_help_options_only_no_arguments_section(runner):
  L693: def test_argument_help_lists_undocumented_arguments(runner):
  L715: def test_argument_help_undocumented_arguments_only_no_section(runner):
  L729: def test_argument_help_deprecated_without_help_lists_all(runner):
  L749: def test_argument_help_empty_string_lists_all(runner):
  L779: def test_argument_get_help_record_never_none(help_text, expected):
  L786: def test_argument_help_optional_metavar(runner):
  L798: def test_deprecated_usage(runner):
  L822: def test_argument_metavar_marks_optional(runner, kwargs, expected):
  L839: def test_deprecated_usage_help_record(runner, deprecated, expected_label):
  L852: def test_deprecated_usage_help_record_without_help(runner):
  L870: def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
  L880: def test_deprecated_warning(runner, deprecated):
  L901: def test_deprecated_required(runner):
  L906: def test_eat_options(runner):
  L922: def test_nargs_star_ordering(runner):
  L935: def test_nargs_specified_plus_star_ordering(runner):
  L971: def test_good_defaults_for_nargs(runner, argument_params, args, expected):
  L1058: def test_bad_defaults_for_nargs(runner, default, message):
  L1070: def test_multiple_param_decls_not_allowed(runner):
  L1079: def test_multiple_not_allowed():
  L1084: def test_subcommand_help(runner):
  L1103: def test_nested_subcommand_help(runner):
  L1125: def test_when_argument_decorator_is_used_multiple_times_cls_is_preserved():
  L1154: def test_duplicate_names_warning(runner, args_one, args_two):
