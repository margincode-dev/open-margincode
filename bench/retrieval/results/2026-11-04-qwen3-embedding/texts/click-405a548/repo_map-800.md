src/click/core.py:
  L60: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
  L1182: def get_help_option_names(self, ctx: Context) -> list[str]:
  L1197: def get_help_option(self, ctx: Context) -> Option | None:
  L2846: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L3366: def get_help_spec(self, ctx: Context) -> str:
  L3397: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L3422: def get_help_extra(self, ctx: Context) -> types.OptionHelpExtra:
  L3802: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L174: class ParameterSource(enum.IntEnum):
  L840: def get_help(self) -> str:
  L936: def set_parameter_source(self, name: str, source: ParameterSource) -> None:
  L945: def get_parameter_source(self, name: str) -> ParameterSource | None:
  L1238: def get_help(self, ctx: Context) -> str:
  L1247: def get_short_help_str(self, limit: int = 45) -> str:
  L1263: def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1282: def format_help_text(self, ctx: Context, formatter: HelpFormatter) -> None:
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
  L2509: def consume_value(
  L2557: def type_cast_value(self, ctx: Context, value: t.Any) -> t.Any:
  L2613: def value_is_missing(self, value: t.Any) -> bool:
  L2631: def process_value(self, ctx: Context, value: t.Any) -> t.Any:
  L2697: def resolve_envvar_value(self, ctx: Context) -> str | None:
  L2742: def value_from_envvar(self, ctx: Context) -> str | cabc.Sequence[str] | None:
  L2758: def handle_parse_result(
  L2849: def get_usage_pieces(self, ctx: Context) -> list[str]:
  L2852: def get_error_hint(self, ctx: Context | None) -> str:
  L2862: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L2886: class Option(Parameter):
  L2979: def __init__(
  L3061: def is_bool_flag(self) -> bool:
  L3070: def flag_activation_value(self) -> t.Any:
  L3082: def _infer_flag_kind(
  L3115: def _pick_type(
  L3154: def _resolve_lazy_default(self, value: t.Any) -> t.Any:
  L3178: def _validate(self, prompt: bool | str, deprecated: bool | str) -> None:
