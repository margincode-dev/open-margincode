## Task
# click-b90faad · click-b90faad · open
Goal: Deprecate a parameter name that Click 9.0 will refuse or lower case

Prepares the ground for #3827
Workspace: /root/bench/av_gemma/ws/rec728b-click-b90faad/click-b90faad
Changed (0): nothing yet

## Map
- repo map (178 files):
  .devcontainer/ (2 files)
  .editorconfig
  .github/ (10 files)
  .gitignore
  .pre-commit-config.yaml
  .readthedocs.yaml
  CHANGES.md
  LICENSE.txt
  README.md
  docs/ (41 files)
  examples/ (39 files)
  pyproject.toml
  src/ (18 files)
  tests/ (59 files)
  uv.lock

## Code
- sym:1828 Argument._parse_decls (src/click/core.py:3778-3795)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:2155 _posixify (src/click/utils.py:32-33)
  def _posixify(name: str) -> str:
- sym:1644 UNSET (src/click/_utils.py:31-31)
  UNSET: t.Literal[Sentinel.UNSET] = Sentinel.UNSET
- sym:2087 Choice.shell_complete (src/click/types.py:495-515)
  def shell_complete( self, ctx: Context, param: Parameter, incomplete: str ) -> list[CompletionItem]:
- sym:1646 T_UNSET (src/click/_utils.py:41-41)
  T_UNSET: t.TypeAlias = t.Literal[Sentinel.UNSET]
- sym:2114 BoolParamType (src/click/types.py:819-885)
  class BoolParamType(ParamType[bool]):
- sym:1647 T_FLAG_NEEDS_VALUE (src/click/_utils.py:44-44)
  T_FLAG_NEEDS_VALUE: t.TypeAlias = t.Literal[Sentinel.FLAG_NEEDS_VALUE]
- sym:2149 STRING (src/click/types.py:1415-1415)
  STRING: t.Final[StringParamType] = StringParamType()
- sym:1723 Command.__init__ (src/click/core.py:1040-1082)
  def __init__( self, name: str | None, context_settings: cabc.MutableMapping[str, t.Any] | None = None, callback: t.Callable[..., t.Any] | None = None, params: list[Parameter] | None = None, help: str | None = None, epilog: str | None = None, short_help: str | None = None, options_metavar: str | None = "[OPTIONS]", add_help_option: bool = True, no_args_is_help: bool = False, hidden: bool = False, deprecated: bool | str = False, ) -> None: #: the name the command thinks it has. Upon registering a command #: on a :class:`Group` the group will default the command name #: with this information. You should instead use the #: :class:`Context`\'s :attr:`~Context.info_name` attribute.
- sym:1461 AliasedGroup.get_command (examples/aliases/aliases.py:42-68)
  def get_command(self, ctx, cmd_name): # Step one: built-in commands as normal
- sym:1645 FLAG_NEEDS_VALUE (src/click/_utils.py:34-34)
  FLAG_NEEDS_VALUE: t.Literal[Sentinel.FLAG_NEEDS_VALUE] = Sentinel.FLAG_NEEDS_VALUE
- sym:1578 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:1668 ConsoleStream.name (src/click/_winconsole.py:196-197)
  def name(self) -> str:
- sym:1781 Parameter.__init__ (src/click/core.py:2335-2417)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, type: types.ParamType[t.Any] | t.Any | None = None, required: bool = False, # XXX The default historically embed two concepts: # - the declaration of a Parameter object carrying the default (handy to # arbitrage the default value of coupled Parameters sharing the same # self.name, like flag options), # - and the actual value of the default. # It is confusing and is the source of many issues discussed in: # https://github.com/pallets/click/pull/3030 # In the future, we might think of splitting it in two, not unlike # Option.is_flag and Option.flag_value: we could have something like # Parameter.is_default and Parameter.default_value. default: t.Any | t.Callable[[], t.Any] | None = UNSET, callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None = None, nargs: int | None = None, multiple: bool = False, metavar: str | None = None, expose_value: bool = True, is_eager: bool = False, envvar: str | cabc.Sequence[str] | None = None, shell_complete: t.Callable[ [Context, Parameter, str], list[CompletionItem] | list[str] ] | None = None, deprecated: bool | str = False, help: str | None = None, ) -> None:
- sym:2037 _NamedTextIOWrapper.name (src/click/testing.py:203-204)
  def name(self) -> str:
- sym:2262 test_flag_value_dual_options (tests/test_basic.py:395-415)
  def test_flag_value_dual_options(runner, default, args, expected):
- sym:2559 test_options_sharing_name_no_warning (tests/test_options.py:3450-3464)
  def test_options_sharing_name_no_warning(runner):
- sym:2717 _CustomTypeNoValue (tests/test_termui.py:1731-1737)
  class _CustomTypeNoValue(click.ParamType):

---
tokens: 1324/2000
