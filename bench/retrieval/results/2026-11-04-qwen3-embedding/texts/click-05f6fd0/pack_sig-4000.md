## Task
# click-05f6fd0 · click-05f6fd0 · open
Goal: Always return a help record for an `Argument`
Workspace: /root/bench/ws/rebf815-click-05f6fd0/click-05f6fd0
Changed (0): nothing yet

## Map
- src/click/core.py
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
- repo map (178 files):
  .devcontainer/ (2 files)
  .editorconfig
  .github/ (10 files)
  .gitignore
  .pre-commit-config.yaml
  .readthedocs.yaml
  CHANGES.md
  LICENSE.txt
  README.md
  docs/ (41 files)
  examples/ (39 files)
  pyproject.toml
  src/ (18 files)
  tests/ (59 files)
  uv.lock

## Code
- sym:5955 Command.format_arguments (src/click/core.py:1312-1326)
  def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:6041 Argument (src/click/core.py:3706-3818)
  class Argument(Parameter):
- sym:6048 Argument.get_help_record (src/click/core.py:3806-3810)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:6016 Parameter.get_help_record (src/click/core.py:2850-2851)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:6034 Option.get_help_record (src/click/core.py:3401-3424)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:6033 Option.get_help_spec (src/click/core.py:3370-3399)
  def get_help_spec(self, ctx: Context) -> str:
- sym:5954 Command.format_options (src/click/core.py:1300-1310)
  def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:6068 argument (src/click/decorators.py:324-349)
  def argument( *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:5948 Command.get_help_option (src/click/core.py:1197-1229)
  def get_help_option(self, ctx: Context) -> Option | None:
- sym:6043 Argument.to_info_dict (src/click/core.py:3754-3757)
  def to_info_dict(self) -> dict[str, t.Any]:
- sym:6042 Argument.__init__ (src/click/core.py:3721-3752)
  def __init__( self, param_decls: cabc.Sequence[str], required: bool | None = None, help: str | None = None, **attrs: t.Any, ) -> None: # Auto-detect the requirement status of the argument if not explicitly set.
- sym:6049 Argument.get_error_hint (src/click/core.py:3812-3815)
  def get_error_hint(self, ctx: Context | None) -> str:
- sym:6665 test_help_spec (tests/test_options.py:85-96)
  def test_help_spec(param_decls, kwargs, hidden, expected):
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
- sym:6446 test_deprecated_usage_help_record (tests/test_arguments.py:458-468)
  def test_deprecated_usage_help_record(runner, deprecated, expected_label):
- sym:6447 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:471-481)
  def test_deprecated_usage_help_record_without_help(runner):
- sym:6448 test_deprecated_empty_help_no_leading_space (tests/test_arguments.py:489-495)
  def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
- sym:6443 test_argument_help_optional_metavar (tests/test_arguments.py:405-414)
  def test_argument_help_optional_metavar(runner):
- sym:6437 test_argument_help (tests/test_arguments.py:295-311)
  def test_argument_help(runner):
- sym:6440 test_argument_help_undocumented_arguments_only_no_section (tests/test_arguments.py:349-360)
  def test_argument_help_undocumented_arguments_only_no_section(runner):
- sym:6441 test_argument_help_deprecated_without_help_lists_all (tests/test_arguments.py:363-380)
  def test_argument_help_deprecated_without_help_lists_all(runner):
- sym:6731 test_show_default_precedence (tests/test_options.py:1439-1444)
  def test_show_default_precedence(ctx_value, opt_value, extra_value, expect):
- sym:6692 test_intrange_default_help_text (tests/test_options.py:772-777)
  def test_intrange_default_help_text(type, expect):
- sym:6693 test_count_default_type_help (tests/test_options.py:780-786)
  def test_count_default_type_help():
- sym:6654 test_argument_to_info_dict_help (tests/test_info_dict.py:292-294)
  def test_argument_to_info_dict_help(help_in, help_out):
- sym:6691 test_dynamic_default_help_special_method (tests/test_options.py:749-760)
  def test_dynamic_default_help_special_method(runner):
- sym:6664 test_deprecated_empty_help_no_leading_space (tests/test_options.py:66-72)
  def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
- sym:6712 test_option_custom_class (tests/test_options.py:1108-1121)
  def test_option_custom_class(runner):
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
- sym:6439 test_argument_help_lists_undocumented_arguments (tests/test_arguments.py:327-346)
  def test_argument_help_lists_undocumented_arguments(runner):
- sym:6442 test_argument_help_empty_string_lists_all (tests/test_arguments.py:383-402)
  def test_argument_help_empty_string_lists_all(runner):
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
tokens: 3349/4000
