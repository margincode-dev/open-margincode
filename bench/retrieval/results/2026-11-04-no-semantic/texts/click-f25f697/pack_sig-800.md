## Task
# click-f25f697 · click-f25f697-2 · open
Goal: List every argument in the `Positional arguments` help section

Cloup already renders the section this way:
Workspace: /root/bench/ws/rebf815-click-f25f697/click-f25f697-2
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
- sym:8765 Command.format_arguments (src/click/core.py:1312-1322)
  def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8878 argument (src/click/decorators.py:324-349)
  def argument( *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:8879 option (src/click/decorators.py:352-377)
  def option( *param_decls: str, cls: type[Option] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:8716 Context (src/click/core.py:213-961)
  class Context:
- sym:8754 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:8762 Command.format_help (src/click/core.py:1263-1280)
  def format_help(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:8826 Parameter.get_help_record (src/click/core.py:2846-2847)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:9248 test_argument_help_options_only_no_arguments_section (tests/test_arguments.py:314-324)
  def test_argument_help_options_only_no_arguments_section(runner):
- sym:9247 test_argument_help (tests/test_arguments.py:295-311)
  def test_argument_help(runner):
- sym:9252 test_deprecated_usage_help_record (tests/test_arguments.py:380-390)
  def test_deprecated_usage_help_record(runner, deprecated, expected_label):
- sym:9253 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:393-403)
  def test_deprecated_usage_help_record_without_help(runner):
- sym:9295 test_datetime_argument_optional_metavar (tests/test_basic.py:678-688)
  def test_datetime_argument_optional_metavar(runner):

---
tokens: 644/800
