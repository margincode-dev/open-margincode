## Task
# click-b90faad · click-b90faad · open
Goal: Deprecate a parameter name that Click 9.0 will refuse or lower case

Prepares the ground for #3827
Workspace: /root/bench/av_tei/ws/rec09fc-click-b90faad/click-b90faad
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
- sym:1873 MissingParameter.__str__ (src/click/exceptions.py:224-229)
  def __str__(self) -> str:
- sym:2087 Choice.shell_complete (src/click/types.py:495-515)
  def shell_complete( self, ctx: Context, param: Parameter, incomplete: str ) -> list[CompletionItem]:
- sym:2188 __getattr__ (src/click/utils.py:657-676)
  def __getattr__(name: str) -> object:
- sym:1939 __getattr__ (src/click/parser.py:503-533)
  def __getattr__(name: str) -> object:
- sym:2114 BoolParamType (src/click/types.py:819-885)
  class BoolParamType(ParamType[bool]):
- sym:1541 __getattr__ (src/click/__init__.py:76-144)
  def __getattr__(name: str) -> object:
- sym:1828 Argument._parse_decls (src/click/core.py:3778-3795)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:1742 Command.invoke (src/click/core.py:1407-1421)
  def invoke(self, ctx: Context) -> t.Any:
- sym:2155 _posixify (src/click/utils.py:32-33)
  def _posixify(name: str) -> str:
- sym:1752 _BaseCommand (src/click/core.py:1671-1675)
  class _BaseCommand(Command, metaclass=_FakeSubclassCheck):
- sym:1723 Command.__init__ (src/click/core.py:1040-1082)
  def __init__( self, name: str | None, context_settings: cabc.MutableMapping[str, t.Any] | None = None, callback: t.Callable[..., t.Any] | None = None, params: list[Parameter] | None = None, help: str | None = None, epilog: str | None = None, short_help: str | None = None, options_metavar: str | None = "[OPTIONS]", add_help_option: bool = True, no_args_is_help: bool = False, hidden: bool = False, deprecated: bool | str = False, ) -> None: #: the name the command thinks it has. Upon registering a command #: on a :class:`Group` the group will default the command name #: with this information. You should instead use the #: :class:`Context`\'s :attr:`~Context.info_name` attribute.
- sym:1784 Parameter.__repr__ (src/click/core.py:2453-2454)
  def __repr__(self) -> str:
- sym:1461 AliasedGroup.get_command (examples/aliases/aliases.py:42-68)
  def get_command(self, ctx, cmd_name): # Step one: built-in commands as normal
- sym:1810 Option._validate (src/click/core.py:3193-3213)
  def _validate(self, prompt: bool | str, deprecated: bool | str) -> None:
- sym:1460 AliasedGroup (examples/aliases/aliases.py:37-73)
  class AliasedGroup(click.Group):
- sym:1724 Command.to_info_dict (src/click/core.py:1084-1093)
  def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
- sym:1500 convert_rotation (examples/imagepipe/imagepipe.py:165-175)
  def convert_rotation(ctx, param, value):
- sym:1833 __getattr__ (src/click/core.py:3822-3843)
  def __getattr__(name: str) -> object:
- sym:1501 convert_flip (examples/imagepipe/imagepipe.py:178-186)
  def convert_flip(ctx, param, value):
- sym:1870 MissingParameter (src/click/exceptions.py:159-229)
  class MissingParameter(BadParameter):
- sym:1725 Command.__repr__ (src/click/core.py:1095-1096)
  def __repr__(self) -> str:
- sym:1867 BadParameter (src/click/exceptions.py:114-156)
  class BadParameter(UsageError):
- sym:1814 Option._parse_decls (src/click/core.py:3277-3336)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:2096 _NumberParamTypeBase (src/click/types.py:593-610)
  class _NumberParamTypeBase( ParamType[_ValueT_co], t.Generic[_ValueT_contra, _ValueT_co] ):
- sym:2159 _LazyFile (src/click/utils.py:103-196)
  class _LazyFile:
- sym:1795 Parameter.process_value (src/click/core.py:2650-2714)
  def process_value(self, ctx: Context, value: t.Any) -> t.Any:
- sym:1720 Context.set_parameter_source (src/click/core.py:936-943)
  def set_parameter_source(self, name: str, source: ParameterSource) -> None:
- sym:2097 _NumberParamTypeBase.convert (src/click/types.py:598-610)
  def convert( self, value: _ValueT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:1993 confirm (src/click/termui.py:289-345)
  def confirm( text: str, default: bool | None = False, abort: bool = False, prompt_suffix: str = ": ", show_default: bool = True, err: bool = False, ) -> bool:
- sym:2077 Choice (src/click/types.py:342-515)
  class Choice(ParamType[_ValueT_co], t.Generic[_ValueT_co]):
- sym:1538 URL (examples/validation/validation.py:12-24)
  class URL(click.ParamType[urlparse.ParseResult]):
- sym:2081 Choice.normalize_choice (src/click/types.py:406-424)
  def normalize_choice(self, choice: object, ctx: Context | None) -> str:
- sym:2115 BoolParamType.str_to_bool (src/click/types.py:856-868)
  def str_to_bool(value: str | bool) -> bool | None:
- sym:1684 _format_deprecated_suffix (src/click/core.py:115-121)
  def _format_deprecated_suffix(deprecated: bool | str) -> str:
- sym:1721 Context.get_parameter_source (src/click/core.py:945-961)
  def get_parameter_source(self, name: str) -> ParameterSource | None:
- sym:2137 Tuple (src/click/types.py:1258-1308)
  class Tuple(CompositeParamType[tuple[t.Any, ...]]):
- sym:2149 STRING (src/click/types.py:1415-1415)
  STRING: t.Final[StringParamType] = StringParamType()
- sym:2076 StringParamType.__repr__ (src/click/types.py:325-326)
  def __repr__(self) -> str:
- sym:1798 Parameter.handle_parse_result (src/click/core.py:2777-2863)
  def handle_parse_result( self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str] ) -> tuple[t.Any, list[str]]:
- sym:1679 _HELP_OPTION_STORAGE_NAME (src/click/core.py:60-60)
  _HELP_OPTION_STORAGE_NAME = "_click_default_help"
- sym:1786 Parameter.human_readable_name (src/click/core.py:2462-2466)
  def human_readable_name(self) -> str:
- sym:2073 UnprocessedParamType.__repr__ (src/click/types.py:300-301)
  def __repr__(self) -> str:
- sym:1881 BadOptionUsage.__init__ (src/click/exceptions.py:316-320)
  def __init__( self, option_name: str, message: str, ctx: Context | None = None ) -> None:
- sym:1804 Option.__init__ (src/click/core.py:2997-3073)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, show_default: bool | str | None = None, prompt: bool | str = False, confirmation_prompt: bool | str = False, prompt_required: bool = True, hide_input: bool = False, is_flag: bool | None = None, flag_value: t.Any = UNSET, multiple: bool = False, count: bool = False, allow_from_autoenv: bool = True, type: types.ParamType[t.Any] | t.Any | None = None, help: str | None = None, hidden: bool = False, show_choices: bool = True, show_envvar: bool = False, deprecated: bool | str = False, **attrs: t.Any, ) -> None:
- sym:1727 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:1880 BadOptionUsage (src/click/exceptions.py:304-320)
  class BadOptionUsage(UsageError):
- sym:1823 Option.process_value (src/click/core.py:3698-3713)
  def process_value(self, ctx: Context, value: t.Any) -> t.Any: # process_value has to be overridden on Options in order to capture # `value == UNSET` cases before `type_cast_value()` gets called. # # Refs: # https://github.com/pallets/click/issues/3069
- sym:1578 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:1668 ConsoleStream.name (src/click/_winconsole.py:196-197)
  def name(self) -> str:
- sym:1689 Context (src/click/core.py:213-961)
  class Context:
- sym:1747 Command._main_shell_completion (src/click/core.py:1626-1656)
  def _main_shell_completion( self, ctx_args: cabc.MutableMapping[str, t.Any], prog_name: str, complete_var: str | None = None, ) -> None:
- sym:1780 Parameter (src/click/core.py:2216-2902)
  class Parameter(ABC):
- sym:1781 Parameter.__init__ (src/click/core.py:2335-2417)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, type: types.ParamType[t.Any] | t.Any | None = None, required: bool = False, # XXX The default historically embed two concepts: # - the declaration of a Parameter object carrying the default (handy to # arbitrage the default value of coupled Parameters sharing the same # self.name, like flag options), # - and the actual value of the default. # It is confusing and is the source of many issues discussed in: # https://github.com/pallets/click/pull/3030 # In the future, we might think of splitting it in two, not unlike # Option.is_flag and Option.flag_value: we could have something like # Parameter.is_default and Parameter.default_value. default: t.Any | t.Callable[[], t.Any] | None = UNSET, callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None = None, nargs: int | None = None, multiple: bool = False, metavar: str | None = None, expose_value: bool = True, is_eager: bool = False, envvar: str | cabc.Sequence[str] | None = None, shell_complete: t.Callable[ [Context, Parameter, str], list[CompletionItem] | list[str] ] | None = None, deprecated: bool | str = False, help: str | None = None, ) -> None:
- sym:1941 CompletionItem (src/click/shell_completion.py:67-101)
  class CompletionItem(t.Generic[_ValueT_co]):
- sym:1955 ShellComplete.get_completions (src/click/shell_completion.py:354-366)
  def get_completions( self, args: list[str], incomplete: str ) -> list[CompletionItem[str]]:
- sym:2037 _NamedTextIOWrapper.name (src/click/testing.py:203-204)
  def name(self) -> str:
- sym:2140 Tuple.name (src/click/types.py:1282-1283)
  def name(self) -> str: # type: ignore[override]
- sym:2262 test_flag_value_dual_options (tests/test_basic.py:395-415)
  def test_flag_value_dual_options(runner, default, args, expected):
- sym:2559 test_options_sharing_name_no_warning (tests/test_options.py:3450-3464)
  def test_options_sharing_name_no_warning(runner):
- sym:2429 NAME_ARGUMENT (tests/test_info_dict.py:30-45)
  NAME_ARGUMENT = (
- sym:2561 test_flag_group_competition_four_flags (tests/test_options.py:3539-3551)
  def test_flag_group_competition_four_flags(runner, opts, args, expected):
- sym:2555 test_flag_group_competition_non_boolean (tests/test_options.py:3340-3354)
  def test_flag_group_competition_non_boolean(runner, opts, args, expected):
- sym:2442 CONTEXT_SETTINGS (tests/test_normalization.py:3-3)
  CONTEXT_SETTINGS = dict(token_normalize_func=lambda x: x.lower())
- sym:2231 test_deprecated_empty_help_no_leading_space (tests/test_arguments.py:504-510)
  def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
- sym:2449 test_deprecated_empty_help_no_leading_space (tests/test_options.py:66-72)
  def test_deprecated_empty_help_no_leading_space(help_text, deprecated, expected):
- sym:2248 test_param_squatting_help_option_name (tests/test_basic.py:73-87)
  def test_param_squatting_help_option_name(runner):
- sym:2431 HELLO_COMMAND (tests/test_info_dict.py:67-78)
  HELLO_COMMAND = (
- sym:2840 test_parameter_duplication (tests/test_utils/test_sentinel.py:90-97)
  def test_parameter_duplication(duplicate):
- sym:2361 test_no_state_leaks (tests/test_context.py:436-519)
  def test_no_state_leaks(runner):
- sym:2384 test_default_map_source (tests/test_defaults.py:254-272)
  def test_default_map_source(runner, args, default_map, expected_value, expected_source):
- sym:2444 test_choice_normalization (tests/test_normalization.py:18-50)
  def test_choice_normalization(runner):
- sym:2777 test_file_surrogates (tests/test_types/test_File.py:13-31)
  def test_file_surrogates(type, tmp_path):
- sym:2396 test_context_protected_args_deprecated (tests/test_deprecations.py:63-67)
  def test_context_protected_args_deprecated():
- sym:2269 test_choice_argument_custom_type (tests/test_basic.py:592-620)
  def test_choice_argument_custom_type(runner):
- sym:2476 test_dynamic_default_help_special_method (tests/test_options.py:749-760)
  def test_dynamic_default_help_special_method(runner):
- sym:2586 test_option_custom (tests/test_shell_completion.py:240-253)
  def test_option_custom():
- sym:2598 test_source_uses_lf_line_endings (tests/test_shell_completion.py:394-405)
  def test_source_uses_lf_line_endings(monkeypatch):
- sym:2600 test_context_settings (tests/test_shell_completion.py:482-492)
  def test_context_settings(runner):

---
tokens: 4099/8000
