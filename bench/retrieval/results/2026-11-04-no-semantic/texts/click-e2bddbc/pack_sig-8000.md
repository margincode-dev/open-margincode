## Task
# click-e2bddbc · click-e2bddbc-2 · open
Goal: Deprecate a parameter name that is not a Python identifier

Click 9.0 refuses such a name; warn a release ahead so the break can diffuse.
Prepares the ground for #3827, as promised in pull/3827#issuecomment-5583963038.
Workspace: /root/bench/ws/rebf815-click-e2bddbc/click-e2bddbc-2
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
- sym:3136 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:3207 Parameter.handle_parse_result (src/click/core.py:2776-2862)
  def handle_parse_result( self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str] ) -> tuple[t.Any, list[str]]:
- sym:2950 __getattr__ (src/click/__init__.py:76-144)
  def __getattr__(name: str) -> object:
- sym:3242 __getattr__ (src/click/core.py:3814-3835)
  def __getattr__(name: str) -> object:
- sym:3348 __getattr__ (src/click/parser.py:503-533)
  def __getattr__(name: str) -> object:
- sym:3031 _tempfilepager (src/click/_termui_impl.py:636-667)
  def _tempfilepager( cmd_path: Path, cmd_params: list[str], color: bool | None = None ) -> t.Iterator[tuple[t.TextIO, bool | None]]:
- sym:3390 _is_incomplete_option (src/click/shell_completion.py:671-693)
  def _is_incomplete_option(ctx: Context, args: list[str], param: Parameter) -> bool:
- sym:3043 TextWrapper (src/click/_textwrap.py:38-188)
  class TextWrapper(textwrap.TextWrapper):
- sym:3097 ParameterSource (src/click/core.py:174-210)
  class ParameterSource(enum.IntEnum):
- sym:3595 _detect_program_name (src/click/utils.py:562-614)
  def _detect_program_name( path: str | None = None, _main: ModuleType | None = None ) -> str:
- sym:3045 TextWrapper._wrap_chunks (src/click/_textwrap.py:66-162)
  def _wrap_chunks(self, chunks: list[str]) -> list[str]:
- sym:3229 Option.resolve_envvar_value (src/click/core.py:3569-3591)
  def resolve_envvar_value(self, ctx: Context) -> str | None:
- sym:3283 NoSuchOption (src/click/exceptions.py:232-265)
  class NoSuchOption(UsageError):
- sym:3286 NoSuchCommand (src/click/exceptions.py:268-301)
  class NoSuchCommand(UsageError):
- sym:3263 version_option (src/click/decorators.py:421-558)
  def version_option( version: str | None = None, *param_decls: str, package_name: str | None = None, prog_name: str | None = None, message: str | None = None, **kwargs: t.Any, ) -> t.Callable[[FC], FC]:
- sym:3284 NoSuchOption.__init__ (src/click/exceptions.py:241-260)
  def __init__( self, option_name: str, message: str | None = None, possibilities: cabc.Iterable[str] | None = None, ctx: Context | None = None, ) -> None:
- sym:3287 NoSuchCommand.__init__ (src/click/exceptions.py:277-296)
  def __init__( self, command_name: str, message: str | None = None, possibilities: cabc.Iterable[str] | None = None, ctx: Context | None = None, ) -> None:
- sym:3597 __getattr__ (src/click/utils.py:669-688)
  def __getattr__(name: str) -> object:
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
- sym:3140 Command.get_help_option (src/click/core.py:1197-1229)
  def get_help_option(self, ctx: Context) -> Option | None:
- sym:3048 Sentinel (src/click/_utils.py:7-28)
  class Sentinel(enum.Enum):
- sym:3129 Context.set_parameter_source (src/click/core.py:936-943)
  def set_parameter_source(self, name: str, source: ParameterSource) -> None:
- sym:3546 Tuple (src/click/types.py:1258-1308)
  class Tuple(CompositeParamType[tuple[t.Any, ...]]):
- sym:3470 ParamType.convert (src/click/types.py:179-203)
  def convert( self, value: _InputT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:3463 ParamType (src/click/types.py:65-239)
  class ParamType(t.Generic[_ValueT_co, _InputT_contra], abc.ABC):
- sym:2960 _FixupStream (src/click/_compat.py:85-151)
  class _FixupStream:
- sym:2987 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:3053 UNSET (src/click/_utils.py:31-31)
  UNSET: t.Literal[Sentinel.UNSET] = Sentinel.UNSET
- sym:3077 ConsoleStream.name (src/click/_winconsole.py:196-197)
  def name(self) -> str:
- sym:3093 _format_deprecated_suffix (src/click/core.py:115-121)
  def _format_deprecated_suffix(deprecated: bool | str) -> str:
- sym:3095 augment_usage_errors (src/click/core.py:129-144)
  def augment_usage_errors( ctx: Context, param: Parameter | None = None ) -> cabc.Generator[None]:
- sym:3098 Context (src/click/core.py:213-961)
  class Context:
- sym:3111 Context.command_path (src/click/core.py:720-736)
  def command_path(self) -> str:
- sym:3130 Context.get_parameter_source (src/click/core.py:945-961)
  def get_parameter_source(self, name: str) -> ParameterSource | None:
- sym:3133 Command.to_info_dict (src/click/core.py:1084-1093)
  def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
- sym:3138 Command.collect_usage_pieces (src/click/core.py:1171-1180)
  def collect_usage_pieces(self, ctx: Context) -> list[str]:
- sym:3141 Command.make_parser (src/click/core.py:1231-1236)
  def make_parser(self, ctx: Context) -> _OptionParser:
- sym:3146 Command.format_options (src/click/core.py:1300-1310)
  def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:3147 Command.format_arguments (src/click/core.py:1312-1322)
  def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:3150 Command.parse_args (src/click/core.py:1370-1404)
  def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
- sym:3152 Command.shell_complete (src/click/core.py:1422-1467)
  def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
- sym:3189 Parameter (src/click/core.py:2215-2901)
  class Parameter(ABC):
- sym:4209 test_expand_args (tests/test_utils/test__expand_args.py:6-17)
  def test_expand_args(monkeypatch):
- sym:4188 test_param_type_input_parameter_defaults_at_runtime (tests/test_types/test_ParamType.py:6-12)
  def test_param_type_input_parameter_defaults_at_runtime():

---
tokens: 2549/8000
