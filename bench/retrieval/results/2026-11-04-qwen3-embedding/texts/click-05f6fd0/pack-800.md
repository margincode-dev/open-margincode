## Task
# click-05f6fd0 · click-05f6fd0 · open
Goal: Always return a help record for an `Argument`

## Map
- src/click/core.py
- src/click/decorators.py
- src/click/shell_completion.py
- tests/test_arguments.py
- repo map: .devcontainer/ .editorconfig .github/ .gitignore .pre-commit-config.yaml .readthedocs.yaml CHANGES.md LICENSE.txt README.md docs/ examples/ pyproject.toml (178 files)

## Code
- sym:5955 Command.format_arguments (src/click/core.py:1312-1326)
  def format_arguments(self, ctx: Context, formatter: HelpFormatter) -> None:
- sym:6041 Argument (src/click/core.py:3706-3818)
  class Argument(Parameter):
- sym:6048 Argument.get_help_record (src/click/core.py:3806-3810)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:6016 Parameter.get_help_record (src/click/core.py:2850-2851)
- sym:6034 Option.get_help_record (src/click/core.py:3401-3424)
- sym:5954 Command.format_options (src/click/core.py:1300-1310)
- sym:6068 argument (src/click/decorators.py:324-349)
- sym:6033 Option.get_help_spec (src/click/core.py:3370-3399)
- sym:5948 Command.get_help_option (src/click/core.py:1197-1229)
- sym:6043 Argument.to_info_dict (src/click/core.py:3754-3757)
- sym:5944 Command.get_params (src/click/core.py:1107-1161)
- sym:5971 Group.__init__ (src/click/core.py:1740-1793)
- sym:5997 Parameter (src/click/core.py:2219-2887)
  class Parameter(ABC):
- sym:6052 FC (src/click/decorators.py:25-25)
- sym:6067 _param_memo (src/click/decorators.py:314-321)
- sym:6197 _is_incomplete_argument (src/click/shell_completion.py:639-659)

---
tokens: 704/800
omitted 32 lower-ranked items (expand an id for detail):
- sym:6446 test_deprecated_usage_help_record (tests/test_arguments.py:458-468)
- sym:6447 test_deprecated_usage_help_record_without_help (tests/test_arguments.py:471-481)
- sym:6448 test_deprecated_empty_help_no_leading_space (tests/test_arguments.py:489-495)
- sym:6421 test_nargs_star (tests/test_arguments.py:10-20)
- sym:6422 test_nargs_tup (tests/test_arguments.py:23-34)
- sym:6423 test_nargs_tup_composite (tests/test_arguments.py:46-55)
- sym:6424 test_nargs_mismatch_with_tuple_type (tests/test_arguments.py:58-64)
- sym:6425 test_nargs_err (tests/test_arguments.py:67-79)
- … and 24 more
