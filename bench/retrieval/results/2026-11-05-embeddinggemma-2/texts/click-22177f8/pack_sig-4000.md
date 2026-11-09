## Task
# click-22177f8 · click-22177f8 · open
Goal: `path_type` narrows the converted value's type for `convert` and `prompt`

Closes #3822
Workspace: /root/bench/av_gemma/ws/rec728b-click-22177f8/click-22177f8
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
  L32: class EchoingStdin:
  L37: def __init__(self, input: t.BinaryIO, output: t.BinaryIO) -> None:
  L42: def __getattr__(self, x: str) -> t.Any:
  L45: def _echo(self, rv: bytes) -> bytes:
  L51: def read(self, n: int = -1) -> bytes:
  L54: def read1(self, n: int = -1) -> bytes:
  L57: def readline(self, n: int = -1) -> bytes:
  L60: def readlines(self) -> list[bytes]:
  L63: def __iter__(self) -> cabc.Iterator[bytes]:
  L66: def __repr__(self) -> str:
  L71: def _pause_echo(stream: EchoingStdin | None) -> cabc.Generator[None]:
  L80: class _FDCapture:
  L95: def __init__(self, targetfd: int) -> None:
  L100: def start(self) -> None:
  L105: def stop(self) -> bytes:
  L117: class BytesIOCopy(io.BytesIO):
  L125: def __init__(self, copy_to: io.BytesIO) -> None:
  L129: def flush(self) -> None:
  L133: def write(self, b: ReadableBuffer) -> int:
  L138: class StreamMixer:
  L150: def __init__(self) -> None:
  L156: class _NamedTextIOWrapper(io.TextIOWrapper):
  L172: def __init__( self, buffer: t.BinaryIO, name: str, mode: str, **kwargs: t.Any, ) -> None:
  L184: def close(self) -> None:
  L192: def fileno(self) -> int:
  L203: def name(self) -> str:
  L207: def mode(self) -> str:
  L211: def make_input_stream( input: str | bytes | t.IO[t.Any] | None, charset: str ) -> t.BinaryIO: # Is already an input stream.
  L231: class Result:
  L262: def __init__( self, runner: CliRunner, stdout_bytes: bytes, stderr_bytes: bytes, output_bytes: bytes, return_value: t.Any, exit_code: int, exception: BaseException | None, exc_info: ExceptionInfo | None = None, ) -> None:
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
  L13: def test_file_surrogates(type, tmp_path):
- tests/test_types/test_FloatRange.py
  L16: def test_range(type, value, expect):
  L27: def test_range_fail(type, value, expect):
  L34: def test_float_range_no_clamp_open():
- repo map: .devcontainer/ .editorconfig .github/ .gitignore .pre-commit-config.yaml .readthedocs.yaml CHANGES.md LICENSE.txt README.md docs/ examples/ pyproject.toml (177 files)

## Code
- sym:10492 ParamType.convert (src/click/types.py:168-192)
  def convert( self, value: _InputT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:10501 FuncParamType.convert (src/click/types.py:264-278)
  def convert( self, value: _ValueT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:10421 prompt (src/click/termui.py:139-150)
  def prompt( text: str, default: str | None = None, hide_input: bool = False, confirmation_prompt: bool | str = False, type: None = None, value_proc: None = None, prompt_suffix: str = ": ", show_default: bool | str = True, err: bool = False, show_choices: bool = True, ) -> str:
- sym:10565 Path.convert (src/click/types.py:1146-1218)
  def convert( self, value: str | os.PathLike[str], param: Parameter | None, ctx: Context | None, ) -> str | bytes | os.PathLike[str]:
- sym:10561 Path (src/click/types.py:1048-1236)
  class Path(ParamType[str | bytes | os.PathLike[str]]):
- sym:9969 URL.convert (examples/validation/validation.py:15-24)
  def convert(self, value, param, ctx) -> urlparse.ParseResult:
- sym:10503 UnprocessedParamType.convert (src/click/types.py:284-287)
  def convert( self, value: _ValueT, param: Parameter | None, ctx: Context | None ) -> _ValueT:
- sym:10564 Path.coerce_path_result (src/click/types.py:1133-1144)
  def coerce_path_result( self, value: str | os.PathLike[str] ) -> str | bytes | os.PathLike[str]:
- sym:10485 ParamType (src/click/types.py:54-228)
  class ParamType(t.Generic[_ValueT_co, _InputT_contra], abc.ABC):
- sym:10525 DateTime.convert (src/click/types.py:555-576)
  def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> datetime:
- sym:10560 PathInfoDict (src/click/types.py:1039-1045)
  class PathInfoDict(ParamTypeInfoDict):
- sym:10520 DateTime (src/click/types.py:511-579)
  class DateTime(ParamType[datetime]):
- sym:10076 T_UNSET (src/click/_utils.py:41-41)
  T_UNSET: t.TypeAlias = t.Literal[Sentinel.UNSET]
- sym:10573 Tuple.convert (src/click/types.py:1274-1293)
  def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> tuple[t.Any, ...]:
- sym:10498 FuncParamType (src/click/types.py:253-278)
  class FuncParamType(ParamType[_ValueT_co], t.Generic[_ValueT_contra, _ValueT_co]):
- sym:10563 Path.to_info_dict (src/click/types.py:1122-1131)
  def to_info_dict(self) -> PathInfoDict:
- sym:11282 DoublingType (tests/typing/typing_prompt.py:32-38)
  class DoublingType(click.ParamType[int, str]):
- sym:11214 test_path_surrogates (tests/test_types/test_Path.py:90-143)
  def test_path_surrogates(tmp_path, monkeypatch):
- sym:10119 Context (src/click/core.py:213-961)
  class Context:
- sym:10140 Context.fail (src/click/core.py:812-818)
  def fail(self, message: str) -> t.NoReturn:
- sym:10210 Parameter (src/click/core.py:2215-2883)
  class Parameter(ABC):
- sym:10223 Parameter.type_cast_value (src/click/core.py:2557-2611)
  def type_cast_value(self, ctx: Context, value: t.Any) -> t.Any:
- sym:10234 Option.__init__ (src/click/core.py:2979-3058)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, show_default: bool | str | None = None, prompt: bool | str = False, confirmation_prompt: bool | str = False, prompt_required: bool = True, hide_input: bool = False, is_flag: bool | None = None, flag_value: t.Any = UNSET, multiple: bool = False, count: bool = False, allow_from_autoenv: bool = True, type: types.ParamType[t.Any] | t.Any | None = None, help: str | None = None, hidden: bool = False, show_choices: bool = True, show_envvar: bool = False, deprecated: bool | str = False, **attrs: t.Any, ) -> None:
- sym:10237 Option._infer_flag_kind (src/click/core.py:3082-3113)
  def _infer_flag_kind( self, is_flag: bool | None, flag_value: t.Any ) -> tuple[bool, bool]:
- sym:10239 Option._resolve_lazy_default (src/click/core.py:3154-3176)
  def _resolve_lazy_default(self, value: t.Any) -> t.Any:
- sym:10240 Option._validate (src/click/core.py:3178-3198)
  def _validate(self, prompt: bool | str, deprecated: bool | str) -> None:
- sym:10241 Option.to_info_dict (src/click/core.py:3200-3221)
  def to_info_dict(self) -> dict[str, t.Any]:
- sym:10249 Option.prompt_for_value (src/click/core.py:3506-3554)
  def prompt_for_value(self, ctx: Context) -> t.Any:
- sym:10252 Option.consume_value (src/click/core.py:3628-3682)
  def consume_value( self, ctx: Context, opts: cabc.Mapping[str, Parameter] ) -> tuple[t.Any, ParameterSource]:
- sym:10417 hidden_prompt_func (src/click/termui.py:79-82)
  def hidden_prompt_func(prompt: str) -> str:
- sym:10419 _build_prompt (src/click/termui.py:108-125)
  def _build_prompt( text: str, suffix: str, show_default: bool | str = False, default: object | None = None, show_choices: bool = True, type: object | None = None, ) -> str:
- sym:10424 confirm (src/click/termui.py:289-345)
  def confirm( text: str, default: bool | None = False, abort: bool = False, prompt_suffix: str = ": ", show_default: bool = True, err: bool = False, ) -> bool:
- sym:10481 CliRunner.isolation (src/click/testing.py:399-594)
  def isolation( self, input: str | bytes | t.IO[t.Any] | None = None, env: cabc.Mapping[str, str | None] | None = None, color: bool = False, ) -> cabc.Generator[tuple[io.BytesIO, io.BytesIO, io.BytesIO]]:
- sym:10494 ParamType.fail (src/click/types.py:204-211)
  def fail( self, message: str, param: Parameter | None = None, ctx: Context | None = None, ) -> t.NoReturn:
- sym:11161 test_hide_input_value_never_leaks_when_err_true (tests/test_termui.py:1966-1984)
  def test_hide_input_value_never_leaks_when_err_true(runner):
- sym:10962 test_dual_options_custom_type_sentinel_flag_value (tests/test_options.py:2124-2163)
  def test_dual_options_custom_type_sentinel_flag_value(runner, sentinel, args, expected):
- sym:11256 test_prompt_default_round_trips_through_type (tests/test_utils/test_prompt.py:17-31)
  def test_prompt_default_round_trips_through_type(capfd, monkeypatch):
- sym:11143 _CustomTypeWithRawValue (tests/test_termui.py:1740-1746)
  class _CustomTypeWithRawValue(click.ParamType):
- sym:11217 test_path_type (tests/test_types/test_Path.py:177-185)
  def test_path_type(runner, cls, expect):
- sym:11283 DoublingType.convert (tests/typing/typing_prompt.py:35-38)
  def convert( self, value: str, param: click.Parameter | None, ctx: click.Context | None ) -> int:
- sym:11141 _CustomTypeNoValue (tests/test_termui.py:1731-1737)
- sym:11212 test_path_resolve_symlink (tests/test_types/test_Path.py:52-72)
- sym:11144 _CustomTypeWithRawValue.convert (tests/test_termui.py:1743-1746)
- sym:11128 test_tempfile_pager_closes_file_before_unlink (tests/test_termui.py:1262-1308)
- sym:11210 test_path_dash_no_byteswarning (tests/test_types/test_Path.py:13-33)
- sym:10627 _prompting_command (tests/test_abort_interrupt.py:59-65)
- sym:10811 test_parameter_source_during_paramtype_convert (tests/test_defaults.py:275-315)
- sym:10877 test_deprecated_prompt (tests/test_options.py:126-128)
- sym:10898 test_dynamic_default_help_unset (tests/test_options.py:713-728)
- sym:10899 test_dynamic_default_help_text (tests/test_options.py:731-746)
- sym:10910 test_callback_validates_prompt (tests/test_options.py:896-909)
- sym:10933 test_string_show_default_shows_custom_string_in_prompt (tests/test_options.py:1359-1369)
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
- sym:11164 test_echo_stdin_prompts (tests/test_testing.py:52-89)
- sym:11166 test_prompts (tests/test_testing.py:115-134)
- sym:11201 test_file_surrogates (tests/test_types/test_File.py:13-31)
- sym:11202 test_range (tests/test_types/test_FloatRange.py:16-17)
- sym:11203 test_range_fail (tests/test_types/test_FloatRange.py:27-31)

---
tokens: 3566/4000
