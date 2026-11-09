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
  L2461: def human_readable_name(self) -> str:
  L60: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
  L174: class ParameterSource(enum.IntEnum):
  L936: def set_parameter_source(self, name: str, source: ParameterSource) -> None:
  L945: def get_parameter_source(self, name: str) -> ParameterSource | None:
  L2215: class Parameter(ABC):
  L2334: def __init__(
  L2419: def _hide_unset(value: t.Any) -> t.Any:
  L2426: def to_info_dict(self) -> dict[str, t.Any]:
  L2452: def __repr__(self) -> str:
  L2456: def _parse_decls(
  L2467: def make_metavar(self, ctx: Context) -> str:
  L2482: def get_default(
  L2487: def get_default(
  L2491: def get_default(
  L2525: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None: ...
  L2527: def consume_value(
  L2575: def type_cast_value(self, ctx: Context, value: t.Any) -> t.Any:
  L2631: def value_is_missing(self, value: t.Any) -> bool:
  L2649: def process_value(self, ctx: Context, value: t.Any) -> t.Any:
  L2715: def resolve_envvar_value(self, ctx: Context) -> str | None:
  L2760: def value_from_envvar(self, ctx: Context) -> str | cabc.Sequence[str] | None:
  L2776: def handle_parse_result(
  L2864: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L2867: def get_usage_pieces(self, ctx: Context) -> list[str]:
  L2870: def get_error_hint(self, ctx: Context | None) -> str:
  L2880: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L3753: def human_readable_name(self) -> str:
  L63: def _complete_visible_commands(
  L82: def _check_nested_chain(
  L102: def _echo_aborted() -> None:
  L107: def _format_deprecated_label(deprecated: bool | str) -> str:
  L115: def _format_deprecated_suffix(deprecated: bool | str) -> str:
  L124: def batch(iterable: cabc.Iterable[V], batch_size: int) -> list[tuple[V, ...]]:
  L129: def augment_usage_errors(
  L147: def iter_params_for_processing(
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
  L840: def get_help(self) -> str:
  L846: def _make_sub_context(self, command: Command) -> Context:
  L855: def invoke(
  L860: def invoke(self, callback: Command, /, *args: t.Any, **kwargs: t.Any) -> t.Any: ...
  L862: def invoke(
  L917: def forward(self, cmd: Command, /, *args: t.Any, **kwargs: t.Any) -> t.Any:
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
