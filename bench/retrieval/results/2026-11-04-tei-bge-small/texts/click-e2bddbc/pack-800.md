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
- sym:2950 __getattr__ (src/click/__init__.py:76-144)
  def __getattr__(name: str) -> object:
- sym:3242 __getattr__ (src/click/core.py:3814-3835)
  def __getattr__(name: str) -> object:
- sym:3136 Command.get_params (src/click/core.py:1107-1161)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:3282 MissingParameter.__str__ (src/click/exceptions.py:224-229)
```python
    def __str__(self) -> str:
        if not self.message:
            param_name = self.param.name if self.param else None
            return _("Missing parameter: {param_name}").format(param_name=param_name)
        else:
            return self.message
```
- sym:3207 Parameter.handle_parse_result (src/click/core.py:2776-2862)
- sym:3597 __getattr__ (src/click/utils.py:669-688)
- sym:3513 _NumberRangeBase.__repr__ (src/click/types.py:731-733)
- sym:3279 MissingParameter (src/click/exceptions.py:159-229)
- sym:3348 __getattr__ (src/click/parser.py:503-533)
- sym:3031 _tempfilepager (src/click/_termui_impl.py:636-667)
- sym:3506 _NumberParamTypeBase.convert (src/click/types.py:598-610)
- sym:3390 _is_incomplete_option (src/click/shell_completion.py:671-693)
- sym:2987 _AtomicFile.name (src/click/_compat.py:463-464)
- sym:3077 ConsoleStream.name (src/click/_winconsole.py:196-197)
- sym:3161 _BaseCommand (src/click/core.py:1670-1674)
- sym:3182 _MultiCommand (src/click/core.py:2140-2144)
- sym:4209 test_expand_args (tests/test_utils/test__expand_args.py:6-17)
- sym:3652 test_param_squatting_help_option_name (tests/test_basic.py:73-87)

---
tokens: 675/800
