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
