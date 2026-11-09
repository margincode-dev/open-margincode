## Task
# click-9f0bcdf · click-9f0bcdf · open
Goal: Add `Parameter.spec` and deprecate `human_readable_name`

## Map
- src/click/core.py
- src/click/_compat.py
- src/click/_winconsole.py
- src/click/testing.py
- src/click/types.py
- examples/aliases/aliases.py
- examples/complex/complex/cli.py
- examples/naval/naval.py
- src/click/__init__.py
- src/click/_termui_impl.py
- src/click/_utils.py

## Code
- sym:334 Parameter.human_readable_name (src/click/core.py:2520-2524)
  def human_readable_name(self) -> str:
- sym:375 Argument.human_readable_name (src/click/core.py:3838-3841)
  def human_readable_name(self) -> str:
- sym:349 Parameter.get_error_hint (src/click/core.py:2929-2937)
  def get_error_hint(self, ctx: Context | None) -> str:
- sym:380 Argument.get_error_hint (src/click/core.py:3899-3902)
- sym:346 Parameter.handle_parse_result (src/click/core.py:2835-2921)
- sym:328 Parameter.__init__ (src/click/core.py:2356-2438)
- sym:373 Argument (src/click/core.py:3800-3905)
  class Argument(Parameter):
- sym:683 Path.convert (src/click/types.py:1161-1233)
- sym:769 test_parameter_name_not_an_identifier_is_deprecated (tests/test_arguments.py:278-293)
- sym:124 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:214 ConsoleStream.name (src/click/_winconsole.py:196-197)
- sym:586 _NamedTextIOWrapper.name (src/click/testing.py:203-204)
- sym:689 Tuple.name (src/click/types.py:1282-1283)
- sym:8 AliasedGroup.resolve_command (examples/aliases/aliases.py:70-73)
- sym:32 ComplexCLI.get_command (examples/complex/complex/cli.py:40-45)
- sym:57 ship_new (examples/naval/naval.py:22-24)

---
tokens: 713/800
omitted 33 lower-ranked items (expand an id for detail):
- repo map: .devcontainer/ .editorconfig .github/ .gitignore .pre-commit-config.yaml .readthedocs.yaml CHANGES.md LICENSE.txt README.md docs/ examples/ pyproject.toml (178 files)
- sym:87 __getattr__ (src/click/__init__.py:76-144)
- sym:91 is_ascii_encoding (src/click/_compat.py:43-48)
- sym:99 _FixupStream.__getattr__ (src/click/_compat.py:105-106)
- sym:121 open_stream (src/click/_compat.py:374-452)
- sym:126 _AtomicFile.__getattr__ (src/click/_compat.py:473-474)
- sym:162 _PagerWriter.__getattr__ (src/click/_termui_impl.py:419-420)
- … and 26 more
