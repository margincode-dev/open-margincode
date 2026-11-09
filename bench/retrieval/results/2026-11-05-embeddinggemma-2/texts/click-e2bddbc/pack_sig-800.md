## Task
# click-e2bddbc · click-e2bddbc · open
Goal: Deprecate a parameter name that is not a Python identifier

Click 9.0 refuses such a name; warn a release ahead so the break can diffuse.
Prepares the ground for #3827, as promised in pull/3827#issuecomment-5583963038.

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
- sym:3136 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:3190 Parameter.__init__ (src/click/core.py:2334-2416)
- sym:3055 T_UNSET (src/click/_utils.py:41-41)
  T_UNSET: t.TypeAlias = t.Literal[Sentinel.UNSET]
- sym:3207 Parameter.handle_parse_result (src/click/core.py:2776-2862)
  def handle_parse_result( self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str] ) -> tuple[t.Any, list[str]]:
- sym:3559 INT (src/click/types.py:1419-1419)
- sym:2950 __getattr__ (src/click/__init__.py:76-144)
- sym:3237 Argument._parse_decls (src/click/core.py:3777-3794)
- sym:3053 UNSET (src/click/_utils.py:31-31)
- sym:3242 __getattr__ (src/click/core.py:3814-3835)
- sym:3233 Argument (src/click/core.py:3715-3811)
  class Argument(Parameter):
- sym:3348 __getattr__ (src/click/parser.py:503-533)
- sym:3031 _tempfilepager (src/click/_termui_impl.py:636-667)
- sym:3562 UUID (src/click/types.py:1430-1430)
- sym:3390 _is_incomplete_option (src/click/shell_completion.py:671-693)
- sym:3558 STRING (src/click/types.py:1415-1415)
- sym:2987 _AtomicFile.name (src/click/_compat.py:463-464)
- sym:3077 ConsoleStream.name (src/click/_winconsole.py:196-197)
- sym:3093 _format_deprecated_suffix (src/click/core.py:115-121)
- sym:3095 augment_usage_errors (src/click/core.py:129-144)
- sym:4209 test_expand_args (tests/test_utils/test__expand_args.py:6-17)

---
tokens: 680/800
