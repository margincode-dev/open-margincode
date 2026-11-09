## Task
# click-b90faad · click-b90faad · open
Goal: Deprecate a parameter name that Click 9.0 will refuse or lower case

Prepares the ground for #3827

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
- sym:1742 Command.invoke (src/click/core.py:1407-1421)
  def invoke(self, ctx: Context) -> t.Any:
- sym:2155 _posixify (src/click/utils.py:32-33)
- sym:1752 _BaseCommand (src/click/core.py:1671-1675)
- sym:1723 Command.__init__ (src/click/core.py:1040-1082)
- sym:1784 Parameter.__repr__ (src/click/core.py:2453-2454)
  def __repr__(self) -> str:
- sym:1461 AliasedGroup.get_command (examples/aliases/aliases.py:42-68)
- sym:1810 Option._validate (src/click/core.py:3193-3213)
- sym:1578 _AtomicFile.name (src/click/_compat.py:463-464)
- sym:1668 ConsoleStream.name (src/click/_winconsole.py:196-197)
- sym:1689 Context (src/click/core.py:213-961)
- sym:1747 Command._main_shell_completion (src/click/core.py:1626-1656)
- sym:2262 test_flag_value_dual_options (tests/test_basic.py:395-415)
- sym:2559 test_options_sharing_name_no_warning (tests/test_options.py:3450-3464)

---
tokens: 681/800
