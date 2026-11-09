## Task
# click-f25f697 · click-f25f697 · open
Goal: List every argument in the `Positional arguments` help section

Cloup already renders the section this way:
Workspace: /root/bench/av_tei/ws/rec09fc-click-f25f697/click-f25f697
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
- sym:8878 argument (src/click/decorators.py:324-349)
  def argument( *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:8960 _OptionParser.parse_args (src/click/parser.py:298-314)
  def parse_args( self, args: list[str] ) -> tuple[dict[str, t.Any], list[str], list[CoreParameter]]:
- sym:8959 _OptionParser.add_argument (src/click/parser.py:290-296)
  def add_argument(self, obj: CoreArgument, dest: str | None, nargs: int = 1) -> None:
- sym:8851 Argument (src/click/core.py:3702-3814)
  class Argument(Parameter):
- sym:8879 option (src/click/decorators.py:352-377)
  def option( *param_decls: str, cls: type[Option] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:8764 Command.format_options (src/click/core.py:1300-1310)
  def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8757 Command.get_help_option_names (src/click/core.py:1182-1195)
  def get_help_option_names(self, ctx: Context) -> list[str]:
- sym:8768 Command.parse_args (src/click/core.py:1370-1404)
  def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
- sym:8796 Group.parse_args (src/click/core.py:2012-2024)
  def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
- sym:8512 ComplexCLI.list_commands (examples/complex/complex/cli.py:32-38)
  def list_commands(self, ctx):
- sym:8780 Group (src/click/core.py:1677-2137)
  class Group(Command):
- sym:8862 FC (src/click/decorators.py:25-25)
  FC = t.TypeVar("FC", bound="_AnyCallable | Command")
- sym:8996 FishComplete.format_completion (src/click/shell_completion.py:504-518)
  def format_completion(self, item: CompletionItem[str]) -> str:
- sym:8805 CommandCollection.list_commands (src/click/core.py:2196-2202)
  def list_commands(self, ctx: Context) -> list[str]:
- sym:8802 CommandCollection.__init__ (src/click/core.py:2165-2173)
  def __init__( self, name: str | None = None, sources: list[Group] | None = None, **kwargs: t.Any, ) -> None:
- sym:8994 FishComplete (src/click/shell_completion.py:484-518)
  class FishComplete(ShellComplete):
- sym:8857 Argument.get_usage_pieces (src/click/core.py:3799-3800)
  def get_usage_pieces(self, ctx: Context) -> list[str]:
- sym:8793 Group.collect_usage_pieces (src/click/core.py:1975-1978)
  def collect_usage_pieces(self, ctx: Context) -> list[str]:
- sym:8951 _Argument (src/click/parser.py:185-213)
  class _Argument:
- sym:8795 Group.format_commands (src/click/core.py:1984-2010)
  def format_commands(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8911 NoArgsIsHelpError (src/click/exceptions.py:332-339)
  class NoArgsIsHelpError(UsageError):
- sym:8648 _pipepager (src/click/_termui_impl.py:549-632)
  def _pipepager( cmd_path: Path, cmd_params: list[str], color: bool | None = None ) -> t.Iterator[tuple[t.TextIO, bool | None]]:
- sym:8992 ZshComplete.get_completion_args (src/click/shell_completion.py:454-464)
  def get_completion_args(self) -> tuple[list[str], str]:
- sym:8856 Argument._parse_decls (src/click/core.py:3780-3797)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:8794 Group.format_options (src/click/core.py:1980-1982)
  def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8934 HelpFormatter.section (src/click/formatting.py:274-286)
  def section(self, name: str) -> cabc.Generator[None]:
- sym:8792 Group.list_commands (src/click/core.py:1971-1973)
  def list_commands(self, ctx: Context) -> list[str]:
- sym:8798 Group.resolve_command (src/click/core.py:2094-2118)
  def resolve_command( self, ctx: Context, args: list[str] ) -> tuple[str | None, Command | None, list[str]]:
- sym:8807 Parameter (src/click/core.py:2215-2883)
  class Parameter(ABC):
- sym:8855 Argument.make_metavar (src/click/core.py:3761-3778)
  def make_metavar(self, ctx: Context) -> str:
- sym:8989 BashComplete.get_completion_args (src/click/shell_completion.py:432-442)
  def get_completion_args(self) -> tuple[list[str], str]:
- sym:9248 test_argument_help_options_only_no_arguments_section (tests/test_arguments.py:314-324)
  def test_argument_help_options_only_no_arguments_section(runner):
- sym:9247 test_argument_help (tests/test_arguments.py:295-311)
  def test_argument_help(runner):
- sym:9253 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:393-403)
  def test_deprecated_usage_help_record_without_help(runner):
- sym:8716 Context (src/click/core.py:213-961)
  class Context:
- sym:8754 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:8762 Command.format_help (src/click/core.py:1263-1280)
  def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8826 Parameter.get_help_record (src/click/core.py:2846-2847)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:8844 Option.get_help_record (src/click/core.py:3397-3420)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:8858 Argument.get_help_record (src/click/core.py:3802-3806)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:8877 _param_memo (src/click/decorators.py:314-321)
  def _param_memo(f: t.Callable[..., t.Any], param: Parameter) -> None:
- sym:8924 HelpFormatter (src/click/formatting.py:110-299)
  class HelpFormatter:
- sym:8933 HelpFormatter.write_dl (src/click/formatting.py:229-271)
  def write_dl( self, rows: cabc.Iterable[tuple[str, str]], col_max: int = 30, col_spacing: int = 2, ) -> None:
- sym:9252 test_deprecated_usage_help_record (tests/test_arguments.py:380-390)
  def test_deprecated_usage_help_record(runner, deprecated, expected_label):
- sym:9596 test_argument_order (tests/test_shell_completion.py:114-128)
  def test_argument_order():
- sym:9295 test_datetime_argument_optional_metavar (tests/test_basic.py:678-688)
  def test_datetime_argument_optional_metavar(runner):
- sym:9249 test_argument_help_optional_metavar (tests/test_arguments.py:327-336)
  def test_argument_help_optional_metavar(runner):
- sym:9452 NAME_ARGUMENT (tests/test_info_dict.py:30-45)
  NAME_ARGUMENT = (
- sym:9462 test_command_to_info_dict_multiple_arguments (tests/test_info_dict.py:304-315)
  def test_command_to_info_dict_multiple_arguments():
- sym:9265 test_nested_subcommand_help (tests/test_arguments.py:644-663)
  def test_nested_subcommand_help(runner):
- sym:9254 test_deprecated_empty_help_no_leading_space (tests/test_arguments.py:411-417)
  def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
- sym:9597 test_argument_default (tests/test_shell_completion.py:131-143)
  def test_argument_default():
- sym:9264 test_subcommand_help (tests/test_arguments.py:625-641)
  def test_subcommand_help(runner):
- sym:9348 test_group_parse_args_collects_base_option_prefixes (tests/test_commands.py:613-632)
  def test_group_parse_args_collects_base_option_prefixes():
- sym:9251 test_argument_metavar_marks_optional (tests/test_arguments.py:363-373)
  def test_argument_metavar_marks_optional(runner, kwargs, expected):
- sym:9294 test_choice_argument_optional_metavar (tests/test_basic.py:651-675)
  def test_choice_argument_optional_metavar(runner):
- sym:9442 test_help_formatter_write_usage (tests/test_formatting.py:578-590)
  def test_help_formatter_write_usage( formatter_kwargs, current_indent, prog, args, prefix, expected ):
- sym:9250 test_deprecated_usage (tests/test_arguments.py:339-347)
  def test_deprecated_usage(runner):
- sym:9537 test_show_default_precedence (tests/test_options.py:1439-1444)
  def test_show_default_precedence(ctx_value, opt_value, extra_value, expect):
- sym:9266 test_when_argument_decorator_is_used_multiple_times_cls_is_preserved (tests/test_arguments.py:666-683)
  def test_when_argument_decorator_is_used_multiple_times_cls_is_preserved():
- sym:9231 test_nargs_star (tests/test_arguments.py:10-20)
  def test_nargs_star(runner):
- sym:9232 test_nargs_tup (tests/test_arguments.py:23-34)
  def test_nargs_tup(runner):
- sym:9233 test_nargs_tup_composite (tests/test_arguments.py:46-55)
  def test_nargs_tup_composite(runner, opts):
- sym:9234 test_nargs_mismatch_with_tuple_type (tests/test_arguments.py:58-64)
  def test_nargs_mismatch_with_tuple_type():
- sym:9235 test_nargs_err (tests/test_arguments.py:67-79)
  def test_nargs_err(runner):
- sym:9236 test_bytes_args (tests/test_arguments.py:82-102)
  def test_bytes_args(runner, monkeypatch):
- sym:9237 test_file_args (tests/test_arguments.py:105-124)
  def test_file_args(runner, tmp_path):

---
tokens: 3127/8000
