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
- sym:1828 Argument._parse_decls (src/click/core.py:3778-3795)
  def _parse_decls( self, decls: cabc.Sequence[str], expose_value: bool ) -> tuple[str, list[str], list[str]]:
- sym:2155 _posixify (src/click/utils.py:32-33)
```python
def _posixify(name: str) -> str:
    return "-".join(name.split()).lower()
```
- sym:1644 UNSET (src/click/_utils.py:31-31)
```python
UNSET: t.Literal[Sentinel.UNSET] = Sentinel.UNSET
```
- sym:2087 Choice.shell_complete (src/click/types.py:495-515)
  def shell_complete( self, ctx: Context, param: Parameter, incomplete: str ) -> list[CompletionItem]:
- sym:1646 T_UNSET (src/click/_utils.py:41-41)
  T_UNSET: t.TypeAlias = t.Literal[Sentinel.UNSET]
- sym:2114 BoolParamType (src/click/types.py:819-885)
  class BoolParamType(ParamType[bool]):
- sym:1647 T_FLAG_NEEDS_VALUE (src/click/_utils.py:44-44)
- sym:2149 STRING (src/click/types.py:1415-1415)
- sym:1723 Command.__init__ (src/click/core.py:1040-1082)
- sym:1461 AliasedGroup.get_command (examples/aliases/aliases.py:42-68)
- sym:1645 FLAG_NEEDS_VALUE (src/click/_utils.py:34-34)
- sym:1578 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:1668 ConsoleStream.name (src/click/_winconsole.py:196-197)
- sym:1781 Parameter.__init__ (src/click/core.py:2335-2417)
- sym:2037 _NamedTextIOWrapper.name (src/click/testing.py:203-204)
- sym:2262 test_flag_value_dual_options (tests/test_basic.py:395-415)
- sym:2559 test_options_sharing_name_no_warning (tests/test_options.py:3450-3464)
- sym:2717 _CustomTypeNoValue (tests/test_termui.py:1731-1737)

---
tokens: 682/800
