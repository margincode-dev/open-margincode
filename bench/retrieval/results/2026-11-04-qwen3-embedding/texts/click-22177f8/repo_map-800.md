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
