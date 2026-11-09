## Task
# click-405a548 · click-405a548 · open
Goal: Move common `help` argument from `Option` and `Argument` to `Parameter`
Workspace: /root/bench/av_gemma/ws/rec728b-click-405a548/click-405a548
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
- src/click/exceptions.py
- src/click/parser.py
- src/click/shell_completion.py
- src/click/types.py
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
- sym:7448 Argument (src/click/core.py:3702-3814)
  class Argument(Parameter):
- sym:7351 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:7355 Command.get_help_option (src/click/core.py:1197-1229)
  def get_help_option(self, ctx: Context) -> Option | None:
- sym:7404 Parameter (src/click/core.py:2215-2883)
  class Parameter(ABC):
- sym:7405 Parameter.__init__ (src/click/core.py:2327-2399)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, type: types.ParamType[t.Any] | t.Any | None = None, required: bool = False, # XXX The default historically embed two concepts: # - the declaration of a Parameter object carrying the default (handy to # arbitrage the default value of coupled Parameters sharing the same # self.name, like flag options), # - and the actual value of the default. # It is confusing and is the source of many issues discussed in: # https://github.com/pallets/click/pull/3030 # In the future, we might think of splitting it in two, not unlike # Option.is_flag and Option.flag_value: we could have something like # Parameter.is_default and Parameter.default_value. default: t.Any | t.Callable[[], t.Any] | None = UNSET, callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None = None, nargs: int | None = None, multiple: bool = False, metavar: str | None = None, expose_value: bool = True, is_eager: bool = False, envvar: str | cabc.Sequence[str] | None = None, shell_complete: t.Callable[ [Context, Parameter, str], list[CompletionItem] | list[str] ] | None = None, deprecated: bool | str = False, ) -> None:
- sym:7481 help_option (src/click/decorators.py:603-627)
  def help_option(*param_decls: str, **kwargs: t.Any) -> t.Callable[[FC], FC]:
- sym:7427 Option (src/click/core.py:2886-3699)
  class Option(Parameter):
- sym:7475 argument (src/click/decorators.py:324-349)
  def argument( *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:7476 option (src/click/decorators.py:352-377)
  def option( *param_decls: str, cls: type[Option] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:7136 ship_move (examples/naval/naval.py:32-34)
  def ship_move(ship, x, y, speed):
- sym:7441 Option.get_help_record (src/click/core.py:3397-3420)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:7303 _HELP_OPTION_STORAGE_NAME (src/click/core.py:60-60)
  _HELP_OPTION_STORAGE_NAME = "_click_default_help"
- sym:7346 Command (src/click/core.py:964-1659)
  class Command:
- sym:7774 STRING (src/click/types.py:1415-1415)
  STRING: t.Final[StringParamType] = StringParamType()
- sym:7442 Option.get_help_extra (src/click/core.py:3422-3504)
  def get_help_extra(self, ctx: Context) -> types.OptionHelpExtra:
- sym:7495 MissingParameter (src/click/exceptions.py:159-229)
  class MissingParameter(BadParameter):
- sym:7778 UUID (src/click/types.py:1430-1430)
  UUID: t.Final[UUIDParameterType] = UUIDParameterType()
- sym:7453 Argument._parse_decls (src/click/core.py:3780-3797)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:7497 MissingParameter.format_message (src/click/exceptions.py:184-222)
  def format_message(self) -> str:
- sym:7775 INT (src/click/types.py:1419-1419)
  INT: t.Final[IntParamType] = IntParamType()
- sym:7544 _Option (src/click/parser.py:127-182)
  class _Option:
- sym:7844 test_argument_help (tests/test_arguments.py:295-311)
  def test_argument_help(runner):
- sym:7310 augment_usage_errors (src/click/core.py:129-144)
  def augment_usage_errors( ctx: Context, param: Parameter | None = None ) -> cabc.Generator[None]:
- sym:7311 iter_params_for_processing (src/click/core.py:147-171)
  def iter_params_for_processing( invocation_order: cabc.Sequence[Parameter], declaration_order: cabc.Sequence[Parameter], ) -> list[Parameter]:
- sym:7347 Command.__init__ (src/click/core.py:1040-1082)
  def __init__( self, name: str | None, context_settings: cabc.MutableMapping[str, t.Any] | None = None, callback: t.Callable[..., t.Any] | None = None, params: list[Parameter] | None = None, help: str | None = None, epilog: str | None = None, short_help: str | None = None, options_metavar: str | None = "[OPTIONS]", add_help_option: bool = True, no_args_is_help: bool = False, hidden: bool = False, deprecated: bool | str = False, ) -> None: #: the name the command thinks it has. Upon registering a command #: on a :class:`Group` the group will default the command name #: with this information. You should instead use the #: :class:`Context`\'s :attr:`~Context.info_name` attribute.
- sym:7361 Command.format_options (src/click/core.py:1300-1310)
  def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:7362 Command.format_arguments (src/click/core.py:1312-1322)
  def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:7367 Command.shell_complete (src/click/core.py:1422-1467)
  def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
- sym:7378 Group.__init__ (src/click/core.py:1736-1789)
  def __init__( self, name: str | None = None, commands: cabc.MutableMapping[str, Command] | cabc.Sequence[Command] | None = None, invoke_without_command: bool = False, no_args_is_help: bool | None = None, subcommand_metavar: str | None = None, chain: bool = False, result_callback: t.Callable[..., t.Any] | None = None, **kwargs: t.Any, ) -> None:
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
- sym:7560 _OptionParser._match_long_opt (src/click/parser.py:363-388)
  def _match_long_opt( self, opt: str, explicit_value: str | None, state: _ParsingState ) -> None:
- sym:7561 _OptionParser._match_short_opt (src/click/parser.py:390-428)
- sym:7562 _OptionParser._get_value_from_state (src/click/parser.py:430-468)
- sym:7604 _is_incomplete_argument (src/click/shell_completion.py:639-659)
  def _is_incomplete_argument(ctx: Context, param: Parameter) -> bool:
- sym:7606 _is_incomplete_option (src/click/shell_completion.py:671-693)
- sym:7608 _resolve_incomplete (src/click/shell_completion.py:757-801)
- sym:7681 ParamType.__call__ (src/click/types.py:144-149)
- sym:7684 ParamType.get_metavar (src/click/types.py:169-170)
- sym:7685 ParamType.get_missing_message (src/click/types.py:172-177)
- sym:7686 ParamType.convert (src/click/types.py:179-203)
- sym:7688 ParamType.fail (src/click/types.py:215-222)
- sym:7689 ParamType.shell_complete (src/click/types.py:224-239)
- sym:7695 FuncParamType.convert (src/click/types.py:275-289)
- sym:7697 UnprocessedParamType.convert (src/click/types.py:295-298)
- sym:7700 StringParamType.convert (src/click/types.py:307-323)
- sym:7707 Choice.get_metavar (src/click/types.py:426-442)
- sym:7708 Choice.get_missing_message (src/click/types.py:444-452)
- sym:7929 test_group_with_args (tests/test_commands.py:129-141)
- sym:8048 HELP_OPTION (tests/test_info_dict.py:9-29)
- sym:8030 test_formatting_custom_type_metavar (tests/test_formatting.py:317-335)
- sym:8049 NAME_ARGUMENT (tests/test_info_dict.py:30-45)
- sym:8107 test_required_option (tests/test_options.py:960-971)
- sym:7862 test_nested_subcommand_help (tests/test_arguments.py:644-663)
- sym:7845 test_argument_help_options_only_no_arguments_section (tests/test_arguments.py:314-324)
- sym:8027 test_formatting_usage_error_no_help (tests/test_formatting.py:221-233)
- sym:8029 test_formatting_usage_error_help_hint (tests/test_formatting.py:285-314)

---
tokens: 3567/4000
