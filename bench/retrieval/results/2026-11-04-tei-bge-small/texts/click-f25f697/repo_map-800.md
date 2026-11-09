src/click/core.py:
  L3802: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L60: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
  L840: def get_help(self) -> str:
  L1182: def get_help_option_names(self, ctx: Context) -> list[str]:
  L1197: def get_help_option(self, ctx: Context) -> Option | None:
  L1238: def get_help(self, ctx: Context) -> str:
  L1247: def get_short_help_str(self, limit: int = 45) -> str:
  L1263: def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1282: def format_help_text(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1312: def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1971: def list_commands(self, ctx: Context) -> list[str]:
  L2196: def list_commands(self, ctx: Context) -> list[str]:
  L2846: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
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
