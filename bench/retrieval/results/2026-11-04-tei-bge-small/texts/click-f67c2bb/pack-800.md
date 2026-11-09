## Task
# click-f67c2bb · click-f67c2bb · open
Goal: Fix short help abbreviation
Workspace: /root/bench/av_tei/ws/rec09fc-click-f67c2bb/click-f67c2bb
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
- sym:4547 Command.get_short_help_str (src/click/core.py:1247-1261)
  def get_short_help_str(self, limit: int = 45) -> str:
- sym:4971 _make_default_short_help (src/click/utils.py:62-112)
  def _make_default_short_help(help: str, max_length: int = 45) -> str:
- sym:4537 Command.to_info_dict (src/click/core.py:1084-1093)
  def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
- sym:4749 _OptionParser._match_short_opt (src/click/parser.py:390-428)
  def _match_short_opt(self, arg: str, state: _ParsingState) -> None:
- sym:4536 Command.__init__ (src/click/core.py:1040-1082)
- sym:4631 Option.get_help_extra (src/click/core.py:3435-3517)
  def get_help_extra(self, ctx: Context) -> types.OptionHelpExtra:
- sym:5631 test_make_default_short_help (tests/test_utils/test_make_default_short_help.py:32-39)
  def test_make_default_short_help(value, max_length, alter, expect):
- sym:5114 test_auto_shorthelp (tests/test_commands.py:66-95)
  def test_auto_shorthelp(runner):
- sym:4496 _format_deprecated_label (src/click/core.py:107-112)
  def _format_deprecated_label(deprecated: bool | str) -> str:
- sym:4556 Command.shell_complete (src/click/core.py:1422-1467)
  def shell_complete(self, ctx: Context, incomplete: str) -> list[CompletionItem]:
- sym:4581 Group.format_commands (src/click/core.py:1984-2010)
- sym:4585 Group.shell_complete (src/click/core.py:2120-2137)
- sym:5056 test_param_squatting_help_option_name (tests/test_basic.py:73-87)
- sym:5217 test_formatting_usage_error_help_hint (tests/test_formatting.py:285-314)
- sym:5239 HELLO_COMMAND (tests/test_info_dict.py:67-78)
- sym:5240 HELLO_GROUP (tests/test_info_dict.py:79-92)

---
tokens: 680/800
