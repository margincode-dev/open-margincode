## Task
# click-f25f697 · click-f25f697 · open
Goal: List every argument in the `Positional arguments` help section

Cloup already renders the section this way:

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
```python
    def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
        """Writes the arguments that have a help record into the formatter."""
        args = []
        for param in self.get_params(ctx):
            rv = param.get_help_record(ctx)
            if rv is not None and isinstance(param, Argument):
                args.append(rv)

        if args:
            with formatter.section(_("Positional arguments")):
                formatter.write_dl(args)
```
- sym:8851 Argument (src/click/core.py:3702-3814)
  class Argument(Parameter):
- sym:8878 argument (src/click/decorators.py:324-349)
  def argument( *param_decls: str, cls: type[Argument] | None = None, **attrs: t.Any ) -> t.Callable[[FC], FC]:
- sym:8857 Argument.get_usage_pieces (src/click/core.py:3799-3800)
  def get_usage_pieces(self, ctx: Context) -> list[str]:
- sym:8960 _OptionParser.parse_args (src/click/parser.py:298-314)
- sym:8795 Group.format_commands (src/click/core.py:1984-2010)
- sym:8764 Command.format_options (src/click/core.py:1300-1310)
- sym:8879 option (src/click/decorators.py:352-377)
- sym:9248 test_argument_help_options_only_no_arguments_section (tests/test_arguments.py:314-324)
  def test_argument_help_options_only_no_arguments_section(runner):
- sym:9247 test_argument_help (tests/test_arguments.py:295-311)
  def test_argument_help(runner):
- sym:8716 Context (src/click/core.py:213-961)
- sym:8754 Command.get_params (src/click/core.py:1107-1161)
- sym:8762 Command.format_help (src/click/core.py:1263-1280)
- sym:8826 Parameter.get_help_record (src/click/core.py:2846-2847)
- sym:9252 test_deprecated_usage_help_record (tests/test_arguments.py:380-390)
- sym:9253 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:393-403)
- sym:9295 test_datetime_argument_optional_metavar (tests/test_basic.py:678-688)

---
tokens: 679/800
