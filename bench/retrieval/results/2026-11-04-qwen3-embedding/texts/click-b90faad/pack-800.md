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
- sym:1798 Parameter.handle_parse_result (src/click/core.py:2777-2863)
  def handle_parse_result( self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str] ) -> tuple[t.Any, list[str]]:
- sym:2114 BoolParamType (src/click/types.py:819-885)
  class BoolParamType(ParamType[bool]):
- sym:1828 Argument._parse_decls (src/click/core.py:3778-3795)
- sym:2155 _posixify (src/click/utils.py:32-33)
  def _posixify(name: str) -> str:
- sym:1723 Command.__init__ (src/click/core.py:1040-1082)
- sym:1461 AliasedGroup.get_command (examples/aliases/aliases.py:42-68)
  def get_command(self, ctx, cmd_name): # Step one: built-in commands as normal
- sym:1578 _AtomicFile.name (src/click/_compat.py:463-464)
- sym:1668 ConsoleStream.name (src/click/_winconsole.py:196-197)
- sym:1689 Context (src/click/core.py:213-961)
- sym:1747 Command._main_shell_completion (src/click/core.py:1626-1656)
- sym:2227 test_deprecated_usage (tests/test_arguments.py:432-440)
- sym:2232 test_deprecated_warning (tests/test_arguments.py:514-532)
- sym:2262 test_flag_value_dual_options (tests/test_basic.py:395-415)
- sym:2223 test_argument_help_deprecated_without_help_lists_all (tests/test_arguments.py:363-380)
- sym:2229 test_deprecated_usage_help_record (tests/test_arguments.py:473-483)
- sym:2230 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:486-496)
- sym:2559 test_options_sharing_name_no_warning (tests/test_options.py:3450-3464)
- sym:2448 test_deprecated_usage (tests/test_options.py:48-58)
- sym:2247 test_param_named_help (tests/test_basic.py:46-70)

---
tokens: 682/800
