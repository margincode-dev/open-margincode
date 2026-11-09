src/click/types.py:
  L168: def convert(
  L194: def split_envvar_value(self, rv: str) -> cabc.Sequence[str]:
  L264: def convert(
  L284: def convert(
  L296: def convert(
  L587: def convert(
  L859: def convert(
  L880: def convert(
  L1133: def coerce_path_result(
  L1146: def convert(
  L1332: def convert_type(ty: None, default: None = None) -> StringParamType: ...
  L1334: def convert_type(
  L1338: def convert_type(
  L1341: def convert_type(
  L49: class ParamTypeInfoDict(t.TypedDict):
  L54: class ParamType(t.Generic[_ValueT_co, _InputT_contra], abc.ABC):
  L111: def to_info_dict(self) -> ParamTypeInfoDict:
  L133: def __call__(
  L141: def __call__(
  L148: def __call__(
  L158: def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
  L161: def get_missing_message(self, param: Parameter, ctx: Context | None) -> str | None:
  L204: def fail(
  L213: def shell_complete(
  L231: class CompositeParamType(ParamType[_ValueT_co]):
  L236: def arity(self) -> int: ...  # type: ignore[override]
  L253: class FuncParamType(ParamType[_ValueT_co], t.Generic[_ValueT_contra, _ValueT_co]):
  L257: def __init__(self, func: t.Callable[[_ValueT_contra], _ValueT_co]) -> None:
  L261: def to_info_dict(self) -> FuncParamTypeInfoDict[_ValueT_contra, _ValueT_co]:
  L281: class UnprocessedParamType(ParamType[t.Any]):
  L289: def __repr__(self) -> str:
  L293: class StringParamType(ParamType[str]):
  L314: def __repr__(self) -> str:
  L443: def convert(
  L549: def _try_to_convert_date(self, value: t.Any, format: str) -> datetime | None:
  L555: def convert(
  L582: class _NumberParamTypeBase(
  L655: def convert(
  L725: class IntParamType(_NumberParamTypeBase[t.SupportsInt | t.SupportsIndex, int]):
  L729: def __repr__(self) -> str:
  L757: class FloatParamType(_NumberParamTypeBase[t.SupportsFloat | t.SupportsIndex, float]):
  L761: def __repr__(self) -> str:
  L808: class BoolParamType(ParamType[bool]):
  L845: def str_to_bool(value: str | bool) -> bool | None:
  L873: def __repr__(self) -> str:
  L877: class UUIDParameterType(ParamType[uuid.UUID]):
  L895: def __repr__(self) -> str:
  L973: def convert(
  L1039: class PathInfoDict(ParamTypeInfoDict):
  L1048: class Path(ParamType[str | bytes | os.PathLike[str]]):
  L1093: def __init__(
  L1122: def to_info_dict(self) -> PathInfoDict:
  L1220: def shell_complete(
  L1274: def convert(
  L1296: def _guess_type(
  L331: class Choice(ParamType[_ValueT_co], t.Generic[_ValueT_co]):
  L364: def __init__(
  L370: def to_info_dict(self) -> ChoiceInfoDict[_ValueT_co]:
  L377: def _normalized_mapping(
  L395: def normalize_choice(self, choice: object, ctx: Context | None) -> str:
  L415: def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
  L433: def get_missing_message(self, param: Parameter, ctx: Context | None) -> str:
  L467: def get_invalid_choice_message(self, value: t.Any, ctx: Context | None) -> str:
  L481: def __repr__(self) -> str:
  L484: def shell_complete(
  L507: class DateTimeInfoDict(ParamTypeInfoDict):
  L511: class DateTime(ParamType[datetime]):
  L536: def __init__(self, formats: cabc.Sequence[str] | None = None):
  L543: def to_info_dict(self) -> DateTimeInfoDict:
  L546: def get_metavar(self, param: Parameter, ctx: Context) -> str:
  L578: def __repr__(self) -> str:
  L621: class _NumberRangeBase(
  L631: def __init__(
  L645: def to_info_dict(self) -> NumberRangeInfoDict[_FloatValueT_co]:
  L689: def _clamp(
  L706: def _describe_range(self) -> str:
  L720: def __repr__(self) -> str:
  L733: class IntRange(_NumberRangeBase[int, int], IntParamType):
  L750: def _clamp(self, bound: int, dir: t.Literal[1, -1], open: bool) -> int:
  L765: class FloatRange(_NumberRangeBase[float, float], FloatParamType):
  L783: def __init__(
  L798: def _clamp(self, bound: float, dir: t.Literal[1, -1], open: bool) -> float:
  L899: class FileInfoDict(ParamTypeInfoDict):
  L904: class File(ParamType[t.IO[t.Any]]):
  L943: def __init__(
  L957: def to_info_dict(self) -> FileInfoDict:
  L964: def resolve_lazy_flag(self, value: str | os.PathLike[str]) -> bool:
  L1018: def shell_complete(
  L1035: def _is_file_like(value: t.Any) -> te.TypeIs[t.IO[t.Any]]:
  L1239: class TupleInfoDict(ParamTypeInfoDict):
  L1243: class Tuple(CompositeParamType[tuple[t.Any, ...]]):
  L1257: def __init__(self, types: cabc.Sequence[type[t.Any] | ParamType[t.Any]]) -> None:
  L1260: def to_info_dict(self) -> TupleInfoDict:
  L1267: def name(self) -> str:  # type: ignore[override]
  L1271: def arity(self) -> int:  # type: ignore[override]
  L1396: UNPROCESSED: t.Final[UnprocessedParamType] = UnprocessedParamType()
  L1400: STRING: t.Final[StringParamType] = StringParamType()
  L1404: INT: t.Final[IntParamType] = IntParamType()
  L1408: FLOAT: t.Final[FloatParamType] = FloatParamType()
  L1412: BOOL: t.Final[BoolParamType] = BoolParamType()
  L1415: UUID: t.Final[UUIDParameterType] = UUIDParameterType()
  L1418: class OptionHelpExtra(t.TypedDict, total=False):
src/click/core.py:
  L2557: def type_cast_value(self, ctx: Context, value: t.Any) -> t.Any:
  L3506: def prompt_for_value(self, ctx: Context) -> t.Any:
  L720: def command_path(self) -> str:
  L2509: def consume_value(
  L2613: def value_is_missing(self, value: t.Any) -> bool:
  L2631: def process_value(self, ctx: Context, value: t.Any) -> t.Any:
  L2697: def resolve_envvar_value(self, ctx: Context) -> str | None:
  L2742: def value_from_envvar(self, ctx: Context) -> str | cabc.Sequence[str] | None:
  L3070: def flag_activation_value(self) -> t.Any:
  L3115: def _pick_type(
  L3556: def resolve_envvar_value(self, ctx: Context) -> str | None:
  L3580: def value_from_envvar(self, ctx: Context) -> t.Any:
  L3628: def consume_value(
  L3684: def process_value(self, ctx: Context, value: t.Any) -> t.Any:
  L60: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
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
  L840: def get_help(self) -> str:
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
  L1182: def get_help_option_names(self, ctx: Context) -> list[str]:
  L1197: def get_help_option(self, ctx: Context) -> Option | None:
  L1231: def make_parser(self, ctx: Context) -> _OptionParser:
  L1238: def get_help(self, ctx: Context) -> str:
  L1247: def get_short_help_str(self, limit: int = 45) -> str:
  L1263: def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1282: def format_help_text(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1300: def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1312: def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1324: def format_epilog(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1333: def make_context(
  L1370: def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
  L1406: def invoke(self, ctx: Context) -> t.Any:
  L1422: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L1470: def main(
  L1480: def main(
  L1489: def main(
  L1625: def _main_shell_completion(
  L1657: def __call__(self, *args: t.Any, **kwargs: t.Any) -> t.Any:
  L1662: class _FakeSubclassCheck(type):
  L1663: def __subclasscheck__(cls, subclass: type) -> bool:
  L1666: def __instancecheck__(cls, instance: t.Any) -> bool:
  L1670: class _BaseCommand(Command, metaclass=_FakeSubclassCheck):
  L1677: class Group(Command):
  L1736: def __init__(
  L1791: def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
  L1809: def add_command(self, cmd: Command, name: str | None = None) -> None:
  L1820: def command(self, __func: t.Callable[..., t.Any]) -> Command: ...
  L1823: def command(
  L1827: def command(
  L1869: def group(self, __func: t.Callable[..., t.Any]) -> Group: ...
  L1872: def group(
  L1876: def group(
  L1920: def result_callback(self, replace: bool = False) -> t.Callable[[F], F]:
  L1965: def get_command(self, ctx: Context, cmd_name: str) -> Command | None:
  L1971: def list_commands(self, ctx: Context) -> list[str]:
  L1975: def collect_usage_pieces(self, ctx: Context) -> list[str]:
  L1980: def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1984: def format_commands(self, ctx: Context, formatter: HelpFormatter) -> None:
  L2012: def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
  L2026: def invoke(self, ctx: Context) -> t.Any:
  L2094: def resolve_command(
  L2120: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L2140: class _MultiCommand(Group, metaclass=_FakeSubclassCheck):
  L2147: class CommandCollection(Group):
  L2165: def __init__(
  L2175: def add_source(self, group: Group) -> None:
  L2179: def get_command(self, ctx: Context, cmd_name: str) -> Command | None:
  L2196: def list_commands(self, ctx: Context) -> list[str]:
  L2205: def _check_iter(value: cabc.Iterable[V]) -> cabc.Iterator[V]:
  L2215: class Parameter(ABC):
  L2327: def __init__(
  L2402: def _hide_unset(value: t.Any) -> t.Any:
  L2409: def to_info_dict(self) -> dict[str, t.Any]:
  L2434: def __repr__(self) -> str:
  L2438: def _parse_decls(
  L2443: def human_readable_name(self) -> str:
  L2449: def make_metavar(self, ctx: Context) -> str:
  L2464: def get_default(
  L2469: def get_default(
  L2473: def get_default(
  L2507: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None: ...
  L2758: def handle_parse_result(
  L2846: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L2849: def get_usage_pieces(self, ctx: Context) -> list[str]:
  L2852: def get_error_hint(self, ctx: Context | None) -> str:
  L2862: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L2886: class Option(Parameter):
  L2979: def __init__(
  L3061: def is_bool_flag(self) -> bool:
  L3082: def _infer_flag_kind(
  L3154: def _resolve_lazy_default(self, value: t.Any) -> t.Any:
  L3178: def _validate(self, prompt: bool | str, deprecated: bool | str) -> None:
  L3200: def to_info_dict(self) -> dict[str, t.Any]:
  L3223: def get_default(
  L3257: def get_error_hint(self, ctx: Context | None) -> str:
  L3263: def _parse_decls(
  L3324: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
  L3366: def get_help_spec(self, ctx: Context) -> str:
  L3397: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L3422: def get_help_extra(self, ctx: Context) -> types.OptionHelpExtra:
  L3702: class Argument(Parameter):
  L3717: def __init__(
  L3750: def to_info_dict(self) -> dict[str, t.Any]:
  L3756: def human_readable_name(self) -> str:
  L3761: def make_metavar(self, ctx: Context) -> str:
  L3780: def _parse_decls(
  L3799: def get_usage_pieces(self, ctx: Context) -> list[str]:
  L3802: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L3808: def get_error_hint(self, ctx: Context | None) -> str:
  L3813: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
  L3817: def __getattr__(name: str) -> object:
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
src/click/termui.py:
  L79: def hidden_prompt_func(prompt: str) -> str:
  L85: def _readline_prompt(func: t.Callable[[str], str], text: str, err: bool) -> str:
  L108: def _build_prompt(
  L139: def prompt(
  L154: def prompt(
  L168: def prompt(
  L58: _HIDDEN_INPUT_MASK = "'***'"
  L61: def _mask_hidden_input(message: str, value: str) -> str:
  L128: def _format_default(default: V) -> V | str:
  L289: def confirm(
  L348: def get_pager_file(
  L367: def echo_via_pager(
  L403: def progressbar(
  L423: def progressbar(
  L443: def progressbar(
  L602: def clear() -> None:
  L616: def _interpret_color(color: int | tuple[int, int, int] | str, offset: int = 0) -> str:
  L641: def style(
  L768: def unstyle(text: str) -> str:
  L780: def secho(
  L815: def edit(
  L825: def edit(
  L835: def edit(
  L848: def edit(
  L923: def launch(url: str, wait: bool = False, locate: bool = False) -> int:
  L957: def getchar(echo: bool = False) -> str:
  L987: def raw_terminal() -> AbstractContextManager[int]:
  L993: def pause(info: str | None = None, err: bool = False) -> None:
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
src/click/utils.py:
  L32: def _posixify(name: str) -> str:
  L36: def _safecall(func: t.Callable[P, R]) -> t.Callable[P, R | None]:
  L52: def make_str(value: t.Any) -> str:
  L62: def _make_default_short_help(help: str, max_length: int = 45) -> str:
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
tests/test_termui.py:
  L1734: def convert(self, value, param, ctx):
  L1743: def convert(self, value, param, ctx):
  L1587: FLAG_VALUE_PROMPT_CASES = [
  L1679: def test_flag_value_prompt(
  L1731: class _CustomTypeNoValue(click.ParamType):
  L1740: class _CustomTypeWithRawValue(click.ParamType):
  L1757: def convert(self, value, param, ctx):
  L1770: def convert(self, value, param, ctx):
  L1779: def convert(self, value, param, ctx):
  L1788: def convert(self, value, param, ctx):
  L1797: def convert(self, value, param, ctx):
  L1806: def convert(self, value, param, ctx):
  L1817: def convert(self, value, param, ctx):
  L257: def test_choices_list_in_prompt(runner, monkeypatch):
  L285: def test_file_prompt_default_format(runner, file_kwargs):
  L558: def test_editor_path_normalization(editor_cmd, filenames, expected_args):
  L585: def test_editor_windows_path_normalization(editor_cmd, expected_cmd):
  L1262: def test_tempfile_pager_closes_file_before_unlink(monkeypatch):
  L1326: def test_prompt_required_with_required(runner, prompt_required, required, args, expect):
  L1353: def test_prompt_required_false(runner, args, expect):
  L1377: def test_confirmation_prompt(runner, prompt, input, default, expect):
  L1430: def test_prompt_and_confirm_ansi_respects_color(
  L1447: def test_false_show_default_cause_no_default_display_in_prompt(runner):
  L1487: def test_string_show_default_in_prompt(
  L1523: BOOLEAN_FLAG_PROMPT_CASES = [
  L1749: class _PasswordLengthType(click.ParamType):
  L1763: class _MixedQuotedAndRawType(click.ParamType):
  L1774: class _StaticMessageType(click.ParamType):
  L1783: class _RejectAllRawType(click.ParamType):
  L1792: class _MultiRawType(click.ParamType):
  L1801: class _MultiReprType(click.ParamType):
  L1810: class _ApostropheReprType(click.ParamType):
  L1945: def test_hide_input_confirmation_prompt_mismatch_unaffected(runner):
  L1966: def test_hide_input_value_never_leaks_when_err_true(runner):
  L26: class FakeClock:
  L27: def __init__(self):
  L30: def advance_time(self, seconds=1):
  L33: def time(self):
  L37: def _create_progress(length=10, **kwargs):
  L44: def test_progressbar_strip_regression(runner, monkeypatch):
  L60: def test_progressbar_length_hint(runner, monkeypatch):
  L90: def test_progressbar_no_tty(runner, monkeypatch):
  L101: def test_progressbar_hidden_manual(runner, monkeypatch):
  L114: def test_progressbar_lands_on_final_position(monkeypatch, update_min_steps, drive):
  L144: def test_progressbar_time_per_iteration(runner, avg, expected):
  L150: def test_progressbar_eta(runner, finished, expected):
  L167: def test_progressbar_format_eta(runner, eta, expected):
  L173: def test_progressbar_format_pos(runner, pos, length):
  L186: def test_progressbar_format_bar(runner, length, finished, pos, avg, expected):
  L203: def test_progressbar_format_progress_line(
  L217: def test_progressbar_format_progress_line_with_show_func(runner, test_item):
  L230: def test_progressbar_init_exceptions(runner):
  L235: def test_progressbar_iter_outside_with_exceptions(runner):
  L242: def test_progressbar_is_iterator(runner, monkeypatch):
  L295: def test_secho(runner):
  L306: def test_secho_non_text(runner, value, expect):
  L313: def test_progressbar_yields_all_items(runner):
  L318: def test_progressbar_update(runner, monkeypatch):
  L341: def test_progressbar_item_show_func(runner, monkeypatch):
  L357: def test_progressbar_update_with_item_show_func(runner, monkeypatch):
  L377: def test_progress_bar_update_min_steps(runner):
  L390: def test_getchar_windows(runner, monkeypatch, key_char, echo):
  L403: def test_getchar_special_key_windows(runner, monkeypatch, special_key_char, key_char):
  L416: def test_getchar_windows_exceptions(runner, monkeypatch, key_char, exc):
  L425: def test_fast_edit(runner):
  L431: def test_edit(runner):
  L453: def test_edit_pathlib(runner, tmp_path, use_iterable):
  L597: def test_editor_env_passed_through():
  L607: def test_editor_failure_exception():
  L614: def test_editor_nonexistent_exception():
  L666: def test_pager_shlex_split(pager_env, expected_parts):
  L676: def _get_real_pager_command() -> str:
  L688: def _run_get_pager_file_with_real_pager(monkeypatch, capfd, writer, color=False):
  L704: def _write_pager_from_multiple_sites(pager):
  L741: def test_get_pager_file_with_real_pager_binary_stream(
  L763: def test_echo_via_pager_real_pager_handles_ansi(monkeypatch, capfd, color, expected):
  L787: def test_pipepager_less_detection_matches_stem(
  L807: def test_pipepager_less_detection_case_insensitive_on_windows(monkeypatch, tmp_path):
  L825: class _FakePagerProcess:
  L829: def __init__(self, argv, *, env=None, **kwargs):
  L835: def terminate(self):
  L838: def wait(self):
  L891: def test_pipepager_less_raw_mode_detection(
  L928: def test_echo_via_pager_streams_each_write(monkeypatch):
  L966: def test_get_pager_file_pager_missing_binary_falls_back(monkeypatch, tmp_path):
  L988: def test_get_pager_file_pager_unset_falls_back_when_no_default(monkeypatch, tmp_path):
  L1008: def test_get_pager_file_missing_pager_keeps_borrowed_stream_open(monkeypatch):
  L1040: def test_echo_via_pager_tty_pager_missing(runner, monkeypatch):
  L1070: def test_get_pager_file_nullpager_wraps_textio_stream(
  L1091: def test_get_pager_file_nullpager_keeps_stringio_stream(monkeypatch):
  L1108: def test_get_pager_file_flushes_stream_on_exception(monkeypatch):
  L1138: def test_get_pager_file_close_does_not_close_borrowed_stream(monkeypatch):
  L1170: def _force_tempfile_pager(monkeypatch, pager_cmd="cat"):
  L1189: def _page_with_get_pager_file():
  L1194: def _page_with_echo_via_pager():
  L1206: def test_tempfile_pager_accepts_text(monkeypatch, capfd, page):
  L1230: def test_tempfile_pager_handles_ansi(monkeypatch, capfd, color, expected):
  L1246: def test_tempfile_pager_passes_pager_params(monkeypatch, capfd):
  L1311: def test_editor_unclosed_quote():
  L1511: REPEAT = object()
  L1517: INVALID = object()
