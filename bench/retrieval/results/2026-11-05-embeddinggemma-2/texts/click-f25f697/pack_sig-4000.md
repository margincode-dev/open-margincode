## Task
# click-f25f697 · click-f25f697 · open
Goal: List every argument in the `Positional arguments` help section

Cloup already renders the section this way:
Workspace: /root/bench/av_gemma/ws/rec728b-click-f25f697/click-f25f697
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
- sym:8851 Argument (src/click/core.py:3702-3814)
  class Argument(Parameter):
- sym:8765 Command.format_arguments (src/click/core.py:1312-1322)
  def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8754 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:8878 argument (src/click/decorators.py:324-349)
  def argument( *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:8856 Argument._parse_decls (src/click/core.py:3780-3797)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:8855 Argument.make_metavar (src/click/core.py:3761-3778)
  def make_metavar(self, ctx: Context) -> str:
- sym:8879 option (src/click/decorators.py:352-377)
  def option( *param_decls: str, cls: type[Option] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:8960 _OptionParser.parse_args (src/click/parser.py:298-314)
  def parse_args( self, args: list[str] ) -> tuple[dict[str, t.Any], list[str], list[CoreParameter]]:
- sym:8854 Argument.human_readable_name (src/click/core.py:3756-3759)
  def human_readable_name(self) -> str:
- sym:8959 _OptionParser.add_argument (src/click/parser.py:290-296)
  def add_argument(self, obj: CoreArgument, dest: str | None, nargs: int = 1) -> None:
- sym:8780 Group (src/click/core.py:1677-2137)
  class Group(Command):
- sym:8996 FishComplete.format_completion (src/click/shell_completion.py:504-518)
  def format_completion(self, item: CompletionItem[str]) -> str:
- sym:8757 Command.get_help_option_names (src/click/core.py:1182-1195)
  def get_help_option_names(self, ctx: Context) -> list[str]:
- sym:9183 _posixify (src/click/utils.py:32-33)
  def _posixify(name: str) -> str:
- sym:8844 Option.get_help_record (src/click/core.py:3397-3420)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:8994 FishComplete (src/click/shell_completion.py:484-518)
  class FishComplete(ShellComplete):
- sym:9248 test_argument_help_options_only_no_arguments_section (tests/test_arguments.py:314-324)
  def test_argument_help_options_only_no_arguments_section(runner):
- sym:9247 test_argument_help (tests/test_arguments.py:295-311)
  def test_argument_help(runner):
- sym:9252 test_deprecated_usage_help_record (tests/test_arguments.py:380-390)
  def test_deprecated_usage_help_record(runner, deprecated, expected_label):
- sym:9253 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:393-403)
  def test_deprecated_usage_help_record_without_help(runner):
- sym:8716 Context (src/click/core.py:213-961)
  class Context:
- sym:8762 Command.format_help (src/click/core.py:1263-1280)
  def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8826 Parameter.get_help_record (src/click/core.py:2846-2847)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:8858 Argument.get_help_record (src/click/core.py:3802-3806)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:8862 FC (src/click/decorators.py:25-25)
  FC = t.TypeVar("FC", bound="_AnyCallable | Command")
- sym:8877 _param_memo (src/click/decorators.py:314-321)
  def _param_memo(f: t.Callable[..., t.Any], param: Parameter) -> None:
- sym:8924 HelpFormatter (src/click/formatting.py:110-299)
  class HelpFormatter:
- sym:8933 HelpFormatter.write_dl (src/click/formatting.py:229-271)
  def write_dl( self, rows: cabc.Iterable[tuple[str, str]], col_max: int = 30, col_spacing: int = 2, ) -> None:
- sym:9295 test_datetime_argument_optional_metavar (tests/test_basic.py:678-688)
  def test_datetime_argument_optional_metavar(runner):
- sym:9452 NAME_ARGUMENT (tests/test_info_dict.py:30-45)
  NAME_ARGUMENT = (
- sym:9424 test_wrapping_long_command_name (tests/test_formatting.py:90-122)
  def test_wrapping_long_command_name(runner):
- sym:9431 test_formatting_usage_custom_help (tests/test_formatting.py:236-249)
  def test_formatting_usage_custom_help(runner):
- sym:9462 test_command_to_info_dict_multiple_arguments (tests/test_info_dict.py:304-315)
  def test_command_to_info_dict_multiple_arguments():
- sym:9249 test_argument_help_optional_metavar (tests/test_arguments.py:327-336)
  def test_argument_help_optional_metavar(runner):
- sym:9265 test_nested_subcommand_help (tests/test_arguments.py:644-663)
  def test_nested_subcommand_help(runner):

---
tokens: 1927/4000
