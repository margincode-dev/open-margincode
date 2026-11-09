## Task
# click-f25f697 · click-f25f697 · open
Goal: List every argument in the `Positional arguments` help section

Cloup already renders the section this way:
Workspace: /root/bench/ws/rebf815-click-f25f697/click-f25f697
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
- sym:8765 Command.format_arguments (src/click/core.py:1312-1322)
  def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8851 Argument (src/click/core.py:3702-3814)
  class Argument(Parameter):
- sym:8960 _OptionParser.parse_args (src/click/parser.py:298-314)
  def parse_args( self, args: list[str] ) -> tuple[dict[str, t.Any], list[str], list[CoreParameter]]:
- sym:8764 Command.format_options (src/click/core.py:1300-1310)
  def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8757 Command.get_help_option_names (src/click/core.py:1182-1195)
  def get_help_option_names(self, ctx: Context) -> list[str]:
- sym:8959 _OptionParser.add_argument (src/click/parser.py:290-296)
  def add_argument(self, obj: CoreArgument, dest: str | None, nargs: int = 1) -> None:
- sym:8795 Group.format_commands (src/click/core.py:1984-2010)
  def format_commands(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8878 argument (src/click/decorators.py:324-349)
  def argument( *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:8857 Argument.get_usage_pieces (src/click/core.py:3799-3800)
  def get_usage_pieces(self, ctx: Context) -> list[str]:
- sym:8879 option (src/click/decorators.py:352-377)
  def option( *param_decls: str, cls: type[Option] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:8843 Option.get_help_spec (src/click/core.py:3366-3395)
  def get_help_spec(self, ctx: Context) -> str:
- sym:8858 Argument.get_help_record (src/click/core.py:3802-3806)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:8756 Command.collect_usage_pieces (src/click/core.py:1171-1180)
  def collect_usage_pieces(self, ctx: Context) -> list[str]:
- sym:8780 Group (src/click/core.py:1677-2137)
  class Group(Command):
- sym:8996 FishComplete.format_completion (src/click/shell_completion.py:504-518)
  def format_completion(self, item: CompletionItem[str]) -> str:
- sym:8994 FishComplete (src/click/shell_completion.py:484-518)
  class FishComplete(ShellComplete):
- sym:8793 Group.collect_usage_pieces (src/click/core.py:1975-1978)
  def collect_usage_pieces(self, ctx: Context) -> list[str]:
- sym:8827 Parameter.get_usage_pieces (src/click/core.py:2849-2850)
  def get_usage_pieces(self, ctx: Context) -> list[str]:
- sym:8648 _pipepager (src/click/_termui_impl.py:549-632)
  def _pipepager( cmd_path: Path, cmd_params: list[str], color: bool | None = None ) -> t.Iterator[tuple[t.TextIO, bool | None]]:
- sym:8762 Command.format_help (src/click/core.py:1263-1280)
  def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8961 _OptionParser._process_args_for_args (src/click/parser.py:316-325)
  def _process_args_for_args(self, state: _ParsingState) -> None:
- sym:8856 Argument._parse_decls (src/click/core.py:3780-3797)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:8954 _ParsingState (src/click/parser.py:216-221)
  class _ParsingState:
- sym:8934 HelpFormatter.section (src/click/formatting.py:274-286)
  def section(self, name: str) -> cabc.Generator[None]:
- sym:8798 Group.resolve_command (src/click/core.py:2094-2118)
  def resolve_command( self, ctx: Context, args: list[str] ) -> tuple[str | None, Command | None, list[str]]:
- sym:8754 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:8807 Parameter (src/click/core.py:2215-2883)
  class Parameter(ABC):
- sym:8855 Argument.make_metavar (src/click/core.py:3761-3778)
  def make_metavar(self, ctx: Context) -> str:
- sym:8768 Command.parse_args (src/click/core.py:1370-1404)
  def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
- sym:8911 NoArgsIsHelpError (src/click/exceptions.py:332-339)
  class NoArgsIsHelpError(UsageError):
- sym:9248 test_argument_help_options_only_no_arguments_section (tests/test_arguments.py:314-324)
  def test_argument_help_options_only_no_arguments_section(runner):
- sym:9247 test_argument_help (tests/test_arguments.py:295-311)
  def test_argument_help(runner):
- sym:9252 test_deprecated_usage_help_record (tests/test_arguments.py:380-390)
  def test_deprecated_usage_help_record(runner, deprecated, expected_label):
- sym:9462 test_command_to_info_dict_multiple_arguments (tests/test_info_dict.py:304-315)
  def test_command_to_info_dict_multiple_arguments():
- sym:9249 test_argument_help_optional_metavar (tests/test_arguments.py:327-336)
  def test_argument_help_optional_metavar(runner):
- sym:9251 test_argument_metavar_marks_optional (tests/test_arguments.py:363-373)
  def test_argument_metavar_marks_optional(runner, kwargs, expected):
- sym:9264 test_subcommand_help (tests/test_arguments.py:625-641)
  def test_subcommand_help(runner):
- sym:8716 Context (src/click/core.py:213-961)
  class Context:
- sym:8767 Command.make_context (src/click/core.py:1333-1368)
  def make_context( self, info_name: str | None, args: list[str], parent: Context | None = None, **extra: t.Any, ) -> Context:
- sym:8826 Parameter.get_help_record (src/click/core.py:2846-2847)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:8844 Option.get_help_record (src/click/core.py:3397-3420)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:8892 UsageError (src/click/exceptions.py:68-111)
  class UsageError(ClickException):
- sym:8924 HelpFormatter (src/click/formatting.py:110-299)
  class HelpFormatter:
- sym:8933 HelpFormatter.write_dl (src/click/formatting.py:229-271)
  def write_dl( self, rows: cabc.Iterable[tuple[str, str]], col_max: int = 30, col_spacing: int = 2, ) -> None:
- sym:8962 _OptionParser._process_args_for_options (src/click/parser.py:327-361)
  def _process_args_for_options(self, state: _ParsingState) -> None:
- sym:9253 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:393-403)
  def test_deprecated_usage_help_record_without_help(runner):
- sym:9295 test_datetime_argument_optional_metavar (tests/test_basic.py:678-688)
  def test_datetime_argument_optional_metavar(runner):
- sym:9265 test_nested_subcommand_help (tests/test_arguments.py:644-663)
  def test_nested_subcommand_help(runner):
- sym:9452 NAME_ARGUMENT (tests/test_info_dict.py:30-45)
  NAME_ARGUMENT = (
- sym:9254 test_deprecated_empty_help_no_leading_space (tests/test_arguments.py:411-417)
  def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
- sym:9294 test_choice_argument_optional_metavar (tests/test_basic.py:651-675)
  def test_choice_argument_optional_metavar(runner):
- sym:9258 test_nargs_star_ordering (tests/test_arguments.py:463-473)
  def test_nargs_star_ordering(runner):
- sym:9259 test_nargs_specified_plus_star_ordering (tests/test_arguments.py:476-486)
  def test_nargs_specified_plus_star_ordering(runner):
- sym:9442 test_help_formatter_write_usage (tests/test_formatting.py:578-590)
  def test_help_formatter_write_usage( formatter_kwargs, current_indent, prog, args, prefix, expected ):
- sym:9432 test_formatting_usage_error_help_hint (tests/test_formatting.py:285-314)
  def test_formatting_usage_error_help_hint( runner, help_names, extra_options, expected_hint ):
- sym:9250 test_deprecated_usage (tests/test_arguments.py:339-347)
  def test_deprecated_usage(runner):
- sym:9431 test_formatting_usage_custom_help (tests/test_formatting.py:236-249)
  def test_formatting_usage_custom_help(runner):
- sym:9438 test_formatting_with_options_metavar_empty (tests/test_formatting.py:423-426)
  def test_formatting_with_options_metavar_empty(runner):
- sym:9333 test_custom_parser (tests/test_commands.py:144-207)
  def test_custom_parser(runner):
- sym:9347 test_command_parse_args_collects_option_prefixes (tests/test_commands.py:600-610)
  def test_command_parse_args_collects_option_prefixes():
- sym:9348 test_group_parse_args_collects_base_option_prefixes (tests/test_commands.py:613-632)
  def test_group_parse_args_collects_base_option_prefixes():

---
tokens: 2909/8000
