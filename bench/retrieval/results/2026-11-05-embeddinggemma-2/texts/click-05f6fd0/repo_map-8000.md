src/click/core.py:
  L3806: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L2850: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L3401: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L60: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
  L840: def get_help(self) -> str:
  L1182: def get_help_option_names(self, ctx: Context) -> list[str]:
  L1197: def get_help_option(self, ctx: Context) -> Option | None:
  L1238: def get_help(self, ctx: Context) -> str:
  L1247: def get_short_help_str(self, limit: int = 45) -> str:
  L1263: def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1282: def format_help_text(self, ctx: Context, formatter: HelpFormatter) -> None:
  L3370: def get_help_spec(self, ctx: Context) -> str:
  L3426: def get_help_extra(self, ctx: Context) -> types.OptionHelpExtra:
  L3706: class Argument(Parameter):
  L3721: def __init__(
  L3754: def to_info_dict(self) -> dict[str, t.Any]:
  L3760: def human_readable_name(self) -> str:
  L3765: def make_metavar(self, ctx: Context) -> str:
  L3784: def _parse_decls(
  L3803: def get_usage_pieces(self, ctx: Context) -> list[str]:
  L3812: def get_error_hint(self, ctx: Context | None) -> str:
  L3817: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
  L63: def _complete_visible_commands(
  L82: def _check_nested_chain(
  L102: def _echo_aborted() -> None:
  L107: def _format_deprecated_label(deprecated: bool | str) -> str:
  L115: def _format_deprecated_suffix(deprecated: bool | str) -> str:
  L124: def batch(iterable: cabc.Iterable[V], batch_size: int) -> list[tuple[V, ...]]:
  L129: def augment_usage_errors(
  L147: def iter_params_for_processing(
  L174: class ParameterSource(enum.IntEnum):
  L213: class Context:
  L345: def __init__(
  L522: def protected_args(self) -> list[str]:
  L533: def to_info_dict(self) -> dict[str, t.Any]:
  L554: def __enter__(self) -> Self:
  L559: def __exit__(
  L574: def scope(self, cleanup: bool = True) -> cabc.Generator[Context]:
  L612: def meta(self) -> dict[str, t.Any]:
  L639: def make_formatter(self) -> HelpFormatter:
  L653: def with_resource(self, context_manager: AbstractContextManager[V]) -> V:
  L682: def call_on_close(self, f: t.Callable[..., t.Any]) -> t.Callable[..., t.Any]:
  L694: def close(self) -> None:
  L701: def _close_with_exception_info(
  L720: def command_path(self) -> str:
  L738: def find_root(self) -> Context:
  L745: def find_object(self, object_type: type[V]) -> V | None:
  L757: def ensure_object(self, object_type: type[V]) -> V:
  L766: def _default_map_has(self, name: str | None) -> bool:
  L781: def lookup_default(
  L786: def lookup_default(
  L790: def lookup_default(self, name: str, call: bool = True) -> t.Any | None:
  L812: def fail(self, message: str) -> t.NoReturn:
  L820: def abort(self) -> t.NoReturn:
  L824: def exit(self, code: int = 0) -> t.NoReturn:
  L834: def get_usage(self) -> str:
  L846: def _make_sub_context(self, command: Command) -> Context:
  L855: def invoke(
  L860: def invoke(self, callback: Command, /, *args: t.Any, **kwargs: t.Any) -> t.Any: ...
  L862: def invoke(
  L917: def forward(self, cmd: Command, /, *args: t.Any, **kwargs: t.Any) -> t.Any:
  L936: def set_parameter_source(self, name: str, source: ParameterSource) -> None:
  L945: def get_parameter_source(self, name: str) -> ParameterSource | None:
  L964: class Command:
  L1040: def __init__(
  L1084: def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
  L1095: def __repr__(self) -> str:
  L1098: def get_usage(self, ctx: Context) -> str:
  L1107: def get_params(self, ctx: Context) -> list[Parameter]:
  L1163: def format_usage(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1171: def collect_usage_pieces(self, ctx: Context) -> list[str]:
  L1231: def make_parser(self, ctx: Context) -> _OptionParser:
  L1300: def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1312: def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1328: def format_epilog(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1337: def make_context(
  L1374: def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
  L1410: def invoke(self, ctx: Context) -> t.Any:
  L1426: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L1474: def main(
  L1484: def main(
  L1493: def main(
  L1629: def _main_shell_completion(
  L1661: def __call__(self, *args: t.Any, **kwargs: t.Any) -> t.Any:
  L1666: class _FakeSubclassCheck(type):
  L1667: def __subclasscheck__(cls, subclass: type) -> bool:
  L1670: def __instancecheck__(cls, instance: t.Any) -> bool:
  L1674: class _BaseCommand(Command, metaclass=_FakeSubclassCheck):
  L1681: class Group(Command):
  L1740: def __init__(
  L1795: def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
  L1813: def add_command(self, cmd: Command, name: str | None = None) -> None:
  L1824: def command(self, __func: t.Callable[..., t.Any]) -> Command: ...
  L1827: def command(
  L1831: def command(
  L1873: def group(self, __func: t.Callable[..., t.Any]) -> Group: ...
  L1876: def group(
  L1880: def group(
  L1924: def result_callback(self, replace: bool = False) -> t.Callable[[F], F]:
  L1969: def get_command(self, ctx: Context, cmd_name: str) -> Command | None:
  L1975: def list_commands(self, ctx: Context) -> list[str]:
  L1979: def collect_usage_pieces(self, ctx: Context) -> list[str]:
  L1984: def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1988: def format_commands(self, ctx: Context, formatter: HelpFormatter) -> None:
  L2016: def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
  L2030: def invoke(self, ctx: Context) -> t.Any:
  L2098: def resolve_command(
  L2124: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L2144: class _MultiCommand(Group, metaclass=_FakeSubclassCheck):
  L2151: class CommandCollection(Group):
  L2169: def __init__(
  L2179: def add_source(self, group: Group) -> None:
  L2183: def get_command(self, ctx: Context, cmd_name: str) -> Command | None:
  L2200: def list_commands(self, ctx: Context) -> list[str]:
  L2209: def _check_iter(value: cabc.Iterable[V]) -> cabc.Iterator[V]:
  L2219: class Parameter(ABC):
  L2331: def __init__(
  L2406: def _hide_unset(value: t.Any) -> t.Any:
  L2413: def to_info_dict(self) -> dict[str, t.Any]:
  L2438: def __repr__(self) -> str:
  L2442: def _parse_decls(
  L2447: def human_readable_name(self) -> str:
  L2453: def make_metavar(self, ctx: Context) -> str:
  L2468: def get_default(
  L2473: def get_default(
  L2477: def get_default(
  L2511: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None: ...
  L2513: def consume_value(
  L2561: def type_cast_value(self, ctx: Context, value: t.Any) -> t.Any:
  L2617: def value_is_missing(self, value: t.Any) -> bool:
  L2635: def process_value(self, ctx: Context, value: t.Any) -> t.Any:
  L2701: def resolve_envvar_value(self, ctx: Context) -> str | None:
  L2746: def value_from_envvar(self, ctx: Context) -> str | cabc.Sequence[str] | None:
  L2762: def handle_parse_result(
  L2853: def get_usage_pieces(self, ctx: Context) -> list[str]:
  L2856: def get_error_hint(self, ctx: Context | None) -> str:
  L2866: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L2890: class Option(Parameter):
  L2983: def __init__(
  L3065: def is_bool_flag(self) -> bool:
  L3074: def flag_activation_value(self) -> t.Any:
  L3086: def _infer_flag_kind(
  L3119: def _pick_type(
  L3158: def _resolve_lazy_default(self, value: t.Any) -> t.Any:
  L3182: def _validate(self, prompt: bool | str, deprecated: bool | str) -> None:
  L3204: def to_info_dict(self) -> dict[str, t.Any]:
  L3227: def get_default(
  L3261: def get_error_hint(self, ctx: Context | None) -> str:
  L3267: def _parse_decls(
  L3328: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
  L3510: def prompt_for_value(self, ctx: Context) -> t.Any:
  L3560: def resolve_envvar_value(self, ctx: Context) -> str | None:
  L3584: def value_from_envvar(self, ctx: Context) -> t.Any:
  L3632: def consume_value(
  L3688: def process_value(self, ctx: Context, value: t.Any) -> t.Any:
  L3821: def __getattr__(name: str) -> object:
src/click/testing.py:
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
  L203: def name(self) -> str:
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
  L382: def get_default_prog_name(self, cli: Command) -> str:
  L389: def make_env(
  L399: def isolation(
  L596: def invoke(
  L742: def isolated_filesystem(
src/click/_compat.py:
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
  L116: def readable(self) -> bool:
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
  L463: def name(self) -> str:
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
src/click/formatting.py:
  L110: class HelpFormatter:
  L127: def __init__(
  L146: def write(self, string: str) -> None:
  L150: def indent(self) -> None:
  L154: def dedent(self) -> None:
  L158: def write_usage(self, prog: str, args: str = "", prefix: str | None = None) -> None:
  L204: def write_heading(self, heading: str) -> None:
  L208: def write_paragraph(self) -> None:
  L213: def write_text(self, text: str) -> None:
  L229: def write_dl(
  L274: def section(self, name: str) -> cabc.Generator[None]:
  L289: def indentation(self) -> cabc.Generator[None]:
  L297: def getvalue(self) -> str:
  L11: FORCED_WIDTH: int | None = None
  L14: def measure_table(rows: cabc.Iterable[tuple[str, str]]) -> tuple[int, ...]:
  L24: def iter_rows(
  L31: def wrap_text(
  L302: def join_options(options: cabc.Iterable[str]) -> tuple[str, bool]:
src/click/utils.py:
  L62: def _make_default_short_help(help: str, max_length: int = 45) -> str:
  L32: def _posixify(name: str) -> str:
  L36: def _safecall(func: t.Callable[P, R]) -> t.Callable[P, R | None]:
  L52: def make_str(value: t.Any) -> str:
  L115: class _LazyFile:
  L132: def __init__(
  L157: def __getattr__(self, name: str) -> t.Any:
  L160: def __repr__(self) -> str:
  L165: def open(self) -> t.IO[t.Any]:
  L183: def close(self) -> None:
  L188: def close_intelligently(self) -> None:
  L195: def __enter__(self) -> _LazyFile:
  L198: def __exit__(
  L206: def __iter__(self) -> cabc.Iterator[t.AnyStr]:
  L211: class _KeepOpenFile:
  L228: def __init__(self, file: t.IO[t.Any]) -> None:
  L231: def __getattr__(self, name: str) -> t.Any:
  L234: def __enter__(self) -> _KeepOpenFile:
  L237: def __exit__(
  L245: def __repr__(self) -> str:
  L248: def __iter__(self) -> cabc.Iterator[t.AnyStr]:
  L252: def echo(
  L349: def _get_binary_stream(name: t.Literal["stdin", "stdout", "stderr"]) -> t.BinaryIO:
  L366: def _get_text_stream(
  L393: def open_file(
  L442: def format_filename(
  L484: def get_app_dir(app_name: str, roaming: bool = True, force_posix: bool = False) -> str:
  L533: class _PacifyFlushWrapper:
  L546: def __init__(self, wrapped: t.IO[t.Any]) -> None:
  L549: def flush(self) -> None:
  L558: def __getattr__(self, attr: str) -> t.Any:
  L562: def _detect_program_name(
  L617: def _expand_args(
  L669: def __getattr__(name: str) -> object:
src/click/types.py:
  L1433: class OptionHelpExtra(t.TypedDict, total=False):
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
  L888: class UUIDParameterType(ParamType[uuid.UUID]):
  L891: def convert(
  L906: def __repr__(self) -> str:
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
  L1282: def name(self) -> str:  # type: ignore[override]
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
src/click/_winconsole.py:
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
  L124: def readable(self) -> t.Literal[True]:
  L127: def readinto(self, b: Buffer) -> int:
  L158: class _WindowsConsoleWriter(_WindowsConsoleRawIOBase):
  L159: def writable(self) -> t.Literal[True]:
  L163: def _get_error_message(errno: int) -> str:
  L170: def write(self, b: Buffer) -> int:
  L190: class ConsoleStream:
  L191: def __init__(self, text_stream: t.TextIO, byte_stream: t.BinaryIO) -> None:
  L196: def name(self) -> str:
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
src/click/decorators.py:
  L324: def argument(
  L603: def help_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
  L25: FC = t.TypeVar("FC", bound="_AnyCallable | Command")
  L28: def pass_context(f: t.Callable[te.Concatenate[Context, P], R]) -> t.Callable[P, R]:
  L39: def pass_obj(f: t.Callable[te.Concatenate[T, P], R]) -> t.Callable[P, R]:
  L51: def make_pass_decorator(
  L100: def pass_meta_key(
  L138: def command(name: _AnyCallable) -> Command: ...
  L144: def command(
  L153: def command(
  L163: def command(
  L168: def command(
  L263: def group(name: _AnyCallable) -> Group: ...
  L269: def group(
  L278: def group(
  L288: def group(
  L293: def group(
  L314: def _param_memo(f: t.Callable[..., t.Any], param: Parameter) -> None:
  L352: def option(
  L380: def confirmation_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
  L404: def password_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
  L421: def version_option(
  L561: def custom_version_option(
src/click/_termui_impl.py:
  L43: class ProgressBar(t.Generic[V]):
  L44: def __init__(
  L115: def __enter__(self) -> ProgressBar[V]:
  L120: def __exit__(
  L128: def __iter__(self) -> cabc.Iterator[V]:
  L134: def __next__(self) -> V:
  L142: def render_finish(self) -> None:
  L157: def pct(self) -> float:
  L163: def time_per_iteration(self) -> float:
  L169: def eta(self) -> float:
  L174: def format_eta(self) -> str:
  L195: def format_pos(self) -> str:
  L201: def format_pct(self) -> str:
  L204: def format_bar(self) -> str:
  L223: def format_progress_line(self) -> str:
  L250: def render_progress(self) -> None:
  L296: def make_step(self, n_steps: int) -> None:
  L318: def update(self, n_steps: int, current_item: V | None = None) -> None:
  L344: def finish(self) -> None:
  L349: def generator(self) -> cabc.Iterator[V]:
  L383: class _PagerWriter:
  L397: def __init__(self, stream: t.TextIO, color: bool) -> None:
  L401: def write(self, text: str) -> int:
  L407: def writelines(self, lines: cabc.Iterable[str]) -> None:
  L411: def flush(self) -> None:
  L414: def close(self) -> None:
  L419: def __getattr__(self, name: str) -> t.Any:
  L423: def _resolve_pager_command(cmd_parts: list[str]) -> tuple[Path, list[str]] | None:
  L451: def _pager_contextmanager(
  L497: def get_pager_file(color: bool | None = None) -> t.Generator[t.TextIO, None, None]:
  L520: def _less_uses_raw_mode(less_env: str, cmd_params: list[str]) -> bool:
  L549: def _pipepager(
  L636: def _tempfilepager(
  L671: def _nullpager(
  L683: class Editor:
  L684: def __init__(
  L696: def get_editor(self) -> str:
  L713: def edit_files(self, filenames: cabc.Iterable[str | os.PathLike[str]]) -> None:
  L744: def edit(self, text: bytes | bytearray) -> bytes | None: ...
  L749: def edit(self, text: str | None) -> str | None: ...
  L751: def edit(self, text: str | bytes | bytearray | None) -> str | bytes | None:
  L799: def open_url(url: str, wait: bool = False, locate: bool = False) -> int:
  L869: def _translate_ch_to_exc(ch: str) -> None:
tests/test_arguments.py:
  L363: def test_argument_help_deprecated_without_help_lists_all(runner):
  L471: def test_deprecated_usage_help_record_without_help(runner):
  L295: def test_argument_help(runner):
  L314: def test_argument_help_options_only_no_arguments_section(runner):
  L327: def test_argument_help_lists_undocumented_arguments(runner):
  L349: def test_argument_help_undocumented_arguments_only_no_section(runner):
  L383: def test_argument_help_empty_string_lists_all(runner):
  L405: def test_argument_help_optional_metavar(runner):
  L458: def test_deprecated_usage_help_record(runner, deprecated, expected_label):
  L269: def test_required_argument(value, expect_missing, processed_value):
  L441: def test_argument_metavar_marks_optional(runner, kwargs, expected):
  L489: def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
  L703: def test_subcommand_help(runner):
  L722: def test_nested_subcommand_help(runner):
  L744: def test_when_argument_decorator_is_used_multiple_times_cls_is_preserved():
