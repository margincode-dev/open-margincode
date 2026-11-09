src/click/core.py:
  L1247: def get_short_help_str(self, limit: int = 45) -> str:
  L60: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
  L840: def get_help(self) -> str:
  L1182: def get_help_option_names(self, ctx: Context) -> list[str]:
  L1197: def get_help_option(self, ctx: Context) -> Option | None:
  L1238: def get_help(self, ctx: Context) -> str:
  L1263: def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
  L1282: def format_help_text(self, ctx: Context, formatter: HelpFormatter) -> None:
  L2864: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L3379: def get_help_spec(self, ctx: Context) -> str:
  L3410: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
  L3435: def get_help_extra(self, ctx: Context) -> types.OptionHelpExtra:
  L3799: def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
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
  L2334: def __init__(
  L2419: def _hide_unset(value: t.Any) -> t.Any:
  L2426: def to_info_dict(self) -> dict[str, t.Any]:
  L2452: def __repr__(self) -> str:
  L2456: def _parse_decls(
  L2461: def human_readable_name(self) -> str:
  L2467: def make_metavar(self, ctx: Context) -> str:
  L2482: def get_default(
  L2487: def get_default(
  L2491: def get_default(
  L2525: def add_to_parser(self, parser: _OptionParser, ctx: Context) -> None: ...
  L2527: def consume_value(
  L2575: def type_cast_value(self, ctx: Context, value: t.Any) -> t.Any:
