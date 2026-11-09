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
- sym:2087 Choice.shell_complete (src/click/types.py:495-515)
  def shell_complete( self, ctx: Context, param: Parameter, incomplete: str ) -> list[CompletionItem]:
- sym:2188 __getattr__ (src/click/utils.py:657-676)
  def __getattr__(name: str) -> object:
- sym:1873 MissingParameter.__str__ (src/click/exceptions.py:224-229)
  def __str__(self) -> str:
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

---
tokens: 2519/4000
