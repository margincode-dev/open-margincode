## Task
# click-05f6fd0 · click-05f6fd0 · open
Goal: Always return a help record for an `Argument`
Workspace: /root/bench/av_tei/ws/rec09fc-click-05f6fd0/click-05f6fd0
Changed (0): nothing yet

## Map
- src/click/core.py
  L60: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
  L63: def _complete_visible_commands( ctx: Context, incomplete: str ) -> cabc.Iterator[tuple[str, Command]]:
  L82: def _check_nested_chain( base_command: Group, cmd_name: str, cmd: Command, register: bool = False ) -> None:
  L102: def _echo_aborted() -> None:
  L107: def _format_deprecated_label(deprecated: bool | str) -> str:
  L115: def _format_deprecated_suffix(deprecated: bool | str) -> str:
  L124: def batch(iterable: cabc.Iterable[V], batch_size: int) -> list[tuple[V, ...]]:
  L129: def augment_usage_errors( ctx: Context, param: Parameter | None = None ) -> cabc.Generator[None]:
  L147: def iter_params_for_processing( invocation_order: cabc.Sequence[Parameter], declaration_order: cabc.Sequence[Parameter], ) -> list[Parameter]:
  L174: class ParameterSource(enum.IntEnum):
  L213: class Context:
  L345: def __init__( self, command: Command, parent: Context | None = None, info_name: str | None = None, obj: t.Any | None = None, auto_envvar_prefix: str | None = None, default_map: cabc.MutableMapping[str, t.Any] | None = None, terminal_width: int | None = None, max_content_width: int | None = None, resilient_parsing: bool = False, allow_extra_args: bool | None = None, allow_interspersed_args: bool | None = None, ignore_unknown_options: bool | None = None, help_option_names: list[str] | None = None, token_normalize_func: t.Callable[[str], str] | None = None, color: bool | None = None, show_default: bool | None = None, ) -> None: #: the parent context or `None` if none exists.
  L522: def protected_args(self) -> list[str]:
  L533: def to_info_dict(self) -> dict[str, t.Any]:
  L554: def __enter__(self) -> Self:
  L559: def __exit__( self, exc_type: type[BaseException] | None, exc_value: BaseException | None, tb: TracebackType | None, ) -> bool | None:
  L574: def scope(self, cleanup: bool = True) -> cabc.Generator[Context]:
  L612: def meta(self) -> dict[str, t.Any]:
  L639: def make_formatter(self) -> HelpFormatter:
  L653: def with_resource(self, context_manager: AbstractContextManager[V]) -> V:
  L682: def call_on_close(self, f: t.Callable[..., t.Any]) -> t.Callable[..., t.Any]:
  L694: def close(self) -> None:
  L701: def _close_with_exception_info( self, exc_type: type[BaseException] | None, exc_value: BaseException | None, tb: TracebackType | None, ) -> bool | None:
  L720: def command_path(self) -> str:
  L738: def find_root(self) -> Context:
  L745: def find_object(self, object_type: type[V]) -> V | None:
  L757: def ensure_object(self, object_type: type[V]) -> V:
  L766: def _default_map_has(self, name: str | None) -> bool:
  L781: def lookup_default( self, name: str, call: t.Literal[True] = True ) -> t.Any | None:
  L812: def fail(self, message: str) -> t.NoReturn:
- src/click/decorators.py
  L25: FC = t.TypeVar("FC", bound="_AnyCallable | Command")
  L28: def pass_context(f: t.Callable[te.Concatenate[Context, P], R]) -> t.Callable[P, R]:
  L39: def pass_obj(f: t.Callable[te.Concatenate[T, P], R]) -> t.Callable[P, R]:
  L51: def make_pass_decorator( object_type: type[T], ensure: bool = False ) -> t.Callable[[t.Callable[te.Concatenate[T, P], R]], t.Callable[P, R]]:
  L100: def pass_meta_key( key: str, *, doc_description: str | None = None ) -> t.Callable[[t.Callable[te.Concatenate[T, P], R]], t.Callable[P, R]]:
  L138: def command(name: _AnyCallable) -> Command:
  L263: def group(name: _AnyCallable) -> Group:
  L314: def _param_memo(f: t.Callable[..., t.Any], param: Parameter) -> None:
  L324: def argument( *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
  L352: def option( *param_decls: str, cls: type[Option] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
  L380: def confirmation_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
  L404: def password_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
  L421: def version_option( version: str | None = None, *param_decls: str, package_name: str | None = None, prog_name: str | None = None, message: str | None = None, **kwargs: t.Any, ) -> t.Callable[[FC], FC]:
  L561: def custom_version_option( callback: t.Callable[[Context], str], *param_decls: str, **kwargs: t.Any, ) -> t.Callable[[FC], FC]:
  L603: def help_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
- src/click/shell_completion.py
  L19: def shell_complete( cli: Command, ctx_args: cabc.MutableMapping[str, t.Any], prog_name: str, complete_var: str, instruction: str, ) -> t.Literal[0, 1]:
  L67: class CompletionItem(t.Generic[_ValueT_co]):
  L88: def __init__( self, value: _ValueT_co, type: str = "plain", help: str | None = None, **kwargs: t.Any, ) -> None:
  L100: def __getattr__(self, name: str) -> t.Any:
  L105: _SOURCE_BASH = """\
  L143: _SOURCE_ZSH = """\
  L187: _SOURCE_FISH = """\
  L214: _SOURCE_POWERSHELL = """\
  L272: class _SourceVarsDict(t.TypedDict):
  L278: class ShellComplete:
  L307: def __init__( self, cli: Command, ctx_args: cabc.MutableMapping[str, t.Any], prog_name: str, complete_var: str, ) -> None:
  L320: def func_name(self) -> str:
  L327: def source_vars(self) -> _SourceVarsDict:
  L339: def source(self) -> str:
  L347: def get_completion_args(self) -> tuple[list[str], str]:
  L354: def get_completions( self, args: list[str], incomplete: str ) -> list[CompletionItem[str]]:
  L368: def format_completion(self, item: CompletionItem[str]) -> str:
  L376: def complete(self) -> str:
  L389: class BashComplete(ShellComplete):
  L396: def _check_version() -> None:
  L428: def source(self) -> str:
  L432: def get_completion_args(self) -> tuple[list[str], str]:
  L444: def format_completion(self, item: CompletionItem[t.Any]) -> str:
  L448: class ZshComplete(ShellComplete):
  L454: def get_completion_args(self) -> tuple[list[str], str]:
  L466: def format_completion(self, item: CompletionItem[str]) -> str:
  L484: class FishComplete(ShellComplete):
  L490: def get_completion_args(self) -> tuple[list[str], str]:
  L504: def format_completion(self, item: CompletionItem[str]) -> str:
  L521: class PowerShellComplete(ShellComplete):
- tests/test_arguments.py
  L10: def test_nargs_star(runner):
  L23: def test_nargs_tup(runner):
  L46: def test_nargs_tup_composite(runner, opts):
  L58: def test_nargs_mismatch_with_tuple_type():
  L67: def test_nargs_err(runner):
  L82: def test_bytes_args(runner, monkeypatch):
  L105: def test_file_args(runner, tmp_path):
  L127: def test_path_allow_dash(runner):
  L138: def test_file_atomics(runner, tmp_path):
  L156: def test_stdout_default(runner):
  L181: def test_nargs_envvar(runner, nargs, value, expect):
  L201: def test_nargs_envvar_only_if_values_empty(runner):
  L214: def test_empty_nargs(runner):
  L234: def test_missing_arg(runner):
  L269: def test_required_argument(value, expect_missing, processed_value):
  L284: def test_implicit_non_required(runner):
  L295: def test_argument_help(runner):
  L314: def test_argument_help_options_only_no_arguments_section(runner):
  L327: def test_argument_help_lists_undocumented_arguments(runner):
  L349: def test_argument_help_undocumented_arguments_only_no_section(runner):
  L363: def test_argument_help_deprecated_without_help_lists_all(runner):
  L383: def test_argument_help_empty_string_lists_all(runner):
  L405: def test_argument_help_optional_metavar(runner):
  L417: def test_deprecated_usage(runner):
  L441: def test_argument_metavar_marks_optional(runner, kwargs, expected):
  L458: def test_deprecated_usage_help_record(runner, deprecated, expected_label):
  L471: def test_deprecated_usage_help_record_without_help(runner):
  L489: def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
  L499: def test_deprecated_warning(runner, deprecated):
  L520: def test_deprecated_required(runner):
- repo map (178 files):
  .devcontainer/ (2 files)
    devcontainer.json
    on-create-command.sh
  .editorconfig
  .github/ (10 files)
    ISSUE_TEMPLATE/ (3 files)
    pull_request_template.md
    workflows/ (6 files)
  .gitignore
  .pre-commit-config.yaml
  .readthedocs.yaml
  CHANGES.md
  LICENSE.txt
  README.md
  docs/ (41 files)
    _static/ (3 files)
    advanced.md
    api.md
    arguments.md
    changes.md
    click-concepts.md
    command-line-reference.md
    commands-and-groups.md
    commands.md
    complex.md
    conf.py
    contrib.md
    contributing.md
    design-opinions.md
    documentation.md
    entry-points.md
    exceptions.md
    extending-click.md
    faqs.md
    handling-files.md
    index.md
    license.md
    option-decorators.md
    options.md
    parameter-types.md
    parameters.md
    prompts.md
    quickstart.md
    setuptools.md
    shell-completion.md
    standalone-apps.md
    support-multiple-versions.md
    testing.md
    unicode-support.md
    upgrade-guides.md
    utils.md
    virtualenv.md
    why.md
    wincmd.md
  examples/ (39 files)
    README
    aliases/ (4 files)
    colors/ (3 files)
    completion/ (3 files)
    complex/ (7 files)
    imagepipe/ (6 files)
    inout/ (3 files)
    naval/ (3 files)
    repo/ (3 files)
    termui/ (3 files)
    validation/ (3 files)
  pyproject.toml
  src/ (18 files)
    click/ (18 files)
  tests/ (59 files)
    conftest.py
    test_abort_interrupt.py
    test_basic.py
    test_chain.py
    test_command_decorators.py
    test_commands.py
    test_compat.py
    test_context.py
    test_custom_classes.py
    test_defaults.py
    test_deprecations.py
    test_exceptions/ (2 files)
    test_formatting.py
    test_imports.py
    test_info_dict.py
    test_normalization.py
    test_options.py
    test_parser.py
    test_shell_completion.py
    test_stream_lifecycle.py
    test_termui.py
    test_testing.py
    test_types/ (9 files)
    test_utils/ (14 files)
    …

## Code
- sym:5955 Command.format_arguments (src/click/core.py:1312-1326)
  def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:5954 Command.format_options (src/click/core.py:1300-1310)
  def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:6048 Argument.get_help_record (src/click/core.py:3806-3810)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:6041 Argument (src/click/core.py:3706-3818)
  class Argument(Parameter):
- sym:6034 Option.get_help_record (src/click/core.py:3401-3424)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:6016 Parameter.get_help_record (src/click/core.py:2850-2851)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:6033 Option.get_help_spec (src/click/core.py:3370-3399)
  def get_help_spec(self, ctx: Context) -> str:
- sym:6068 argument (src/click/decorators.py:324-349)
  def argument( *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:5948 Command.get_help_option (src/click/core.py:1197-1229)
  def get_help_option(self, ctx: Context) -> Option | None:
- sym:6074 help_option (src/click/decorators.py:603-627)
  def help_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
- sym:6084 UsageError.show (src/click/exceptions.py:87-111)
  def show(self, file: t.IO[t.Any] | None = None) -> None:
- sym:6446 test_deprecated_usage_help_record (tests/test_arguments.py:458-468)
  def test_deprecated_usage_help_record(runner, deprecated, expected_label):
- sym:6447 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:471-481)
  def test_deprecated_usage_help_record_without_help(runner):
- sym:5950 Command.get_help (src/click/core.py:1238-1245)
  def get_help(self, ctx: Context) -> str:
- sym:6035 Option.get_help_extra (src/click/core.py:3426-3508)
  def get_help_extra(self, ctx: Context) -> types.OptionHelpExtra:
- sym:5952 Command.format_help (src/click/core.py:1263-1280)
  def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:6437 test_argument_help (tests/test_arguments.py:295-311)
  def test_argument_help(runner):
- sym:6443 test_argument_help_optional_metavar (tests/test_arguments.py:405-414)
  def test_argument_help_optional_metavar(runner):
- sym:6441 test_argument_help_deprecated_without_help_lists_all (tests/test_arguments.py:363-380)
  def test_argument_help_deprecated_without_help_lists_all(runner):
- sym:6442 test_argument_help_empty_string_lists_all (tests/test_arguments.py:383-402)
  def test_argument_help_empty_string_lists_all(runner):
- sym:6440 test_argument_help_undocumented_arguments_only_no_section (tests/test_arguments.py:349-360)
  def test_argument_help_undocumented_arguments_only_no_section(runner):
- sym:5944 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:5971 Group.__init__ (src/click/core.py:1740-1793)
  def __init__( self, name: str | None = None, commands: cabc.MutableMapping[str, Command] | cabc.Sequence[Command] | None = None, invoke_without_command: bool = False, no_args_is_help: bool | None = None, subcommand_metavar: str | None = None, chain: bool = False, result_callback: t.Callable[..., t.Any] | None = None, **kwargs: t.Any, ) -> None:
- sym:5997 Parameter (src/click/core.py:2219-2887)
  class Parameter(ABC):
- sym:6052 FC (src/click/decorators.py:25-25)
  FC = t.TypeVar("FC", bound="_AnyCallable | Command")
- sym:6067 _param_memo (src/click/decorators.py:314-321)
  def _param_memo(f: t.Callable[..., t.Any], param: Parameter) -> None:
- sym:6197 _is_incomplete_argument (src/click/shell_completion.py:639-659)
  def _is_incomplete_argument(ctx: Context, param: Parameter) -> bool:
- sym:6693 test_count_default_type_help (tests/test_options.py:780-786)
  def test_count_default_type_help():
- sym:6654 test_argument_to_info_dict_help (tests/test_info_dict.py:292-294)
  def test_argument_to_info_dict_help(help_in, help_out):
- sym:6714 test_option_custom_class_reusable (tests/test_options.py:1160-1185)
  def test_option_custom_class_reusable(runner):
- sym:6448 test_deprecated_empty_help_no_leading_space (tests/test_arguments.py:489-495)
  def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
- sym:6439 test_argument_help_lists_undocumented_arguments (tests/test_arguments.py:327-346)
  def test_argument_help_lists_undocumented_arguments(runner):
- sym:6438 test_argument_help_options_only_no_arguments_section (tests/test_arguments.py:314-324)
  def test_argument_help_options_only_no_arguments_section(runner):
- sym:6665 test_help_spec (tests/test_options.py:85-96)
  def test_help_spec(param_decls, kwargs, hidden, expected):
- sym:6645 HELP_OPTION (tests/test_info_dict.py:9-29)
  HELP_OPTION = (
- sym:6731 test_show_default_precedence (tests/test_options.py:1439-1444)
  def test_show_default_precedence(ctx_value, opt_value, extra_value, expect):
- sym:6692 test_intrange_default_help_text (tests/test_options.py:772-777)
  def test_intrange_default_help_text(type, expect):
- sym:6625 test_formatting_usage_custom_help (tests/test_formatting.py:236-249)
  def test_formatting_usage_custom_help(runner):
- sym:6624 test_formatting_usage_error_no_help (tests/test_formatting.py:221-233)
  def test_formatting_usage_error_no_help(runner):
- sym:6464 test_param_named_help (tests/test_basic.py:46-70)
  def test_param_named_help(runner, help_names, params, args, expected_output):
- sym:6691 test_dynamic_default_help_special_method (tests/test_options.py:749-760)
  def test_dynamic_default_help_special_method(runner):
- sym:6664 test_deprecated_empty_help_no_leading_space (tests/test_options.py:66-72)
  def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
- sym:6646 NAME_ARGUMENT (tests/test_info_dict.py:30-45)
  NAME_ARGUMENT = (
- sym:6712 test_option_custom_class (tests/test_options.py:1108-1121)
  def test_option_custom_class(runner):
- sym:6730 test_do_not_show_default_empty_multiple (tests/test_options.py:1413-1421)
  def test_do_not_show_default_empty_multiple():
- sym:6723 test_show_default_string (tests/test_options.py:1350-1356)
  def test_show_default_string(runner):
- sym:6466 test_option_reusing_help_flag (tests/test_basic.py:90-110)
  def test_option_reusing_help_flag(runner):
- sym:6694 test_file_type_help_default (tests/test_options.py:789-802)
  def test_file_type_help_default():
- sym:6729 test_do_not_show_no_default (tests/test_options.py:1404-1410)
  def test_do_not_show_no_default(runner):
- sym:6458 test_subcommand_help (tests/test_arguments.py:703-719)
  def test_subcommand_help(runner):
- sym:6721 test_show_true_default_boolean_flag_value (tests/test_options.py:1315-1329)
  def test_show_true_default_boolean_flag_value(runner):
- sym:6626 test_formatting_usage_error_help_hint (tests/test_formatting.py:285-314)
  def test_formatting_usage_error_help_hint( runner, help_names, extra_options, expected_hint ):
- sym:6722 test_hide_false_default_boolean_flag_value (tests/test_options.py:1333-1347)
  def test_hide_false_default_boolean_flag_value(runner, default):
- sym:6720 test_show_default_boolean_flag_name (tests/test_options.py:1298-1312)
  def test_show_default_boolean_flag_name(runner, default, expect):
- sym:6655 test_argument_to_info_dict_nargs (tests/test_info_dict.py:297-301)
  def test_argument_to_info_dict_nargs():
- sym:6421 test_nargs_star (tests/test_arguments.py:10-20)
  def test_nargs_star(runner):
- sym:6422 test_nargs_tup (tests/test_arguments.py:23-34)
  def test_nargs_tup(runner):
- sym:6423 test_nargs_tup_composite (tests/test_arguments.py:46-55)
  def test_nargs_tup_composite(runner, opts):
- sym:6424 test_nargs_mismatch_with_tuple_type (tests/test_arguments.py:58-64)
  def test_nargs_mismatch_with_tuple_type():
- sym:6425 test_nargs_err (tests/test_arguments.py:67-79)
  def test_nargs_err(runner):
- sym:6426 test_bytes_args (tests/test_arguments.py:82-102)
  def test_bytes_args(runner, monkeypatch):
- sym:6427 test_file_args (tests/test_arguments.py:105-124)
  def test_file_args(runner, tmp_path):
- sym:6428 test_path_allow_dash (tests/test_arguments.py:127-135)
  def test_path_allow_dash(runner):
- sym:6429 test_file_atomics (tests/test_arguments.py:138-153)
  def test_file_atomics(runner, tmp_path):
- sym:6430 test_stdout_default (tests/test_arguments.py:156-167)
  def test_stdout_default(runner):
- sym:6431 test_nargs_envvar (tests/test_arguments.py:181-198)
  def test_nargs_envvar(runner, nargs, value, expect):
- sym:6432 test_nargs_envvar_only_if_values_empty (tests/test_arguments.py:201-211)
  def test_nargs_envvar_only_if_values_empty(runner):
- sym:6433 test_empty_nargs (tests/test_arguments.py:214-231)
  def test_empty_nargs(runner):
- sym:6434 test_missing_arg (tests/test_arguments.py:234-242)
  def test_missing_arg(runner):
- sym:6435 test_required_argument (tests/test_arguments.py:269-281)
  def test_required_argument(value, expect_missing, processed_value):
- sym:6436 test_implicit_non_required (tests/test_arguments.py:284-292)
  def test_implicit_non_required(runner):
- sym:6444 test_deprecated_usage (tests/test_arguments.py:417-425)
  def test_deprecated_usage(runner):
- sym:6445 test_argument_metavar_marks_optional (tests/test_arguments.py:441-451)
  def test_argument_metavar_marks_optional(runner, kwargs, expected):
- sym:6449 test_deprecated_warning (tests/test_arguments.py:499-517)
  def test_deprecated_warning(runner, deprecated):
- sym:6450 test_deprecated_required (tests/test_arguments.py:520-522)
  def test_deprecated_required(runner):
- sym:6451 test_eat_options (tests/test_arguments.py:525-538)
  def test_eat_options(runner):
- sym:6452 test_nargs_star_ordering (tests/test_arguments.py:541-551)
  def test_nargs_star_ordering(runner):
- sym:6453 test_nargs_specified_plus_star_ordering (tests/test_arguments.py:554-564)
  def test_nargs_specified_plus_star_ordering(runner):

---
tokens: 5681/8000
