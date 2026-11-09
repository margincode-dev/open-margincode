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
  L3200: def to_info_dict(self) -> dict[str, t.Any]:
  L3223: def get_default(
  L3257: def get_error_hint(self, ctx: Context | None) -> str:
  L3263: def _parse_decls(
  L3324: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
  L3506: def prompt_for_value(self, ctx: Context) -> t.Any:
  L3556: def resolve_envvar_value(self, ctx: Context) -> str | None:
  L3580: def value_from_envvar(self, ctx: Context) -> t.Any:
  L3628: def consume_value(
  L3684: def process_value(self, ctx: Context, value: t.Any) -> t.Any:
  L3702: class Argument(Parameter):
  L3717: def __init__(
  L3750: def to_info_dict(self) -> dict[str, t.Any]:
  L3756: def human_readable_name(self) -> str:
  L3761: def make_metavar(self, ctx: Context) -> str:
  L3780: def _parse_decls(
  L3799: def get_usage_pieces(self, ctx: Context) -> list[str]:
  L3808: def get_error_hint(self, ctx: Context | None) -> str:
  L3813: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None:
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
  L1231: def make_parser(self, ctx: Context) -> _OptionParser:
  L1300: def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1312: def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1324: def format_epilog(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1333: def make_context(
  L1370: def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
  L1406: def invoke(self, ctx: Context) -> t.Any:
  L1422: def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
  L1470: def main(
  L1480: def main(
