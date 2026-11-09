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
