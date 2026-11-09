## Task
# click-b90faad · click-b90faad · open
Goal: Deprecate a parameter name that Click 9.0 will refuse or lower case

Prepares the ground for #3827
Workspace: /root/bench/ws/rebf815-click-b90faad/click-b90faad
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
- sym:1786 Parameter.human_readable_name (src/click/core.py:2462-2466)
  def human_readable_name(self) -> str:
- sym:1727 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:2087 Choice.shell_complete (src/click/types.py:495-515)
  def shell_complete( self, ctx: Context, param: Parameter, incomplete: str ) -> list[CompletionItem]:
- sym:1798 Parameter.handle_parse_result (src/click/core.py:2777-2863)
  def handle_parse_result( self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str] ) -> tuple[t.Any, list[str]]:
- sym:2114 BoolParamType (src/click/types.py:819-885)
  class BoolParamType(ParamType[bool]):
- sym:1828 Argument._parse_decls (src/click/core.py:3778-3795)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:2155 _posixify (src/click/utils.py:32-33)
  def _posixify(name: str) -> str:
- sym:1723 Command.__init__ (src/click/core.py:1040-1082)
  def __init__( self, name: str | None, context_settings: cabc.MutableMapping[str, t.Any] | None = None, callback: t.Callable[..., t.Any] | None = None, params: list[Parameter] | None = None, help: str | None = None, epilog: str | None = None, short_help: str | None = None, options_metavar: str | None = "[OPTIONS]", add_help_option: bool = True, no_args_is_help: bool = False, hidden: bool = False, deprecated: bool | str = False, ) -> None: #: the name the command thinks it has. Upon registering a command #: on a :class:`Group` the group will default the command name #: with this information. You should instead use the #: :class:`Context`\'s :attr:`~Context.info_name` attribute.
- sym:1461 AliasedGroup.get_command (examples/aliases/aliases.py:42-68)
  def get_command(self, ctx, cmd_name): # Step one: built-in commands as normal
- sym:1460 AliasedGroup (examples/aliases/aliases.py:37-73)
  class AliasedGroup(click.Group):
- sym:1541 __getattr__ (src/click/__init__.py:76-144)
  def __getattr__(name: str) -> object:
- sym:1500 convert_rotation (examples/imagepipe/imagepipe.py:165-175)
  def convert_rotation(ctx, param, value):
- sym:1501 convert_flip (examples/imagepipe/imagepipe.py:178-186)
  def convert_flip(ctx, param, value):
- sym:1781 Parameter.__init__ (src/click/core.py:2335-2417)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, type: types.ParamType[t.Any] | t.Any | None = None, required: bool = False, # XXX The default historically embed two concepts: # - the declaration of a Parameter object carrying the default (handy to # arbitrage the default value of coupled Parameters sharing the same # self.name, like flag options), # - and the actual value of the default. # It is confusing and is the source of many issues discussed in: # https://github.com/pallets/click/pull/3030 # In the future, we might think of splitting it in two, not unlike # Option.is_flag and Option.flag_value: we could have something like # Parameter.is_default and Parameter.default_value. default: t.Any | t.Callable[[], t.Any] | None = UNSET, callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None = None, nargs: int | None = None, multiple: bool = False, metavar: str | None = None, expose_value: bool = True, is_eager: bool = False, envvar: str | cabc.Sequence[str] | None = None, shell_complete: t.Callable[ [Context, Parameter, str], list[CompletionItem] | list[str] ] | None = None, deprecated: bool | str = False, help: str | None = None, ) -> None:
- sym:1814 Option._parse_decls (src/click/core.py:3277-3336)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:2159 _LazyFile (src/click/utils.py:103-196)
  class _LazyFile:
- sym:1795 Parameter.process_value (src/click/core.py:2650-2714)
  def process_value(self, ctx: Context, value: t.Any) -> t.Any:
- sym:1691 Context.protected_args (src/click/core.py:522-531)
  def protected_args(self) -> list[str]:
- sym:1720 Context.set_parameter_source (src/click/core.py:936-943)
  def set_parameter_source(self, name: str, source: ParameterSource) -> None:
- sym:1683 _format_deprecated_label (src/click/core.py:107-112)
  def _format_deprecated_label(deprecated: bool | str) -> str:
- sym:2077 Choice (src/click/types.py:342-515)
  class Choice(ParamType[_ValueT_co], t.Generic[_ValueT_co]):
- sym:2081 Choice.normalize_choice (src/click/types.py:406-424)
  def normalize_choice(self, choice: object, ctx: Context | None) -> str:
- sym:2115 BoolParamType.str_to_bool (src/click/types.py:856-868)
  def str_to_bool(value: str | bool) -> bool | None:
- sym:1721 Context.get_parameter_source (src/click/core.py:945-961)
  def get_parameter_source(self, name: str) -> ParameterSource | None:
- sym:2137 Tuple (src/click/types.py:1258-1308)
  class Tuple(CompositeParamType[tuple[t.Any, ...]]):
- sym:1722 Command (src/click/core.py:964-1660)
  class Command:
- sym:1827 Argument.make_metavar (src/click/core.py:3759-3776)
  def make_metavar(self, ctx: Context) -> str:
- sym:1730 Command.get_help_option_names (src/click/core.py:1182-1195)
  def get_help_option_names(self, ctx: Context) -> list[str]:
- sym:1826 Argument.human_readable_name (src/click/core.py:3754-3757)
  def human_readable_name(self) -> str:
- sym:1873 MissingParameter.__str__ (src/click/exceptions.py:224-229)
  def __str__(self) -> str:
- sym:1823 Option.process_value (src/click/core.py:3698-3713)
  def process_value(self, ctx: Context, value: t.Any) -> t.Any: # process_value has to be overridden on Options in order to capture # `value == UNSET` cases before `type_cast_value()` gets called. # # Refs: # https://github.com/pallets/click/issues/3069
- sym:2559 test_options_sharing_name_no_warning (tests/test_options.py:3450-3464)
  def test_options_sharing_name_no_warning(runner):
- sym:1578 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:1668 ConsoleStream.name (src/click/_winconsole.py:196-197)
  def name(self) -> str:
- sym:1689 Context (src/click/core.py:213-961)
  class Context:
- sym:1702 Context.command_path (src/click/core.py:720-736)
  def command_path(self) -> str:
- sym:1724 Command.to_info_dict (src/click/core.py:1084-1093)
  def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
- sym:1729 Command.collect_usage_pieces (src/click/core.py:1171-1180)
  def collect_usage_pieces(self, ctx: Context) -> list[str]:
- sym:1731 Command.get_help_option (src/click/core.py:1197-1229)
  def get_help_option(self, ctx: Context) -> Option | None:
- sym:1732 Command.make_parser (src/click/core.py:1231-1236)
  def make_parser(self, ctx: Context) -> _OptionParser:
- sym:1737 Command.format_options (src/click/core.py:1300-1310)
  def format_options(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:1738 Command.format_arguments (src/click/core.py:1312-1323)
  def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:1741 Command.parse_args (src/click/core.py:1371-1405)
  def parse_args(self, ctx: Context, args: list[str]) -> list[str]:
- sym:1743 Command.shell_complete (src/click/core.py:1423-1468)
  def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
- sym:1780 Parameter (src/click/core.py:2216-2902)
  class Parameter(ABC):
- sym:1801 Parameter.get_error_hint (src/click/core.py:2871-2879)
  def get_error_hint(self, ctx: Context | None) -> str:
- sym:1824 Argument (src/click/core.py:3716-3819)
  class Argument(Parameter):
- sym:1831 Argument.get_error_hint (src/click/core.py:3813-3816)
  def get_error_hint(self, ctx: Context | None) -> str:
- sym:2227 test_deprecated_usage (tests/test_arguments.py:432-440)
  def test_deprecated_usage(runner):
- sym:2232 test_deprecated_warning (tests/test_arguments.py:514-532)
  def test_deprecated_warning(runner, deprecated):
- sym:2262 test_flag_value_dual_options (tests/test_basic.py:395-415)
  def test_flag_value_dual_options(runner, default, args, expected):
- sym:2223 test_argument_help_deprecated_without_help_lists_all (tests/test_arguments.py:363-380)
  def test_argument_help_deprecated_without_help_lists_all(runner):
- sym:2229 test_deprecated_usage_help_record (tests/test_arguments.py:473-483)
  def test_deprecated_usage_help_record(runner, deprecated, expected_label):
- sym:2230 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:486-496)
  def test_deprecated_usage_help_record_without_help(runner):
- sym:2448 test_deprecated_usage (tests/test_options.py:48-58)
  def test_deprecated_usage(runner, deprecated):
- sym:2247 test_param_named_help (tests/test_basic.py:46-70)
  def test_param_named_help(runner, help_names, params, args, expected_output):
- sym:2451 test_deprecated_warning (tests/test_options.py:100-118)
  def test_deprecated_warning(runner, deprecated):
- sym:2233 test_deprecated_required (tests/test_arguments.py:535-537)
  def test_deprecated_required(runner):
- sym:2453 test_deprecated_prompt (tests/test_options.py:126-128)
  def test_deprecated_prompt(runner):
- sym:2561 test_flag_group_competition_four_flags (tests/test_options.py:3539-3551)
  def test_flag_group_competition_four_flags(runner, opts, args, expected):
- sym:2555 test_flag_group_competition_non_boolean (tests/test_options.py:3340-3354)
  def test_flag_group_competition_non_boolean(runner, opts, args, expected):
- sym:2321 test_deprecated_in_help_messages (tests/test_commands.py:559-568)
  def test_deprecated_in_help_messages(runner, doc, deprecated):
- sym:2442 CONTEXT_SETTINGS (tests/test_normalization.py:3-3)
  CONTEXT_SETTINGS = dict(token_normalize_func=lambda x: x.lower())
- sym:2248 test_param_squatting_help_option_name (tests/test_basic.py:73-87)
  def test_param_squatting_help_option_name(runner):
- sym:2323 test_deprecated_in_invocation (tests/test_commands.py:588-597)
  def test_deprecated_in_invocation(runner, deprecated):
- sym:2231 test_deprecated_empty_help_no_leading_space (tests/test_arguments.py:504-510)
  def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
- sym:2452 test_deprecated_required (tests/test_options.py:121-123)
  def test_deprecated_required(runner):
- sym:2439 test_param_to_info_dict_deprecated_help (tests/test_info_dict.py:321-324)
  def test_param_to_info_dict_deprecated_help(make_param, help_in, deprecated, help_out):
- sym:2840 test_parameter_duplication (tests/test_utils/test_sentinel.py:90-97)
  def test_parameter_duplication(duplicate):
- sym:2361 test_no_state_leaks (tests/test_context.py:436-519)
  def test_no_state_leaks(runner):
- sym:2558 test_argument_sharing_param_name_warning (tests/test_options.py:3431-3447)
  def test_argument_sharing_param_name_warning(runner):
- sym:2384 test_default_map_source (tests/test_defaults.py:254-272)
  def test_default_map_source(runner, args, default_map, expected_value, expected_source):
- sym:2449 test_deprecated_empty_help_no_leading_space (tests/test_options.py:66-72)
  def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
- sym:2396 test_context_protected_args_deprecated (tests/test_deprecations.py:63-67)
  def test_context_protected_args_deprecated():
- sym:2444 test_choice_normalization (tests/test_normalization.py:18-50)
  def test_choice_normalization(runner):
- sym:2777 test_file_surrogates (tests/test_types/test_File.py:13-31)
  def test_file_surrogates(type, tmp_path):
- sym:2322 test_deprecated_empty_help_no_leading_space (tests/test_commands.py:573-584)
  def test_deprecated_empty_help_no_leading_space(runner, doc, deprecated):

---
tokens: 3869/8000
