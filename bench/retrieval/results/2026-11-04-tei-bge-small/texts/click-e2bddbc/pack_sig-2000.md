## Task
# click-e2bddbc · click-e2bddbc · open
Goal: Deprecate a parameter name that is not a Python identifier

Click 9.0 refuses such a name; warn a release ahead so the break can diffuse.
Prepares the ground for #3827, as promised in pull/3827#issuecomment-5583963038.
Workspace: /root/bench/av_tei/ws/rec09fc-click-e2bddbc/click-e2bddbc
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
- sym:2950 __getattr__ (src/click/__init__.py:76-144)
  def __getattr__(name: str) -> object:
- sym:3242 __getattr__ (src/click/core.py:3814-3835)
  def __getattr__(name: str) -> object:
- sym:3136 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:3282 MissingParameter.__str__ (src/click/exceptions.py:224-229)
  def __str__(self) -> str:
- sym:3207 Parameter.handle_parse_result (src/click/core.py:2776-2862)
  def handle_parse_result( self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str] ) -> tuple[t.Any, list[str]]:
- sym:3597 __getattr__ (src/click/utils.py:669-688)
  def __getattr__(name: str) -> object:
- sym:3513 _NumberRangeBase.__repr__ (src/click/types.py:731-733)
  def __repr__(self) -> str:
- sym:3279 MissingParameter (src/click/exceptions.py:159-229)
  class MissingParameter(BadParameter):
- sym:3348 __getattr__ (src/click/parser.py:503-533)
  def __getattr__(name: str) -> object:
- sym:3031 _tempfilepager (src/click/_termui_impl.py:636-667)
  def _tempfilepager( cmd_path: Path, cmd_params: list[str], color: bool | None = None ) -> t.Iterator[tuple[t.TextIO, bool | None]]:
- sym:3506 _NumberParamTypeBase.convert (src/click/types.py:598-610)
  def convert( self, value: _ValueT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:3390 _is_incomplete_option (src/click/shell_completion.py:671-693)
  def _is_incomplete_option(ctx: Context, args: list[str], param: Parameter) -> bool:
- sym:2987 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:3077 ConsoleStream.name (src/click/_winconsole.py:196-197)
  def name(self) -> str:
- sym:3161 _BaseCommand (src/click/core.py:1670-1674)
  class _BaseCommand(Command, metaclass=_FakeSubclassCheck):
- sym:3182 _MultiCommand (src/click/core.py:2140-2144)
  class _MultiCommand(Group, metaclass=_FakeSubclassCheck):
- sym:4209 test_expand_args (tests/test_utils/test__expand_args.py:6-17)
  def test_expand_args(monkeypatch):
- sym:3652 test_param_squatting_help_option_name (tests/test_basic.py:73-87)
  def test_param_squatting_help_option_name(runner):

---
tokens: 1344/2000
