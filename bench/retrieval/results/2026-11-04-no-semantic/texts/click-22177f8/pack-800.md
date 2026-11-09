## Task
# click-22177f8 · click-22177f8-2 · open
Goal: `path_type` narrows the converted value's type for `convert` and `prompt`

Closes #3822

## Map
- examples/validation/validation.py
- src/click/termui.py
- src/click/types.py
- src/click/core.py
- src/click/testing.py
- tests/test_abort_interrupt.py
- tests/test_defaults.py
- tests/test_options.py
- tests/test_stream_lifecycle.py
- tests/test_termui.py
- tests/test_testing.py
- tests/test_types/test_File.py

## Code
- sym:10492 ParamType.convert (src/click/types.py:168-192)
  def convert( self, value: _InputT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:9969 URL.convert (examples/validation/validation.py:15-24)
  def convert(self, value, param, ctx) -> urlparse.ParseResult:
- sym:10421 prompt (src/click/termui.py:139-150)
- sym:10501 FuncParamType.convert (src/click/types.py:264-278)
- sym:10503 UnprocessedParamType.convert (src/click/types.py:284-287)
- sym:10485 ParamType (src/click/types.py:54-228)
- sym:10525 DateTime.convert (src/click/types.py:555-576)
- sym:10520 DateTime (src/click/types.py:511-579)
  class DateTime(ParamType[datetime]):
- sym:10119 Context (src/click/core.py:213-961)
  class Context:
- sym:10140 Context.fail (src/click/core.py:812-818)
- sym:10210 Parameter (src/click/core.py:2215-2883)
- sym:10223 Parameter.type_cast_value (src/click/core.py:2557-2611)
- sym:10234 Option.__init__ (src/click/core.py:2979-3058)
- sym:10237 Option._infer_flag_kind (src/click/core.py:3082-3113)
- sym:10239 Option._resolve_lazy_default (src/click/core.py:3154-3176)
- sym:10240 Option._validate (src/click/core.py:3178-3198)

---
tokens: 705/800
omitted 38 lower-ranked items (expand an id for detail):
- tests/test_types/test_FloatRange.py
- repo map: .devcontainer/ .editorconfig .github/ .gitignore .pre-commit-config.yaml .readthedocs.yaml CHANGES.md LICENSE.txt README.md docs/ examples/ pyproject.toml (177 files)
- sym:10241 Option.to_info_dict (src/click/core.py:3200-3221)
- sym:10249 Option.prompt_for_value (src/click/core.py:3506-3554)
- sym:10252 Option.consume_value (src/click/core.py:3628-3682)
- sym:10417 hidden_prompt_func (src/click/termui.py:79-82)
- sym:10419 _build_prompt (src/click/termui.py:108-125)
- … and 31 more
