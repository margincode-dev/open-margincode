## Task
# click-f67c2bb · click-f67c2bb · open
Goal: Fix short help abbreviation
Workspace: /root/bench/ws/rebf815-click-f67c2bb/click-f67c2bb
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
- sym:4547 Command.get_short_help_str (src/click/core.py:1247-1261)
  def get_short_help_str(self, limit: int = 45) -> str:
- sym:4537 Command.to_info_dict (src/click/core.py:1084-1093)
  def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
- sym:4971 _make_default_short_help (src/click/utils.py:62-112)
  def _make_default_short_help(help: str, max_length: int = 45) -> str:
- sym:4536 Command.__init__ (src/click/core.py:1040-1082)
  def __init__( self, name: str | None, context_settings: cabc.MutableMapping[str, t.Any] | None = None, callback: t.Callable[..., t.Any] | None = None, params: list[Parameter] | None = None, help: str | None = None, epilog: str | None = None, short_help: str | None = None, options_metavar: str | None = "[OPTIONS]", add_help_option: bool = True, no_args_is_help: bool = False, hidden: bool = False, deprecated: bool | str = False, ) -> None: #: the name the command thinks it has. Upon registering a command #: on a :class:`Group` the group will default the command name #: with this information. You should instead use the #: :class:`Context`\'s :attr:`~Context.info_name` attribute.
- sym:4535 Command (src/click/core.py:964-1659)
  class Command:
- sym:4581 Group.format_commands (src/click/core.py:1984-2010)
  def format_commands(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:4585 Group.shell_complete (src/click/core.py:2120-2137)
  def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
- sym:5001 __getattr__ (src/click/utils.py:669-688)
  def __getattr__(name: str) -> object:
- sym:4556 Command.shell_complete (src/click/core.py:1422-1467)
  def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
- sym:4749 _OptionParser._match_short_opt (src/click/parser.py:390-428)
  def _match_short_opt(self, arg: str, state: _ParsingState) -> None:
- sym:4274 AliasedGroup.get_command (examples/aliases/aliases.py:42-68)
  def get_command(self, ctx, cmd_name): # Step one: built-in commands as normal
- sym:4733 _Option.__init__ (src/click/parser.py:128-163)
  def __init__( self, obj: CoreOption, opts: cabc.Sequence[str], dest: str | None, action: str | None = None, nargs: int = 1, const: t.Any | None = None, ):
- sym:4273 AliasedGroup (examples/aliases/aliases.py:37-73)
  class AliasedGroup(click.Group):
- sym:4751 _OptionParser._process_opts (src/click/parser.py:470-500)
  def _process_opts(self, arg: str, state: _ParsingState) -> None:
- sym:4743 _OptionParser.add_option (src/click/parser.py:265-288)
  def add_option( self, obj: CoreOption, opts: cabc.Sequence[str], dest: str | None, action: str | None = None, nargs: int = 1, const: t.Any | None = None, ) -> None:
- sym:4732 _Option (src/click/parser.py:127-182)
  class _Option:
- sym:4742 _OptionParser.__init__ (src/click/parser.py:241-263)
  def __init__(self, ctx: Context | None = None) -> None: #: The :class:`~click.Context` for this parser. This might be #: `None` for some advanced use cases.
- sym:4433 _less_uses_raw_mode (src/click/_termui_impl.py:520-545)
  def _less_uses_raw_mode(less_env: str, cmd_params: list[str]) -> bool:
- sym:4630 Option.get_help_record (src/click/core.py:3410-3433)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:4492 _HELP_OPTION_STORAGE_NAME (src/click/core.py:60-60)
  _HELP_OPTION_STORAGE_NAME = "_click_default_help"
- sym:4544 Command.get_help_option (src/click/core.py:1197-1229)
  def get_help_option(self, ctx: Context) -> Option | None:
- sym:4391 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:4481 ConsoleStream.name (src/click/_winconsole.py:196-197)
  def name(self) -> str:
- sym:4496 _format_deprecated_label (src/click/core.py:107-112)
  def _format_deprecated_label(deprecated: bool | str) -> str:
- sym:4502 Context (src/click/core.py:213-961)
  class Context:
- sym:4540 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:4850 _NamedTextIOWrapper.name (src/click/testing.py:203-204)
  def name(self) -> str:
- sym:4953 Tuple.name (src/click/types.py:1282-1283)
  def name(self) -> str: # type: ignore[override]
- sym:5631 test_make_default_short_help (tests/test_utils/test_make_default_short_help.py:32-39)
  def test_make_default_short_help(value, max_length, alter, expect):
- sym:5114 test_auto_shorthelp (tests/test_commands.py:66-95)
  def test_auto_shorthelp(runner):
- sym:5239 HELLO_COMMAND (tests/test_info_dict.py:67-78)
  HELLO_COMMAND = (
- sym:5240 HELLO_GROUP (tests/test_info_dict.py:79-92)
  HELLO_GROUP = (
- sym:5221 test_removing_multiline_marker (tests/test_formatting.py:387-403)
  def test_removing_multiline_marker(runner):
- sym:5530 _PasswordLengthType.convert (tests/test_termui.py:1757-1760)
  def convert(self, value, param, ctx):
- sym:5529 _PasswordLengthType (tests/test_termui.py:1749-1760)
  class _PasswordLengthType(click.ParamType):
- sym:5246 test_param_to_info_dict_help (tests/test_info_dict.py:300-302)
  def test_param_to_info_dict_help(make_param, help_in, help_out):
- sym:5247 test_param_to_info_dict_deprecated_help (tests/test_info_dict.py:321-324)
  def test_param_to_info_dict_deprecated_help(make_param, help_in, deprecated, help_out):
- sym:5055 test_param_named_help (tests/test_basic.py:46-70)
  def test_param_named_help(runner, help_names, params, args, expected_output):
- sym:5126 test_get_help_option_names (tests/test_commands.py:462-469)
  def test_get_help_option_names(help_option_names, params, expected):
- sym:5241 test_parameter (tests/test_info_dict.py:212-214)
  def test_parameter(obj, expect):
- sym:5242 test_command (tests/test_info_dict.py:253-256)
  def test_command(obj, expect):
- sym:5243 test_context (tests/test_info_dict.py:259-269)
  def test_context():
- sym:5244 test_paramtype_no_name (tests/test_info_dict.py:272-276)
  def test_paramtype_no_name():
- sym:5248 test_argument_to_info_dict_nargs (tests/test_info_dict.py:327-331)
  def test_argument_to_info_dict_nargs():
- sym:5249 test_command_to_info_dict_multiple_arguments (tests/test_info_dict.py:334-345)
  def test_command_to_info_dict_multiple_arguments():

---
tokens: 2436/8000
