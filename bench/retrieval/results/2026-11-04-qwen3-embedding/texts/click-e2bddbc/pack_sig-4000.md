## Task
# click-e2bddbc · click-e2bddbc · open
Goal: Deprecate a parameter name that is not a Python identifier

Click 9.0 refuses such a name; warn a release ahead so the break can diffuse.
Prepares the ground for #3827, as promised in pull/3827#issuecomment-5583963038.
Workspace: /root/bench/ws/rebf815-click-e2bddbc/click-e2bddbc
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
- sym:3207 Parameter.handle_parse_result (src/click/core.py:2776-2862)
  def handle_parse_result( self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str] ) -> tuple[t.Any, list[str]]:
- sym:2950 __getattr__ (src/click/__init__.py:76-144)
  def __getattr__(name: str) -> object:
- sym:3136 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
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
- sym:3129 Context.set_parameter_source (src/click/core.py:936-943)
  def set_parameter_source(self, name: str, source: ParameterSource) -> None:
- sym:3130 Context.get_parameter_source (src/click/core.py:945-961)
  def get_parameter_source(self, name: str) -> ParameterSource | None:
- sym:3636 test_deprecated_warning (tests/test_arguments.py:421-439)
  def test_deprecated_warning(runner, deprecated):
- sym:3631 test_deprecated_usage (tests/test_arguments.py:339-347)
  def test_deprecated_usage(runner):
- sym:4209 test_expand_args (tests/test_utils/test__expand_args.py:6-17)
  def test_expand_args(monkeypatch):
- sym:3855 test_deprecated_warning (tests/test_options.py:100-118)
  def test_deprecated_warning(runner, deprecated):
- sym:3637 test_deprecated_required (tests/test_arguments.py:442-444)
  def test_deprecated_required(runner):
- sym:3852 test_deprecated_usage (tests/test_options.py:48-58)
  def test_deprecated_usage(runner, deprecated):
- sym:3727 test_deprecated_in_invocation (tests/test_commands.py:588-597)
  def test_deprecated_in_invocation(runner, deprecated):
- sym:3857 test_deprecated_prompt (tests/test_options.py:126-128)
  def test_deprecated_prompt(runner):
- sym:3800 test_context_protected_args_deprecated (tests/test_deprecations.py:63-67)
  def test_context_protected_args_deprecated():
- sym:3634 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:393-403)
  def test_deprecated_usage_help_record_without_help(runner):
- sym:4188 test_param_type_input_parameter_defaults_at_runtime (tests/test_types/test_ParamType.py:6-12)
  def test_param_type_input_parameter_defaults_at_runtime():
- sym:3652 test_param_squatting_help_option_name (tests/test_basic.py:73-87)
  def test_param_squatting_help_option_name(runner):
- sym:3856 test_deprecated_required (tests/test_options.py:121-123)
  def test_deprecated_required(runner):
- sym:3633 test_deprecated_usage_help_record (tests/test_arguments.py:380-390)
  def test_deprecated_usage_help_record(runner, deprecated, expected_label):
- sym:3962 test_argument_sharing_param_name_warning (tests/test_options.py:3431-3447)
  def test_argument_sharing_param_name_warning(runner):
- sym:3648 test_duplicate_names_warning (tests/test_arguments.py:695-703)
  def test_duplicate_names_warning(runner, args_one, args_two):

---
tokens: 1994/4000
