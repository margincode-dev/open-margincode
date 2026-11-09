## Task
# click-22177f8 · click-22177f8 · open
Goal: `path_type` narrows the converted value's type for `convert` and `prompt`

Closes #3822
Workspace: /root/bench/ws/rebf815-click-22177f8/click-22177f8
Changed (0): nothing yet

## Map
- examples/validation/validation.py
  L6: def validate_count(ctx, param, value):
  L12: class URL(click.ParamType[urlparse.ParseResult]):
  L15: def convert(self, value, param, ctx) -> urlparse.ParseResult:
  L34: def cli(count, foo, url):
- src/click/termui.py
  L58: _HIDDEN_INPUT_MASK = "'***'"
  L61: def _mask_hidden_input(message: str, value: str) -> str:
  L79: def hidden_prompt_func(prompt: str) -> str:
  L85: def _readline_prompt(func: t.Callable[[str], str], text: str, err: bool) -> str:
  L108: def _build_prompt( text: str, suffix: str, show_default: bool | str = False, default: object | None = None, show_choices: bool = True, type: object | None = None, ) -> str:
  L128: def _format_default(default: V) -> V | str:
  L139: def prompt( text: str, default: str | None = None, hide_input: bool = False, confirmation_prompt: bool | str = False, type: None = None, value_proc: None = None, prompt_suffix: str = ": ", show_default: bool | str = True, err: bool = False, show_choices: bool = True, ) -> str:
  L289: def confirm( text: str, default: bool | None = False, abort: bool = False, prompt_suffix: str = ": ", show_default: bool = True, err: bool = False, ) -> bool:
  L348: def get_pager_file( color: bool | None = None, ) -> t.ContextManager[t.TextIO]:
  L367: def echo_via_pager( text_or_generator: cabc.Iterable[str] | t.Callable[[], cabc.Iterable[str]] | str, color: bool | None = None, ) -> None:
  L403: def progressbar( *, length: int, label: str | None = None, hidden: bool = False, show_eta: bool = True, show_percent: bool | None = None, show_pos: bool = False, fill_char: str = "#", empty_char: str = "-", bar_template: str = "%(label)s [%(bar)s] %(info)s", info_sep: str = " ", width: int = 36, file: t.TextIO | None = None, color: bool | None = None, update_min_steps: int = 1, ) -> ProgressBar[int]:
  L602: def clear() -> None:
  L616: def _interpret_color(color: int | tuple[int, int, int] | str, offset: int = 0) -> str:
  L641: def style( text: t.Any, fg: int | tuple[int, int, int] | str | None = None, bg: int | tuple[int, int, int] | str | None = None, bold: bool | None = None, dim: bool | None = None, underline: bool | None = None, overline: bool | None = None, italic: bool | None = None, blink: bool | None = None, reverse: bool | None = None, strikethrough: bool | None = None, reset: bool = True, ) -> str:
  L768: def unstyle(text: str) -> str:
  L780: def secho( message: t.Any | None = None, file: t.IO[t.AnyStr] | None = None, nl: bool = True, err: bool = False, color: bool | None = None, **styles: t.Any, ) -> None:
  L815: def edit( text: bytes | bytearray, editor: str | None = None, env: cabc.Mapping[str, str] | None = None, require_save: bool = False, extension: str = ".txt", ) -> bytes | None:
  L923: def launch(url: str, wait: bool = False, locate: bool = False) -> int:
  L957: def getchar(echo: bool = False) -> str:
  L987: def raw_terminal() -> AbstractContextManager[int]:
  L993: def pause(info: str | None = None, err: bool = False) -> None:
- src/click/types.py
  L49: class ParamTypeInfoDict(t.TypedDict):
  L54: class ParamType(t.Generic[_ValueT_co, _InputT_contra], abc.ABC):
  L111: def to_info_dict(self) -> ParamTypeInfoDict:
  L133: def __call__( self, value: None, param: Parameter | None = None, ctx: Context | None = None, ) -> None:
  L158: def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
  L161: def get_missing_message(self, param: Parameter, ctx: Context | None) -> str | None:
  L168: def convert( self, value: _InputT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
  L194: def split_envvar_value(self, rv: str) -> cabc.Sequence[str]:
  L204: def fail( self, message: str, param: Parameter | None = None, ctx: Context | None = None, ) -> t.NoReturn:
  L213: def shell_complete( self, ctx: Context, param: Parameter, incomplete: str ) -> list[CompletionItem]:
  L231: class CompositeParamType(ParamType[_ValueT_co]):
  L236: def arity(self) -> int:
  L253: class FuncParamType(ParamType[_ValueT_co], t.Generic[_ValueT_contra, _ValueT_co]):
  L257: def __init__(self, func: t.Callable[[_ValueT_contra], _ValueT_co]) -> None:
  L261: def to_info_dict(self) -> FuncParamTypeInfoDict[_ValueT_contra, _ValueT_co]:
  L264: def convert( self, value: _ValueT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
  L281: class UnprocessedParamType(ParamType[t.Any]):
  L284: def convert( self, value: _ValueT, param: Parameter | None, ctx: Context | None ) -> _ValueT:
  L289: def __repr__(self) -> str:
  L293: class StringParamType(ParamType[str]):
  L296: def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> str:
  L314: def __repr__(self) -> str:
  L331: class Choice(ParamType[_ValueT_co], t.Generic[_ValueT_co]):
  L364: def __init__( self, choices: cabc.Iterable[_ValueT_co], case_sensitive: bool = True ) -> None:
  L370: def to_info_dict(self) -> ChoiceInfoDict[_ValueT_co]:
  L377: def _normalized_mapping( self, ctx: Context | None = None ) -> cabc.Mapping[_ValueT_co, str]:
  L395: def normalize_choice(self, choice: object, ctx: Context | None) -> str:
  L415: def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
  L433: def get_missing_message(self, param: Parameter, ctx: Context | None) -> str:
  L443: def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- src/click/core.py
  L60: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
  L63: def _complete_visible_commands( ctx: Context, incomplete: str ) -> cabc.Iterator[tuple[str, Command]]:
  L82: def _check_nested_chain( base_command: Group, cmd_name: str, cmd: Command, register: bool = False ) -> None:
  L102: def _echo_aborted() -> None:
  L107: def _format_deprecated_label(deprecated: bool | str) -> str:
  L115: def _format_deprecated_suffix(deprecated: bool | str) -> str:
  L124: def batch(iterable: cabc.Iterable[V], batch_size: int) -> list[tuple[V, ...]]:
  L129: def augment_usage_errors( ctx: Context, param: Parameter | None = None ) -> cabc.Generator[None]:
  L147: def iter_params_for_processing( invocation_order: cabc.Sequence[Parameter], declaration_order: cabc.Sequence[Parameter], ) -> list[Parameter]:
  L174: class ParameterSource(enum.IntEnum):
  L213: class Context:
  L345: def __init__( self, command: Command, parent: Context | None = None, info_name: str | None = None, obj: t.Any | None = None, auto_envvar_prefix: str | None = None, default_map: cabc.MutableMapping[str, t.Any] | None = None, terminal_width: int | None = None, max_content_width: int | None = None, resilient_parsing: bool = False, allow_extra_args: bool | None = None, allow_interspersed_args: bool | None = None, ignore_unknown_options: bool | None = None, help_option_names: list[str] | None = None, token_normalize_func: t.Callable[[str], str] | None = None, color: bool | None = None, show_default: bool | None = None, ) -> None: #: the parent context or `None` if none exists.
  L522: def protected_args(self) -> list[str]:
  L533: def to_info_dict(self) -> dict[str, t.Any]:
  L554: def __enter__(self) -> Self:
  L559: def __exit__( self, exc_type: type[BaseException] | None, exc_value: BaseException | None, tb: TracebackType | None, ) -> bool | None:
  L574: def scope(self, cleanup: bool = True) -> cabc.Generator[Context]:
  L612: def meta(self) -> dict[str, t.Any]:
  L639: def make_formatter(self) -> HelpFormatter:
  L653: def with_resource(self, context_manager: AbstractContextManager[V]) -> V:
  L682: def call_on_close(self, f: t.Callable[..., t.Any]) -> t.Callable[..., t.Any]:
  L694: def close(self) -> None:
  L701: def _close_with_exception_info( self, exc_type: type[BaseException] | None, exc_value: BaseException | None, tb: TracebackType | None, ) -> bool | None:
  L720: def command_path(self) -> str:
  L738: def find_root(self) -> Context:
  L745: def find_object(self, object_type: type[V]) -> V | None:
  L757: def ensure_object(self, object_type: type[V]) -> V:
  L766: def _default_map_has(self, name: str | None) -> bool:
  L781: def lookup_default( self, name: str, call: t.Literal[True] = True ) -> t.Any | None:
  L812: def fail(self, message: str) -> t.NoReturn:
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
- repo map (177 files):
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
  tests/ (58 files)
  uv.lock

## Code
- sym:10492 ParamType.convert (src/click/types.py:168-192)
  def convert( self, value: _InputT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:10503 UnprocessedParamType.convert (src/click/types.py:284-287)
  def convert( self, value: _ValueT, param: Parameter | None, ctx: Context | None ) -> _ValueT:
- sym:10501 FuncParamType.convert (src/click/types.py:264-278)
  def convert( self, value: _ValueT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:10421 prompt (src/click/termui.py:139-150)
  def prompt( text: str, default: str | None = None, hide_input: bool = False, confirmation_prompt: bool | str = False, type: None = None, value_proc: None = None, prompt_suffix: str = ": ", show_default: bool | str = True, err: bool = False, show_choices: bool = True, ) -> str:
- sym:10485 ParamType (src/click/types.py:54-228)
  class ParamType(t.Generic[_ValueT_co, _InputT_contra], abc.ABC):
- sym:10223 Parameter.type_cast_value (src/click/core.py:2557-2611)
  def type_cast_value(self, ctx: Context, value: t.Any) -> t.Any:
- sym:10561 Path (src/click/types.py:1048-1236)
  class Path(ParamType[str | bytes | os.PathLike[str]]):
- sym:10565 Path.convert (src/click/types.py:1146-1218)
  def convert( self, value: str | os.PathLike[str], param: Parameter | None, ctx: Context | None, ) -> str | bytes | os.PathLike[str]:
- sym:10564 Path.coerce_path_result (src/click/types.py:1133-1144)
  def coerce_path_result( self, value: str | os.PathLike[str] ) -> str | bytes | os.PathLike[str]:
- sym:10528 _NumberParamTypeBase.convert (src/click/types.py:587-599)
  def convert( self, value: _ValueT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:9969 URL.convert (examples/validation/validation.py:15-24)
  def convert(self, value, param, ctx) -> urlparse.ParseResult:
- sym:10234 Option.__init__ (src/click/core.py:2979-3058)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, show_default: bool | str | None = None, prompt: bool | str = False, confirmation_prompt: bool | str = False, prompt_required: bool = True, hide_input: bool = False, is_flag: bool | None = None, flag_value: t.Any = UNSET, multiple: bool = False, count: bool = False, allow_from_autoenv: bool = True, type: types.ParamType[t.Any] | t.Any | None = None, help: str | None = None, hidden: bool = False, show_choices: bool = True, show_envvar: bool = False, deprecated: bool | str = False, **attrs: t.Any, ) -> None:
- sym:10525 DateTime.convert (src/click/types.py:555-576)
  def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> datetime:
- sym:10575 convert_type (src/click/types.py:1332-1332)
  def convert_type(ty: None, default: None = None) -> StringParamType:
- sym:10520 DateTime (src/click/types.py:511-579)
  class DateTime(ParamType[datetime]):
- sym:10527 _NumberParamTypeBase (src/click/types.py:582-599)
  class _NumberParamTypeBase( ParamType[_ValueT_co], t.Generic[_ValueT_contra, _ValueT_co] ):
- sym:10498 FuncParamType (src/click/types.py:253-278)
  class FuncParamType(ParamType[_ValueT_co], t.Generic[_ValueT_contra, _ValueT_co]):
- sym:10502 UnprocessedParamType (src/click/types.py:281-290)
  class UnprocessedParamType(ParamType[t.Any]):
- sym:10557 File.convert (src/click/types.py:973-1016)
  def convert( self, value: str | os.PathLike[str] | t.IO[t.Any], param: Parameter | None, ctx: Context | None, ) -> t.IO[t.Any]:
- sym:10562 Path.__init__ (src/click/types.py:1093-1120)
  def __init__( self, exists: bool = False, file_okay: bool = True, dir_okay: bool = True, writable: bool = False, readable: bool = True, resolve_path: bool = False, allow_dash: bool = False, path_type: type | None = None, executable: bool = False, ) -> None:
- sym:10515 Choice.convert (src/click/types.py:443-465)
  def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- sym:10573 Tuple.convert (src/click/types.py:1274-1293)
  def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> tuple[t.Any, ...]:
- sym:10547 BoolParamType.convert (src/click/types.py:859-871)
  def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> bool:
- sym:10238 Option._pick_type (src/click/core.py:3115-3152)
  def _pick_type( self, type_arg: t.Any, flag_value: t.Any, count: bool, is_flag: bool, ) -> types.ParamType[t.Any]:
- sym:11256 test_prompt_default_round_trips_through_type (tests/test_utils/test_prompt.py:17-31)
  def test_prompt_default_round_trips_through_type(capfd, monkeypatch):
- sym:11283 DoublingType.convert (tests/typing/typing_prompt.py:35-38)
  def convert( self, value: str, param: click.Parameter | None, ctx: click.Context | None ) -> int:
- sym:11282 DoublingType (tests/typing/typing_prompt.py:32-38)
  class DoublingType(click.ParamType[int, str]):
- sym:10119 Context (src/click/core.py:213-961)
  class Context:
- sym:10140 Context.fail (src/click/core.py:812-818)
  def fail(self, message: str) -> t.NoReturn:
- sym:10210 Parameter (src/click/core.py:2215-2883)
  class Parameter(ABC):
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
- sym:11160 test_hide_input_confirmation_prompt_mismatch_unaffected (tests/test_termui.py:1945-1963)
  def test_hide_input_confirmation_prompt_mismatch_unaffected(runner):
- sym:10811 test_parameter_source_during_paramtype_convert (tests/test_defaults.py:275-315)
  def test_parameter_source_during_paramtype_convert(runner):
- sym:11202 test_range (tests/test_types/test_FloatRange.py:16-17)
  def test_range(type, value, expect):
- sym:10910 test_callback_validates_prompt (tests/test_options.py:896-909)
  def test_callback_validates_prompt(runner, monkeypatch):
- sym:11214 test_path_surrogates (tests/test_types/test_Path.py:90-143)
  def test_path_surrogates(tmp_path, monkeypatch):
- sym:11144 _CustomTypeWithRawValue.convert (tests/test_termui.py:1743-1746)
  def convert(self, value, param, ctx):
- sym:11212 test_path_resolve_symlink (tests/test_types/test_Path.py:52-72)
  def test_path_resolve_symlink(tmp_path, runner):
- sym:11128 test_tempfile_pager_closes_file_before_unlink (tests/test_termui.py:1262-1308)
  def test_tempfile_pager_closes_file_before_unlink(monkeypatch):
- sym:11147 _MixedQuotedAndRawType (tests/test_termui.py:1763-1771)
  class _MixedQuotedAndRawType(click.ParamType):
- sym:11210 test_path_dash_no_byteswarning (tests/test_types/test_Path.py:13-33)
  def test_path_dash_no_byteswarning():
- sym:11257 test_prompt_default_validated_by_type (tests/test_utils/test_prompt.py:34-40)
  def test_prompt_default_validated_by_type(capfd, monkeypatch):
- sym:11217 test_path_type (tests/test_types/test_Path.py:177-185)
  def test_path_type(runner, cls, expect):
- sym:11255 test_prompt_cast_default (tests/test_utils/test_prompt.py:10-14)
  def test_prompt_cast_default(capfd, monkeypatch):
- sym:11139 FLAG_VALUE_PROMPT_CASES (tests/test_termui.py:1587-1669)
  FLAG_VALUE_PROMPT_CASES = [
- sym:11148 _MixedQuotedAndRawType.convert (tests/test_termui.py:1770-1771)
  def convert(self, value, param, ctx):
- sym:11143 _CustomTypeWithRawValue (tests/test_termui.py:1740-1746)
  class _CustomTypeWithRawValue(click.ParamType):
- sym:11216 test_invalid_path_with_esc_sequence (tests/test_types/test_Path.py:160-165)
  def test_invalid_path_with_esc_sequence():
- sym:11154 _MultiRawType.convert (tests/test_termui.py:1797-1798)
  def convert(self, value, param, ctx):
- sym:11284 SimpleType (tests/typing/typing_prompt.py:46-47)
  class SimpleType(click.ParamType[int]):
- sym:11219 test_explicit_container_type_splits_string (tests/test_types/test_convert_type.py:40-48)
  def test_explicit_container_type_splits_string(container_type, value, expected):
- sym:11151 _RejectAllRawType (tests/test_termui.py:1783-1789)
  class _RejectAllRawType(click.ParamType):
- sym:11152 _RejectAllRawType.convert (tests/test_termui.py:1788-1789)
  def convert(self, value, param, ctx):
- sym:11141 _CustomTypeNoValue (tests/test_termui.py:1731-1737)
  class _CustomTypeNoValue(click.ParamType):
- sym:11206 test_range (tests/test_types/test_IntRange.py:20-21)
  def test_range(type, value, expect):
- sym:11218 test_convert_type_from_container_default (tests/test_types/test_convert_type.py:25-30)
  def test_convert_type_from_container_default(default):
- sym:11142 _CustomTypeNoValue.convert (tests/test_termui.py:1734-1737)
  def convert(self, value, param, ctx):
- sym:10627 _prompting_command (tests/test_abort_interrupt.py:59-65)
  def _prompting_command():
- sym:10877 test_deprecated_prompt (tests/test_options.py:126-128)
  def test_deprecated_prompt(runner):
- sym:10898 test_dynamic_default_help_unset (tests/test_options.py:713-728)
  def test_dynamic_default_help_unset(runner):
- sym:10899 test_dynamic_default_help_text (tests/test_options.py:731-746)
  def test_dynamic_default_help_text(runner):
- sym:10933 test_string_show_default_shows_custom_string_in_prompt (tests/test_options.py:1359-1369)
  def test_string_show_default_shows_custom_string_in_prompt(runner):
- sym:11043 test_invoke_with_logger_and_prompt (tests/test_stream_lifecycle.py:204-217)
  def test_invoke_with_logger_and_prompt():
- sym:11080 test_choices_list_in_prompt (tests/test_termui.py:257-279)
  def test_choices_list_in_prompt(runner, monkeypatch):
- sym:11081 test_file_prompt_default_format (tests/test_termui.py:285-292)
  def test_file_prompt_default_format(runner, file_kwargs):
- sym:11130 test_prompt_required_with_required (tests/test_termui.py:1326-1333)
  def test_prompt_required_with_required(runner, prompt_required, required, args, expect):
- sym:11131 test_prompt_required_false (tests/test_termui.py:1353-1365)
  def test_prompt_required_false(runner, args, expect):
- sym:11132 test_confirmation_prompt (tests/test_termui.py:1377-1394)
  def test_confirmation_prompt(runner, prompt, input, default, expect):
- sym:11134 test_false_show_default_cause_no_default_display_in_prompt (tests/test_termui.py:1447-1456)
  def test_false_show_default_cause_no_default_display_in_prompt(runner):
- sym:11140 test_flag_value_prompt (tests/test_termui.py:1679-1728)
  def test_flag_value_prompt( runner, opt_decls, opt_params, args, prompt, input, expected ):
- sym:11159 test_hide_input_error_message (tests/test_termui.py:1929-1942)
  def test_hide_input_error_message( runner, type, user_input, expected_fragment, unexpected_fragment ):
- sym:11164 test_echo_stdin_prompts (tests/test_testing.py:52-89)
  def test_echo_stdin_prompts():
- sym:11166 test_prompts (tests/test_testing.py:115-134)
  def test_prompts():
- sym:11201 test_file_surrogates (tests/test_types/test_File.py:13-31)
  def test_file_surrogates(type, tmp_path):
- sym:11203 test_range_fail (tests/test_types/test_FloatRange.py:27-31)
  def test_range_fail(type, value, expect):

---
tokens: 7027/8000
