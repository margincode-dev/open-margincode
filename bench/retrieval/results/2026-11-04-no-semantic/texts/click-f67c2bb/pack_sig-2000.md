## Task
# click-f67c2bb · click-f67c2bb-2 · open
Goal: Fix short help abbreviation
Workspace: /root/bench/ws/rebf815-click-f67c2bb/click-f67c2bb-2
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
- sym:4391 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:4481 ConsoleStream.name (src/click/_winconsole.py:196-197)
  def name(self) -> str:
- sym:4496 _format_deprecated_label (src/click/core.py:107-112)
  def _format_deprecated_label(deprecated: bool | str) -> str:
- sym:4502 Context (src/click/core.py:213-961)
  class Context:
- sym:5631 test_make_default_short_help (tests/test_utils/test_make_default_short_help.py:32-39)
  def test_make_default_short_help(value, max_length, alter, expect):
- sym:5114 test_auto_shorthelp (tests/test_commands.py:66-95)
  def test_auto_shorthelp(runner):
- sym:5239 HELLO_COMMAND (tests/test_info_dict.py:67-78)
  HELLO_COMMAND = (
- sym:5240 HELLO_GROUP (tests/test_info_dict.py:79-92)
  HELLO_GROUP = (

---
tokens: 1187/2000
