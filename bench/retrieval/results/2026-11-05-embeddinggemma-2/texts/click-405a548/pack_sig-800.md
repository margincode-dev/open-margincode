## Task
# click-405a548 · click-405a548 · open
Goal: Move common `help` argument from `Option` and `Argument` to `Parameter`

## Map
- src/click/core.py
- src/click/decorators.py
- src/click/exceptions.py
- src/click/parser.py
- src/click/shell_completion.py
- src/click/types.py
- repo map: .devcontainer/ .editorconfig .github/ .gitignore .pre-commit-config.yaml .readthedocs.yaml CHANGES.md LICENSE.txt README.md docs/ examples/ pyproject.toml (178 files)

## Code
- sym:7448 Argument (src/click/core.py:3702-3814)
  class Argument(Parameter):
- sym:7351 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:7405 Parameter.__init__ (src/click/core.py:2327-2399)
- sym:7355 Command.get_help_option (src/click/core.py:1197-1229)
  def get_help_option(self, ctx: Context) -> Option | None:
- sym:7404 Parameter (src/click/core.py:2215-2883)
  class Parameter(ABC):
- sym:7427 Option (src/click/core.py:2886-3699)
  class Option(Parameter):
- sym:7475 argument (src/click/decorators.py:324-349)
- sym:7476 option (src/click/decorators.py:352-377)
- sym:7136 ship_move (examples/naval/naval.py:32-34)
- sym:7441 Option.get_help_record (src/click/core.py:3397-3420)
- sym:7303 _HELP_OPTION_STORAGE_NAME (src/click/core.py:60-60)
- sym:7346 Command (src/click/core.py:964-1659)
- sym:7310 augment_usage_errors (src/click/core.py:129-144)
- sym:7311 iter_params_for_processing (src/click/core.py:147-171)
- sym:7347 Command.__init__ (src/click/core.py:1040-1082)
- sym:7361 Command.format_options (src/click/core.py:1300-1310)

---
tokens: 701/800
omitted 39 lower-ranked items (expand an id for detail):
- sym:7362 Command.format_arguments (src/click/core.py:1312-1322)
- sym:7367 Command.shell_complete (src/click/core.py:1422-1467)
- sym:7378 Group.__init__ (src/click/core.py:1736-1789)
- sym:7446 Option.consume_value (src/click/core.py:3628-3682)
- sym:7459 FC (src/click/decorators.py:25-25)
- sym:7474 _param_memo (src/click/decorators.py:314-321)
- sym:7477 confirmation_option (src/click/decorators.py:380-401)
- sym:7478 password_option (src/click/decorators.py:404-418)
- … and 31 more
