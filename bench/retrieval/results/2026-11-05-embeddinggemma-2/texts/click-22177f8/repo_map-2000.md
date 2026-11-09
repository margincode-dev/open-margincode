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
