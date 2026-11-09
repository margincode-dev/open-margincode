## Task
# click-22177f8 · click-22177f8-2 · open
Goal: `path_type` narrows the converted value's type for `convert` and `prompt`

Closes #3822
Workspace: /root/bench/ws/rebf815-click-22177f8/click-22177f8-2
Changed (0): nothing yet

## Map
- examples/validation/validation.py
  L6: def validate_count(ctx, param, value):
  L12: class URL(click.ParamType[urlparse.ParseResult]):
  L15: def convert(self, value, param, ctx) -> urlparse.ParseResult:
  L34: def cli(count, foo, url):
- src/click/termui.py
- src/click/types.py
- src/click/core.py
- src/click/testing.py
- tests/test_abort_interrupt.py
  L25: class RecordingStderr:
  L33: def __init__(self, interrupt_at=None):
  L38: def isatty(self):
  L46: def write(self, value):
  L50: def flush(self):
  L54: def _interrupt_prompt(text):
  L59: def _prompting_command():
  L68: def _interrupting_command():
  L76: def _exit_code(cli, stderr, monkeypatch):
  L94: def test_interrupt_while_reporting_abort(monkeypatch, source, interrupt_at):
  L117: def test_interrupt_while_reporting_error(monkeypatch):
  L133: def test_late_interrupt_with_disabled_standalone_mode(monkeypatch):
  L153: def test_interrupt_while_exiting_after_success(monkeypatch):
- tests/test_defaults.py
- tests/test_options.py
- tests/test_stream_lifecycle.py
- tests/test_termui.py
- tests/test_testing.py
- tests/test_types/test_File.py
- tests/test_types/test_FloatRange.py
- repo map: .devcontainer/ .editorconfig .github/ .gitignore .pre-commit-config.yaml .readthedocs.yaml CHANGES.md LICENSE.txt README.md docs/ examples/ pyproject.toml (177 files)

## Code
- sym:10492 ParamType.convert (src/click/types.py:168-192)
  def convert( self, value: _InputT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:9969 URL.convert (examples/validation/validation.py:15-24)
  def convert(self, value, param, ctx) -> urlparse.ParseResult:
- sym:10421 prompt (src/click/termui.py:139-150)
  def prompt( text: str, default: str | None = None, hide_input: bool = False, confirmation_prompt: bool | str = False, type: None = None, value_proc: None = None, prompt_suffix: str = ": ", show_default: bool | str = True, err: bool = False, show_choices: bool = True, ) -> str:
- sym:10501 FuncParamType.convert (src/click/types.py:264-278)
  def convert( self, value: _ValueT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:10503 UnprocessedParamType.convert (src/click/types.py:284-287)
  def convert( self, value: _ValueT, param: Parameter | None, ctx: Context | None ) -> _ValueT:
- sym:10485 ParamType (src/click/types.py:54-228)
- sym:10525 DateTime.convert (src/click/types.py:555-576)
- sym:10520 DateTime (src/click/types.py:511-579)
  class DateTime(ParamType[datetime]):
- sym:10119 Context (src/click/core.py:213-961)
  class Context:
- sym:10140 Context.fail (src/click/core.py:812-818)
- sym:10210 Parameter (src/click/core.py:2215-2883)
  class Parameter(ABC):
- sym:10223 Parameter.type_cast_value (src/click/core.py:2557-2611)
- sym:10234 Option.__init__ (src/click/core.py:2979-3058)
- sym:10237 Option._infer_flag_kind (src/click/core.py:3082-3113)
- sym:10239 Option._resolve_lazy_default (src/click/core.py:3154-3176)
- sym:10240 Option._validate (src/click/core.py:3178-3198)
- sym:10241 Option.to_info_dict (src/click/core.py:3200-3221)
- sym:10249 Option.prompt_for_value (src/click/core.py:3506-3554)
- sym:10252 Option.consume_value (src/click/core.py:3628-3682)
- sym:10417 hidden_prompt_func (src/click/termui.py:79-82)
- sym:10419 _build_prompt (src/click/termui.py:108-125)
- sym:10424 confirm (src/click/termui.py:289-345)
- sym:10481 CliRunner.isolation (src/click/testing.py:399-594)
- sym:10494 ParamType.fail (src/click/types.py:204-211)
- sym:11256 test_prompt_default_round_trips_through_type (tests/test_utils/test_prompt.py:17-31)
- sym:11283 DoublingType.convert (tests/typing/typing_prompt.py:35-38)
- sym:11282 DoublingType (tests/typing/typing_prompt.py:32-38)
- sym:11214 test_path_surrogates (tests/test_types/test_Path.py:90-143)
- sym:10627 _prompting_command (tests/test_abort_interrupt.py:59-65)
- sym:10811 test_parameter_source_during_paramtype_convert (tests/test_defaults.py:275-315)
- sym:10877 test_deprecated_prompt (tests/test_options.py:126-128)
- sym:10898 test_dynamic_default_help_unset (tests/test_options.py:713-728)
- sym:10899 test_dynamic_default_help_text (tests/test_options.py:731-746)
- sym:10910 test_callback_validates_prompt (tests/test_options.py:896-909)
- sym:10933 test_string_show_default_shows_custom_string_in_prompt (tests/test_options.py:1359-1369)
- sym:10962 test_dual_options_custom_type_sentinel_flag_value (tests/test_options.py:2124-2163)
- sym:11043 test_invoke_with_logger_and_prompt (tests/test_stream_lifecycle.py:204-217)
- sym:11080 test_choices_list_in_prompt (tests/test_termui.py:257-279)
- sym:11081 test_file_prompt_default_format (tests/test_termui.py:285-292)
- sym:11130 test_prompt_required_with_required (tests/test_termui.py:1326-1333)
- sym:11131 test_prompt_required_false (tests/test_termui.py:1353-1365)
- sym:11132 test_confirmation_prompt (tests/test_termui.py:1377-1394)
- sym:11134 test_false_show_default_cause_no_default_display_in_prompt (tests/test_termui.py:1447-1456)
- sym:11140 test_flag_value_prompt (tests/test_termui.py:1679-1728)
- sym:11159 test_hide_input_error_message (tests/test_termui.py:1929-1942)
- sym:11160 test_hide_input_confirmation_prompt_mismatch_unaffected (tests/test_termui.py:1945-1963)
- sym:11161 test_hide_input_value_never_leaks_when_err_true (tests/test_termui.py:1966-1984)
- sym:11164 test_echo_stdin_prompts (tests/test_testing.py:52-89)
- sym:11166 test_prompts (tests/test_testing.py:115-134)
- sym:11201 test_file_surrogates (tests/test_types/test_File.py:13-31)
- sym:11202 test_range (tests/test_types/test_FloatRange.py:16-17)
- sym:11203 test_range_fail (tests/test_types/test_FloatRange.py:27-31)

---
tokens: 1770/2000
