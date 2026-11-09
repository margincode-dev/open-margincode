## Task
# click-405a548 · click-405a548-2 · open
Goal: Move common `help` argument from `Option` and `Argument` to `Parameter`
Workspace: /root/bench/ws/rebf815-click-405a548/click-405a548-2
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
- src/click/exceptions.py
  L19: def _join_param_hints(param_hint: cabc.Sequence[str] | str | None) -> str | None:
  L26: def _format_possibilities(possibilities: list[str]) -> str:
  L35: class ClickException(Exception):
  L44: def __init__(self, message: str) -> None:
  L51: def format_message(self) -> str:
  L54: def __str__(self) -> str:
  L57: def show(self, file: t.IO[t.Any] | None = None) -> None:
  L68: class UsageError(ClickException):
  L82: def __init__(self, message: str, ctx: Context | None = None) -> None:
  L87: def show(self, file: t.IO[t.Any] | None = None) -> None:
  L114: class BadParameter(UsageError):
  L135: def __init__( self, message: str, ctx: Context | None = None, param: Parameter | None = None, param_hint: cabc.Sequence[str] | str | None = None, ) -> None:
  L146: def format_message(self) -> str:
  L159: class MissingParameter(BadParameter):
  L173: def __init__( self, message: str | None = None, ctx: Context | None = None, param: Parameter | None = None, param_hint: cabc.Sequence[str] | str | None = None, param_type: str | None = None, ) -> None:
  L184: def format_message(self) -> str:
  L224: def __str__(self) -> str:
  L232: class NoSuchOption(UsageError):
  L241: def __init__( self, option_name: str, message: str | None = None, possibilities: cabc.Iterable[str] | None = None, ctx: Context | None = None, ) -> None:
  L262: def format_message(self) -> str:
  L268: class NoSuchCommand(UsageError):
  L277: def __init__( self, command_name: str, message: str | None = None, possibilities: cabc.Iterable[str] | None = None, ctx: Context | None = None, ) -> None:
  L298: def format_message(self) -> str:
  L304: class BadOptionUsage(UsageError):
  L316: def __init__( self, option_name: str, message: str, ctx: Context | None = None ) -> None:
  L323: class BadArgumentUsage(UsageError):
  L332: class NoArgsIsHelpError(UsageError):
  L335: def __init__(self, ctx: Context) -> None:
  L338: def show(self, file: t.IO[t.Any] | None = None) -> None:
  L342: class FileError(ClickException):
- src/click/parser.py
  L51: def _unpack_args( args: cabc.Sequence[str], nargs_spec: cabc.Sequence[int] ) -> tuple[cabc.Sequence[str | cabc.Sequence[str | T_UNSET] | T_UNSET], list[str]]:
  L111: def _split_opt(opt: str) -> tuple[str, str]:
  L120: def _normalize_opt(opt: str, ctx: Context | None) -> str:
  L127: class _Option:
  L128: def __init__( self, obj: CoreOption, opts: cabc.Sequence[str], dest: str | None, action: str | None = None, nargs: int = 1, const: t.Any | None = None, ):
  L166: def takes_value(self) -> bool:
  L169: def process(self, value: t.Any, state: _ParsingState) -> None:
  L185: class _Argument:
  L186: def __init__(self, obj: CoreArgument, dest: str | None, nargs: int = 1):
  L191: def process( self, value: str | cabc.Sequence[str | T_UNSET] | T_UNSET, state: _ParsingState, ) -> None:
  L216: class _ParsingState:
  L217: def __init__(self, rargs: list[str]) -> None:
  L224: class _OptionParser:
  L241: def __init__(self, ctx: Context | None = None) -> None: #: The :class:`~click.Context` for this parser. This might be #: `None` for some advanced use cases.
  L265: def add_option( self, obj: CoreOption, opts: cabc.Sequence[str], dest: str | None, action: str | None = None, nargs: int = 1, const: t.Any | None = None, ) -> None:
  L290: def add_argument(self, obj: CoreArgument, dest: str | None, nargs: int = 1) -> None:
  L298: def parse_args( self, args: list[str] ) -> tuple[dict[str, t.Any], list[str], list[CoreParameter]]:
  L316: def _process_args_for_args(self, state: _ParsingState) -> None:
  L327: def _process_args_for_options(self, state: _ParsingState) -> None:
  L363: def _match_long_opt( self, opt: str, explicit_value: str | None, state: _ParsingState ) -> None:
  L390: def _match_short_opt(self, arg: str, state: _ParsingState) -> None:
  L430: def _get_value_from_state( self, option_name: str, option: _Option, state: _ParsingState ) -> str | cabc.Sequence[str] | T_UNSET | T_FLAG_NEEDS_VALUE:
  L470: def _process_opts(self, arg: str, state: _ParsingState) -> None:
  L503: def __getattr__(name: str) -> object:
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
- src/click/types.py
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
    test_arguments.py
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
    …

## Code
- sym:7448 Argument (src/click/core.py:3702-3814)
  class Argument(Parameter):
- sym:7404 Parameter (src/click/core.py:2215-2883)
  class Parameter(ABC):
- sym:7351 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:7355 Command.get_help_option (src/click/core.py:1197-1229)
  def get_help_option(self, ctx: Context) -> Option | None:
- sym:7707 Choice.get_metavar (src/click/types.py:426-442)
  def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
- sym:7427 Option (src/click/core.py:2886-3699)
  class Option(Parameter):
- sym:7475 argument (src/click/decorators.py:324-349)
  def argument( *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:7476 option (src/click/decorators.py:352-377)
  def option( *param_decls: str, cls: type[Option] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:7481 help_option (src/click/decorators.py:603-627)
  def help_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
- sym:7347 Command.__init__ (src/click/core.py:1040-1082)
  def __init__( self, name: str | None, context_settings: cabc.MutableMapping[str, t.Any] | None = None, callback: t.Callable[..., t.Any] | None = None, params: list[Parameter] | None = None, help: str | None = None, epilog: str | None = None, short_help: str | None = None, options_metavar: str | None = "[OPTIONS]", add_help_option: bool = True, no_args_is_help: bool = False, hidden: bool = False, deprecated: bool | str = False, ) -> None: #: the name the command thinks it has. Upon registering a command #: on a :class:`Group` the group will default the command name #: with this information. You should instead use the #: :class:`Context`\'s :attr:`~Context.info_name` attribute.
- sym:7608 _resolve_incomplete (src/click/shell_completion.py:757-801)
  def _resolve_incomplete( ctx: Context, args: list[str], incomplete: str ) -> tuple[Command | Parameter, str]:
- sym:7604 _is_incomplete_argument (src/click/shell_completion.py:639-659)
  def _is_incomplete_argument(ctx: Context, param: Parameter) -> bool:
- sym:7136 ship_move (examples/naval/naval.py:32-34)
  def ship_move(ship, x, y, speed):
- sym:7346 Command (src/click/core.py:964-1659)
  class Command:
- sym:7495 MissingParameter (src/click/exceptions.py:159-229)
  class MissingParameter(BadParameter):
- sym:7497 MissingParameter.format_message (src/click/exceptions.py:184-222)
  def format_message(self) -> str:
- sym:7354 Command.get_help_option_names (src/click/core.py:1182-1195)
  def get_help_option_names(self, ctx: Context) -> list[str]:
- sym:7310 augment_usage_errors (src/click/core.py:129-144)
  def augment_usage_errors( ctx: Context, param: Parameter | None = None ) -> cabc.Generator[None]:
- sym:7311 iter_params_for_processing (src/click/core.py:147-171)
  def iter_params_for_processing( invocation_order: cabc.Sequence[Parameter], declaration_order: cabc.Sequence[Parameter], ) -> list[Parameter]:
- sym:7361 Command.format_options (src/click/core.py:1300-1310)
  def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:7362 Command.format_arguments (src/click/core.py:1312-1322)
  def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:7367 Command.shell_complete (src/click/core.py:1422-1467)
  def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
- sym:7378 Group.__init__ (src/click/core.py:1736-1789)
  def __init__( self, name: str | None = None, commands: cabc.MutableMapping[str, Command] | cabc.Sequence[Command] | None = None, invoke_without_command: bool = False, no_args_is_help: bool | None = None, subcommand_metavar: str | None = None, chain: bool = False, result_callback: t.Callable[..., t.Any] | None = None, **kwargs: t.Any, ) -> None:
- sym:7405 Parameter.__init__ (src/click/core.py:2327-2399)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, type: types.ParamType[t.Any] | t.Any | None = None, required: bool = False, # XXX The default historically embed two concepts: # - the declaration of a Parameter object carrying the default (handy to # arbitrage the default value of coupled Parameters sharing the same # self.name, like flag options), # - and the actual value of the default. # It is confusing and is the source of many issues discussed in: # https://github.com/pallets/click/pull/3030 # In the future, we might think of splitting it in two, not unlike # Option.is_flag and Option.flag_value: we could have something like # Parameter.is_default and Parameter.default_value. default: t.Any | t.Callable[[], t.Any] | None = UNSET, callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None = None, nargs: int | None = None, multiple: bool = False, metavar: str | None = None, expose_value: bool = True, is_eager: bool = False, envvar: str | cabc.Sequence[str] | None = None, shell_complete: t.Callable[ [Context, Parameter, str], list[CompletionItem] | list[str] ] | None = None, deprecated: bool | str = False, ) -> None:
- sym:7446 Option.consume_value (src/click/core.py:3628-3682)
  def consume_value( self, ctx: Context, opts: cabc.Mapping[str, Parameter] ) -> tuple[t.Any, ParameterSource]:
- sym:7459 FC (src/click/decorators.py:25-25)
  FC = t.TypeVar("FC", bound="_AnyCallable | Command")
- sym:7474 _param_memo (src/click/decorators.py:314-321)
  def _param_memo(f: t.Callable[..., t.Any], param: Parameter) -> None:
- sym:7477 confirmation_option (src/click/decorators.py:380-401)
  def confirmation_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
- sym:7478 password_option (src/click/decorators.py:404-418)
  def password_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
- sym:7479 version_option (src/click/decorators.py:421-558)
  def version_option( version: str | None = None, *param_decls: str, package_name: str | None = None, prog_name: str | None = None, message: str | None = None, **kwargs: t.Any, ) -> t.Callable[[FC], FC]:
- sym:7480 custom_version_option (src/click/decorators.py:561-600)
  def custom_version_option( callback: t.Callable[[Context], str], *param_decls: str, **kwargs: t.Any, ) -> t.Callable[[FC], FC]:
- sym:7491 UsageError.show (src/click/exceptions.py:87-111)
  def show(self, file: t.IO[t.Any] | None = None) -> None:
- sym:7493 BadParameter.__init__ (src/click/exceptions.py:135-144)
  def __init__( self, message: str, ctx: Context | None = None, param: Parameter | None = None, param_hint: cabc.Sequence[str] | str | None = None, ) -> None:
- sym:7496 MissingParameter.__init__ (src/click/exceptions.py:173-182)
  def __init__( self, message: str | None = None, ctx: Context | None = None, param: Parameter | None = None, param_hint: cabc.Sequence[str] | str | None = None, param_type: str | None = None, ) -> None:
- sym:7545 _Option.__init__ (src/click/parser.py:128-163)
  def __init__( self, obj: CoreOption, opts: cabc.Sequence[str], dest: str | None, action: str | None = None, nargs: int = 1, const: t.Any | None = None, ):
- sym:7555 _OptionParser.add_option (src/click/parser.py:265-288)
  def add_option( self, obj: CoreOption, opts: cabc.Sequence[str], dest: str | None, action: str | None = None, nargs: int = 1, const: t.Any | None = None, ) -> None:
- sym:7560 _OptionParser._match_long_opt (src/click/parser.py:363-388)
  def _match_long_opt( self, opt: str, explicit_value: str | None, state: _ParsingState ) -> None:
- sym:7561 _OptionParser._match_short_opt (src/click/parser.py:390-428)
  def _match_short_opt(self, arg: str, state: _ParsingState) -> None:
- sym:7562 _OptionParser._get_value_from_state (src/click/parser.py:430-468)
  def _get_value_from_state( self, option_name: str, option: _Option, state: _ParsingState ) -> str | cabc.Sequence[str] | T_UNSET | T_FLAG_NEEDS_VALUE:
- sym:7606 _is_incomplete_option (src/click/shell_completion.py:671-693)
  def _is_incomplete_option(ctx: Context, args: list[str], param: Parameter) -> bool:
- sym:7681 ParamType.__call__ (src/click/types.py:144-149)
  def __call__( self, value: None, param: Parameter | None = None, ctx: Context | None = None, ) -> None:
- sym:7684 ParamType.get_metavar (src/click/types.py:169-170)
  def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
- sym:7685 ParamType.get_missing_message (src/click/types.py:172-177)
  def get_missing_message(self, param: Parameter, ctx: Context | None) -> str | None:
- sym:7686 ParamType.convert (src/click/types.py:179-203)
  def convert( self, value: _InputT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:7688 ParamType.fail (src/click/types.py:215-222)
  def fail( self, message: str, param: Parameter | None = None, ctx: Context | None = None, ) -> t.NoReturn:
- sym:7689 ParamType.shell_complete (src/click/types.py:224-239)
  def shell_complete( self, ctx: Context, param: Parameter, incomplete: str ) -> list[CompletionItem]:
- sym:7695 FuncParamType.convert (src/click/types.py:275-289)
  def convert( self, value: _ValueT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:7697 UnprocessedParamType.convert (src/click/types.py:295-298)
  def convert( self, value: _ValueT, param: Parameter | None, ctx: Context | None ) -> _ValueT:
- sym:7700 StringParamType.convert (src/click/types.py:307-323)
  def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> str:
- sym:7708 Choice.get_missing_message (src/click/types.py:444-452)
  def get_missing_message(self, param: Parameter, ctx: Context | None) -> str:
- sym:7929 test_group_with_args (tests/test_commands.py:129-141)
  def test_group_with_args(runner, args, exit_code, expect):
- sym:8030 test_formatting_custom_type_metavar (tests/test_formatting.py:317-335)
  def test_formatting_custom_type_metavar(runner):
- sym:8107 test_required_option (tests/test_options.py:960-971)
  def test_required_option(value, expect_missing, processed_value):
- sym:7844 test_argument_help (tests/test_arguments.py:295-311)
  def test_argument_help(runner):
- sym:7862 test_nested_subcommand_help (tests/test_arguments.py:644-663)
  def test_nested_subcommand_help(runner):
- sym:7845 test_argument_help_options_only_no_arguments_section (tests/test_arguments.py:314-324)
  def test_argument_help_options_only_no_arguments_section(runner):
- sym:8027 test_formatting_usage_error_no_help (tests/test_formatting.py:221-233)
  def test_formatting_usage_error_no_help(runner):
- sym:8029 test_formatting_usage_error_help_hint (tests/test_formatting.py:285-314)
  def test_formatting_usage_error_help_hint( runner, help_names, extra_options, expected_hint ):
- sym:8028 test_formatting_usage_custom_help (tests/test_formatting.py:236-249)
  def test_formatting_usage_custom_help(runner):
- sym:8194 test_argument_default (tests/test_shell_completion.py:131-143)
  def test_argument_default():
- sym:8202 test_option_flag (tests/test_shell_completion.py:213-224)
  def test_option_flag():
- sym:7868 test_param_squatting_help_option_name (tests/test_basic.py:73-87)
  def test_param_squatting_help_option_name(runner):
- sym:7861 test_subcommand_help (tests/test_arguments.py:625-641)
  def test_subcommand_help(runner):
- sym:7981 test_no_state_leaks (tests/test_context.py:436-519)
  def test_no_state_leaks(runner):
- sym:8203 test_flag_option_with_nargs_option (tests/test_shell_completion.py:227-237)
  def test_flag_option_with_nargs_option():
- sym:8208 test_double_dash (tests/test_shell_completion.py:293-305)
  def test_double_dash():
- sym:8057 test_argument_to_info_dict_help (tests/test_info_dict.py:292-294)
  def test_argument_to_info_dict_help(help_in, help_out):
- sym:7846 test_argument_help_optional_metavar (tests/test_arguments.py:327-336)
  def test_argument_help_optional_metavar(runner):

---
tokens: 7147/8000
