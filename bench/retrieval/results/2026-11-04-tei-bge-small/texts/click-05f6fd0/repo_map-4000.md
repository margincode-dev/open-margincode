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
