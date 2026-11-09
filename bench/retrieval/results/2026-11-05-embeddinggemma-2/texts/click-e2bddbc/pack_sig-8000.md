## Task
# click-e2bddbc · click-e2bddbc · open
Goal: Deprecate a parameter name that is not a Python identifier

Click 9.0 refuses such a name; warn a release ahead so the break can diffuse.
Prepares the ground for #3827, as promised in pull/3827#issuecomment-5583963038.
Workspace: /root/bench/av_gemma/ws/rec728b-click-e2bddbc/click-e2bddbc
Changed (0): nothing yet

## Map
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
- sym:3348 __getattr__ (src/click/parser.py:503-533)
  def __getattr__(name: str) -> object:
- sym:2950 __getattr__ (src/click/__init__.py:76-144)
  def __getattr__(name: str) -> object:
- sym:3463 ParamType (src/click/types.py:65-239)
  class ParamType(t.Generic[_ValueT_co, _InputT_contra], abc.ABC):
- sym:3597 __getattr__ (src/click/utils.py:669-688)
  def __getattr__(name: str) -> object:
- sym:3136 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:3190 Parameter.__init__ (src/click/core.py:2334-2416)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, type: types.ParamType[t.Any] | t.Any | None = None, required: bool = False, # XXX The default historically embed two concepts: # - the declaration of a Parameter object carrying the default (handy to # arbitrage the default value of coupled Parameters sharing the same # self.name, like flag options), # - and the actual value of the default. # It is confusing and is the source of many issues discussed in: # https://github.com/pallets/click/pull/3030 # In the future, we might think of splitting it in two, not unlike # Option.is_flag and Option.flag_value: we could have something like # Parameter.is_default and Parameter.default_value. default: t.Any | t.Callable[[], t.Any] | None = UNSET, callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None = None, nargs: int | None = None, multiple: bool = False, metavar: str | None = None, expose_value: bool = True, is_eager: bool = False, envvar: str | cabc.Sequence[str] | None = None, shell_complete: t.Callable[ [Context, Parameter, str], list[CompletionItem] | list[str] ] | None = None, deprecated: bool | str = False, help: str | None = None, ) -> None:
- sym:3055 T_UNSET (src/click/_utils.py:41-41)
  T_UNSET: t.TypeAlias = t.Literal[Sentinel.UNSET]
- sym:3207 Parameter.handle_parse_result (src/click/core.py:2776-2862)
  def handle_parse_result( self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str] ) -> tuple[t.Any, list[str]]:
- sym:3559 INT (src/click/types.py:1419-1419)
  INT: t.Final[IntParamType] = IntParamType()
- sym:3237 Argument._parse_decls (src/click/core.py:3777-3794)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:3053 UNSET (src/click/_utils.py:31-31)
  UNSET: t.Literal[Sentinel.UNSET] = Sentinel.UNSET
- sym:3242 __getattr__ (src/click/core.py:3814-3835)
  def __getattr__(name: str) -> object:
- sym:3233 Argument (src/click/core.py:3715-3811)
  class Argument(Parameter):
- sym:3031 _tempfilepager (src/click/_termui_impl.py:636-667)
  def _tempfilepager( cmd_path: Path, cmd_params: list[str], color: bool | None = None ) -> t.Iterator[tuple[t.TextIO, bool | None]]:
- sym:3562 UUID (src/click/types.py:1430-1430)
  UUID: t.Final[UUIDParameterType] = UUIDParameterType()
- sym:3390 _is_incomplete_option (src/click/shell_completion.py:671-693)
  def _is_incomplete_option(ctx: Context, args: list[str], param: Parameter) -> bool:
- sym:3558 STRING (src/click/types.py:1415-1415)
  STRING: t.Final[StringParamType] = StringParamType()
- sym:3043 TextWrapper (src/click/_textwrap.py:38-188)
  class TextWrapper(textwrap.TextWrapper):
- sym:3557 UNPROCESSED (src/click/types.py:1411-1411)
  UNPROCESSED: t.Final[UnprocessedParamType] = UnprocessedParamType()
- sym:3097 ParameterSource (src/click/core.py:174-210)
  class ParameterSource(enum.IntEnum):
- sym:3564 _posixify (src/click/utils.py:32-33)
  def _posixify(name: str) -> str:
- sym:3595 _detect_program_name (src/click/utils.py:562-614)
  def _detect_program_name( path: str | None = None, _main: ModuleType | None = None ) -> str:
- sym:3045 TextWrapper._wrap_chunks (src/click/_textwrap.py:66-162)
  def _wrap_chunks(self, chunks: list[str]) -> list[str]:
- sym:3301 FORCED_WIDTH (src/click/formatting.py:11-11)
  FORCED_WIDTH: int | None = None
- sym:3229 Option.resolve_envvar_value (src/click/core.py:3569-3591)
  def resolve_envvar_value(self, ctx: Context) -> str | None:
- sym:3236 Argument.make_metavar (src/click/core.py:3758-3775)
  def make_metavar(self, ctx: Context) -> str:
- sym:3283 NoSuchOption (src/click/exceptions.py:232-265)
  class NoSuchOption(UsageError):
- sym:3560 FLOAT (src/click/types.py:1423-1423)
  FLOAT: t.Final[FloatParamType] = FloatParamType()
- sym:3286 NoSuchCommand (src/click/exceptions.py:268-301)
  class NoSuchCommand(UsageError):
- sym:3056 T_FLAG_NEEDS_VALUE (src/click/_utils.py:44-44)
  T_FLAG_NEEDS_VALUE: t.TypeAlias = t.Literal[Sentinel.FLAG_NEEDS_VALUE]
- sym:3263 version_option (src/click/decorators.py:421-558)
  def version_option( version: str | None = None, *param_decls: str, package_name: str | None = None, prog_name: str | None = None, message: str | None = None, **kwargs: t.Any, ) -> t.Callable[[FC], FC]:
- sym:3189 Parameter (src/click/core.py:2215-2901)
  class Parameter(ABC):
- sym:3284 NoSuchOption.__init__ (src/click/exceptions.py:241-260)
  def __init__( self, option_name: str, message: str | None = None, possibilities: cabc.Iterable[str] | None = None, ctx: Context | None = None, ) -> None:
- sym:3287 NoSuchCommand.__init__ (src/click/exceptions.py:277-296)
  def __init__( self, command_name: str, message: str | None = None, possibilities: cabc.Iterable[str] | None = None, ctx: Context | None = None, ) -> None:
- sym:3529 UUIDParameterType.__repr__ (src/click/types.py:906-907)
  def __repr__(self) -> str:
- sym:3213 Option.__init__ (src/click/core.py:2996-3072)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, show_default: bool | str | None = None, prompt: bool | str = False, confirmation_prompt: bool | str = False, prompt_required: bool = True, hide_input: bool = False, is_flag: bool | None = None, flag_value: t.Any = UNSET, multiple: bool = False, count: bool = False, allow_from_autoenv: bool = True, type: types.ParamType[t.Any] | t.Any | None = None, help: str | None = None, hidden: bool = False, show_choices: bool = True, show_envvar: bool = False, deprecated: bool | str = False, **attrs: t.Any, ) -> None:
- sym:3578 _KeepOpenFile (src/click/utils.py:211-249)
  class _KeepOpenFile:
- sym:3276 BadParameter (src/click/exceptions.py:114-156)
  class BadParameter(UsageError):
- sym:3264 custom_version_option (src/click/decorators.py:561-600)
  def custom_version_option( callback: t.Callable[[Context], str], *param_decls: str, **kwargs: t.Any, ) -> t.Callable[[FC], FC]:
- sym:3461 CliRunner.isolated_filesystem (src/click/testing.py:742-798)
  def isolated_filesystem( self, temp_dir: str | os.PathLike[str] | None = None ) -> cabc.Generator[str]:
- sym:3026 _resolve_pager_command (src/click/_termui_impl.py:423-448)
  def _resolve_pager_command(cmd_parts: list[str]) -> tuple[Path, list[str]] | None:
- sym:3561 BOOL (src/click/types.py:1427-1427)
  BOOL: t.Final[BoolParamType] = BoolParamType()
- sym:3140 Command.get_help_option (src/click/core.py:1197-1229)
  def get_help_option(self, ctx: Context) -> Option | None:
- sym:3223 Option._parse_decls (src/click/core.py:3276-3335)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:3048 Sentinel (src/click/_utils.py:7-28)
  class Sentinel(enum.Enum):
- sym:3088 _HELP_OPTION_STORAGE_NAME (src/click/core.py:60-60)
  _HELP_OPTION_STORAGE_NAME = "_click_default_help"
- sym:3129 Context.set_parameter_source (src/click/core.py:936-943)
  def set_parameter_source(self, name: str, source: ParameterSource) -> None:
- sym:3398 _format_default (src/click/termui.py:128-135)
  def _format_default(default: V) -> V | str:
- sym:3546 Tuple (src/click/types.py:1258-1308)
  class Tuple(CompositeParamType[tuple[t.Any, ...]]):
- sym:3470 ParamType.convert (src/click/types.py:179-203)
  def convert( self, value: _InputT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:2994 _is_jupyter_kernel_output (src/click/_compat.py:495-499)
  def _is_jupyter_kernel_output(stream: t.IO[t.Any]) -> bool:
- sym:2960 _FixupStream (src/click/_compat.py:85-151)
  class _FixupStream:
- sym:2987 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:3077 ConsoleStream.name (src/click/_winconsole.py:196-197)
  def name(self) -> str:
- sym:3161 _BaseCommand (src/click/core.py:1670-1674)
  class _BaseCommand(Command, metaclass=_FakeSubclassCheck):
- sym:3182 _MultiCommand (src/click/core.py:2140-2144)
  class _MultiCommand(Group, metaclass=_FakeSubclassCheck):
- sym:3337 _OptionParser (src/click/parser.py:224-500)
  class _OptionParser:
- sym:3387 split_arg_string (src/click/shell_completion.py:603-636)
  def split_arg_string(string: str) -> list[str]:
- sym:3446 _NamedTextIOWrapper.name (src/click/testing.py:203-204)
  def name(self) -> str:
- sym:3549 Tuple.name (src/click/types.py:1282-1283)
  def name(self) -> str: # type: ignore[override]
- sym:3586 _get_binary_stream (src/click/utils.py:349-363)
  def _get_binary_stream(name: t.Literal["stdin", "stdout", "stderr"]) -> t.BinaryIO:
- sym:3587 _get_text_stream (src/click/utils.py:366-390)
  def _get_text_stream( name: t.Literal["stdin", "stdout", "stderr"], encoding: str | None = None, errors: str | None = "strict", ) -> t.TextIO:
- sym:4209 test_expand_args (tests/test_utils/test__expand_args.py:6-17)
  def test_expand_args(monkeypatch):
- sym:4188 test_param_type_input_parameter_defaults_at_runtime (tests/test_types/test_ParamType.py:6-12)
  def test_param_type_input_parameter_defaults_at_runtime():
- sym:4121 _CustomTypeNoValue (tests/test_termui.py:1731-1737)
  class _CustomTypeNoValue(click.ParamType):
- sym:3833 NAME_ARGUMENT (tests/test_info_dict.py:30-45)
  NAME_ARGUMENT = (
- sym:3794 _Marker (tests/test_defaults.py:470-473)
  class _Marker:
- sym:3841 PARAM_KINDS (tests/test_info_dict.py:279-282)
  PARAM_KINDS = [
- sym:4252 push (tests/typing/typing_aliased_group.py:40-41)
  def push() -> None:
- sym:4255 hello (tests/typing/typing_group_kw_options.py:7-8)
  def hello() -> None:

---
tokens: 3761/8000
