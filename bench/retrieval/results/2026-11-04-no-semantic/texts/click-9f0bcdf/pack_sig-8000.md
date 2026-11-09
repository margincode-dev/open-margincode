## Task
# click-9f0bcdf · click-9f0bcdf-2 · open
Goal: Add `Parameter.spec` and deprecate `human_readable_name`
Workspace: /root/bench/ws/rebf815-click-9f0bcdf/click-9f0bcdf-2
Changed (0): nothing yet

## Map
- src/click/core.py
  L61: _HELP_OPTION_STORAGE_NAME = "_click_default_help"
  L64: def _complete_visible_commands( ctx: Context, incomplete: str ) -> cabc.Iterator[tuple[str, Command]]:
  L83: def _check_nested_chain( base_command: Group, cmd_name: str, cmd: Command, register: bool = False ) -> None:
  L103: def _echo_aborted() -> None:
  L108: def _outside_click_stacklevel() -> int:
  L128: def _format_deprecated_label(deprecated: bool | str) -> str:
  L136: def _format_deprecated_suffix(deprecated: bool | str) -> str:
  L145: def batch(iterable: cabc.Iterable[V], batch_size: int) -> list[tuple[V, ...]]:
  L150: def augment_usage_errors( ctx: Context, param: Parameter | None = None ) -> cabc.Generator[None]:
  L168: def iter_params_for_processing( invocation_order: cabc.Sequence[Parameter], declaration_order: cabc.Sequence[Parameter], ) -> list[Parameter]:
  L195: class ParameterSource(enum.IntEnum):
  L234: class Context:
  L366: def __init__( self, command: Command, parent: Context | None = None, info_name: str | None = None, obj: t.Any | None = None, auto_envvar_prefix: str | None = None, default_map: cabc.MutableMapping[str, t.Any] | None = None, terminal_width: int | None = None, max_content_width: int | None = None, resilient_parsing: bool = False, allow_extra_args: bool | None = None, allow_interspersed_args: bool | None = None, ignore_unknown_options: bool | None = None, help_option_names: list[str] | None = None, token_normalize_func: t.Callable[[str], str] | None = None, color: bool | None = None, show_default: bool | None = None, ) -> None: #: the parent context or `None` if none exists.
  L543: def protected_args(self) -> list[str]:
  L554: def to_info_dict(self) -> dict[str, t.Any]:
  L575: def __enter__(self) -> Self:
  L580: def __exit__( self, exc_type: type[BaseException] | None, exc_value: BaseException | None, tb: TracebackType | None, ) -> bool | None:
  L595: def scope(self, cleanup: bool = True) -> cabc.Generator[Context]:
  L633: def meta(self) -> dict[str, t.Any]:
  L660: def make_formatter(self) -> HelpFormatter:
  L674: def with_resource(self, context_manager: AbstractContextManager[V]) -> V:
  L703: def call_on_close(self, f: t.Callable[..., t.Any]) -> t.Callable[..., t.Any]:
  L715: def close(self) -> None:
  L722: def _close_with_exception_info( self, exc_type: type[BaseException] | None, exc_value: BaseException | None, tb: TracebackType | None, ) -> bool | None:
  L741: def command_path(self) -> str:
  L759: def find_root(self) -> Context:
  L766: def find_object(self, object_type: type[V]) -> V | None:
  L778: def ensure_object(self, object_type: type[V]) -> V:
  L787: def _default_map_has(self, name: str | None) -> bool:
  L802: def lookup_default( self, name: str, call: t.Literal[True] = True ) -> t.Any | None:
- src/click/_compat.py
  L13: CYGWIN = sys.platform.startswith("cygwin")
  L14: WIN = sys.platform.startswith("win")
  L22: def _make_text_stream( stream: t.BinaryIO, encoding: str | None, errors: str | None, force_readable: bool = False, force_writable: bool = False, ) -> t.TextIO:
  L43: def is_ascii_encoding(encoding: str) -> bool:
  L51: def get_best_encoding(stream: t.IO[t.Any]) -> str:
  L59: class _NonClosingTextIOWrapper(io.TextIOWrapper):
  L60: def __init__( self, stream: t.BinaryIO, encoding: str | None, errors: str | None, force_readable: bool = False, force_writable: bool = False, **extra: t.Any, ) -> None:
  L74: def __del__(self) -> None:
  L80: def isatty(self) -> bool: # https://bitbucket.org/pypy/pypy/issue/1803
  L85: class _FixupStream:
  L95: def __init__( self, stream: t.BinaryIO, force_readable: bool = False, force_writable: bool = False, ):
  L105: def __getattr__(self, name: str) -> t.Any:
  L108: def read1(self, size: int) -> bytes:
  L116: def readable(self) -> bool:
  L128: def writable(self) -> bool:
  L143: def seekable(self) -> bool:
  L154: def _is_binary_reader(stream: t.IO[t.Any], default: bool = False) -> bool:
  L163: def _is_binary_writer(stream: t.IO[t.Any], default: bool = False) -> bool:
  L176: def _find_binary_reader(stream: t.IO[t.Any]) -> t.BinaryIO | None: # We need to figure out if the given stream is already binary. # This can happen because the official docs recommend detaching # the streams to get binary streams. Some code might do this, so # we need to deal with this case explicitly.
  L194: def _find_binary_writer(stream: t.IO[t.Any]) -> t.BinaryIO | None: # We need to figure out if the given stream is already binary. # This can happen because the official docs recommend detaching # the streams to get binary streams. Some code might do this, so # we need to deal with this case explicitly.
  L212: def _stream_is_misconfigured(stream: t.TextIO) -> bool:
  L221: def _is_compat_stream_attr(stream: t.TextIO, attr: str, value: str | None) -> bool:
  L230: def _is_compatible_text_stream( stream: t.TextIO, encoding: str | None, errors: str | None ) -> bool:
  L241: def _force_correct_text_stream( text_stream: t.IO[t.Any], encoding: str | None, errors: str | None, is_binary: t.Callable[[t.IO[t.Any], bool], bool], find_binary: t.Callable[[t.IO[t.Any]], t.BinaryIO | None], force_readable: bool = False, force_writable: bool = False, ) -> t.TextIO:
  L287: def _force_correct_text_reader( text_reader: t.IO[t.Any], encoding: str | None, errors: str | None, force_readable: bool = False, ) -> t.TextIO:
  L303: def _force_correct_text_writer( text_writer: t.IO[t.Any], encoding: str | None, errors: str | None, force_writable: bool = False, ) -> t.TextIO:
  L319: def get_binary_stdin() -> t.BinaryIO:
  L326: def get_binary_stdout() -> t.BinaryIO:
  L333: def get_binary_stderr() -> t.BinaryIO:
  L340: def get_text_stdin(encoding: str | None = None, errors: str | None = None) -> t.TextIO:
- src/click/_winconsole.py
  L53: STDIN_HANDLE = GetStdHandle(-10)
  L54: STDOUT_HANDLE = GetStdHandle(-11)
  L55: STDERR_HANDLE = GetStdHandle(-12)
  L60: ERROR_SUCCESS = 0
  L61: ERROR_NOT_ENOUGH_MEMORY = 8
  L62: ERROR_OPERATION_ABORTED = 995
  L64: EOF = b"\x1a"
  L65: MAX_BYTES_WRITTEN = 32767
  L114: class _WindowsConsoleRawIOBase(io.RawIOBase):
  L115: def __init__(self, handle: int | None) -> None:
  L118: def isatty(self) -> t.Literal[True]:
  L123: class _WindowsConsoleReader(_WindowsConsoleRawIOBase):
  L124: def readable(self) -> t.Literal[True]:
  L127: def readinto(self, b: Buffer) -> int:
  L158: class _WindowsConsoleWriter(_WindowsConsoleRawIOBase):
  L159: def writable(self) -> t.Literal[True]:
  L163: def _get_error_message(errno: int) -> str:
  L170: def write(self, b: Buffer) -> int:
  L190: class ConsoleStream:
  L191: def __init__(self, text_stream: t.TextIO, byte_stream: t.BinaryIO) -> None:
  L196: def name(self) -> str:
  L199: def write(self, x: t.AnyStr) -> int:
  L208: def writelines(self, lines: cabc.Iterable[t.AnyStr]) -> None:
  L212: def __getattr__(self, name: str) -> t.Any:
  L215: def isatty(self) -> bool:
  L218: def __repr__(self) -> str:
  L222: def _get_text_stdin(buffer_stream: t.BinaryIO) -> t.TextIO:
  L232: def _get_text_stdout(buffer_stream: t.BinaryIO) -> t.TextIO:
  L242: def _get_text_stderr(buffer_stream: t.BinaryIO) -> t.TextIO:
  L259: def _is_console(f: t.TextIO) -> bool:
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
- src/click/types.py
  L60: class ParamTypeInfoDict(t.TypedDict):
  L65: class ParamType(t.Generic[_ValueT_co, _InputT_contra], abc.ABC):
  L122: def to_info_dict(self) -> ParamTypeInfoDict:
  L144: def __call__( self, value: None, param: Parameter | None = None, ctx: Context | None = None, ) -> None:
  L169: def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
  L172: def get_missing_message(self, param: Parameter, ctx: Context | None) -> str | None:
  L179: def convert( self, value: _InputT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
  L205: def split_envvar_value(self, rv: str) -> cabc.Sequence[str]:
  L215: def fail( self, message: str, param: Parameter | None = None, ctx: Context | None = None, ) -> t.NoReturn:
  L224: def shell_complete( self, ctx: Context, param: Parameter, incomplete: str ) -> list[CompletionItem]:
  L242: class CompositeParamType(ParamType[_ValueT_co]):
  L247: def arity(self) -> int:
  L264: class FuncParamType(ParamType[_ValueT_co], t.Generic[_ValueT_contra, _ValueT_co]):
  L268: def __init__(self, func: t.Callable[[_ValueT_contra], _ValueT_co]) -> None:
  L272: def to_info_dict(self) -> FuncParamTypeInfoDict[_ValueT_contra, _ValueT_co]:
  L275: def convert( self, value: _ValueT_contra, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
  L292: class UnprocessedParamType(ParamType[t.Any]):
  L295: def convert( self, value: _ValueT, param: Parameter | None, ctx: Context | None ) -> _ValueT:
  L300: def __repr__(self) -> str:
  L304: class StringParamType(ParamType[str]):
  L307: def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> str:
  L325: def __repr__(self) -> str:
  L342: class Choice(ParamType[_ValueT_co], t.Generic[_ValueT_co]):
  L375: def __init__( self, choices: cabc.Iterable[_ValueT_co], case_sensitive: bool = True ) -> None:
  L381: def to_info_dict(self) -> ChoiceInfoDict[_ValueT_co]:
  L388: def _normalized_mapping( self, ctx: Context | None = None ) -> cabc.Mapping[_ValueT_co, str]:
  L406: def normalize_choice(self, choice: object, ctx: Context | None) -> str:
  L426: def get_metavar(self, param: Parameter, ctx: Context) -> str | None:
  L444: def get_missing_message(self, param: Parameter, ctx: Context | None) -> str:
  L454: def convert( self, value: t.Any, param: Parameter | None, ctx: Context | None ) -> _ValueT_co:
- examples/aliases/aliases.py
  L7: class Config:
  L10: def __init__(self):
  L14: def add_alias(self, alias, cmd):
  L17: def read_config(self, filename):
  L25: def write_config(self, filename):
  L37: class AliasedGroup(click.Group):
  L42: def get_command(self, ctx, cmd_name): # Step one: built-in commands as normal
  L70: def resolve_command(self, ctx, args): # always return the command's name, not the alias
  L76: def read_config(ctx, param, value):
  L97: def cli():
  L102: def push():
  L108: def pull():
  L114: def clone():
  L120: def commit():
  L127: def status(config):
  L139: def alias(config, alias_, cmd, config_file):
- examples/complex/complex/cli.py
  L7: CONTEXT_SETTINGS = dict(auto_envvar_prefix="COMPLEX")
  L10: class Environment:
  L11: def __init__(self):
  L15: def log(self, msg, *args):
  L21: def vlog(self, msg, *args):
  L31: class ComplexCLI(click.Group):
  L32: def list_commands(self, ctx):
  L40: def get_command(self, ctx, name):
  L56: def cli(ctx, verbose, home):
- examples/naval/naval.py
- src/click/__init__.py
  L76: def __getattr__(name: str) -> object:
- src/click/_termui_impl.py
- src/click/_utils.py
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
- sym:334 Parameter.human_readable_name (src/click/core.py:2520-2524)
  def human_readable_name(self) -> str:
- sym:375 Argument.human_readable_name (src/click/core.py:3838-3841)
  def human_readable_name(self) -> str:
- sym:349 Parameter.get_error_hint (src/click/core.py:2929-2937)
  def get_error_hint(self, ctx: Context | None) -> str:
- sym:380 Argument.get_error_hint (src/click/core.py:3899-3902)
  def get_error_hint(self, ctx: Context | None) -> str:
- sym:373 Argument (src/click/core.py:3800-3905)
  class Argument(Parameter):
- sym:683 Path.convert (src/click/types.py:1161-1233)
  def convert( self, value: str | os.PathLike[str], param: Parameter | None, ctx: Context | None, ) -> _PathT_co:
- sym:680 Path.__init__ (src/click/types.py:1107-1134)
  def __init__( self, exists: bool = False, file_okay: bool = True, dir_okay: bool = True, writable: bool = False, readable: bool = True, resolve_path: bool = False, allow_dash: bool = False, path_type: type[_PathT_co] | None = None, executable: bool = False, ) -> None:
- sym:267 Context.set_parameter_source (src/click/core.py:957-964)
  def set_parameter_source(self, name: str, source: ParameterSource) -> None:
- sym:465 _unpack_args (src/click/parser.py:51-108)
  def _unpack_args( args: cabc.Sequence[str], nargs_spec: cabc.Sequence[int] ) -> tuple[cabc.Sequence[str | cabc.Sequence[str | T_UNSET] | T_UNSET], list[str]]:
- sym:268 Context.get_parameter_source (src/click/core.py:966-982)
  def get_parameter_source(self, name: str) -> ParameterSource | None:
- sym:365 Option.get_help_spec (src/click/core.py:3464-3493)
  def get_help_spec(self, ctx: Context) -> str:
- sym:679 Path (src/click/types.py:1059-1251)
  class Path(ParamType[_PathT_co, str | os.PathLike[str]], t.Generic[_PathT_co]):
- sym:97 _FixupStream (src/click/_compat.py:85-151)
  class _FixupStream:
- sym:274 Command.get_params (src/click/core.py:1128-1182)
  def get_params(self, ctx: Context) -> list[Parameter]:
- sym:98 _FixupStream.__init__ (src/click/_compat.py:95-103)
  def __init__( self, stream: t.BinaryIO, force_readable: bool = False, force_writable: bool = False, ):
- sym:101 _FixupStream.readable (src/click/_compat.py:116-126)
  def readable(self) -> bool:
- sym:206 _WindowsConsoleReader.readable (src/click/_winconsole.py:124-125)
  def readable(self) -> t.Literal[True]:
- sym:112 _force_correct_text_reader (src/click/_compat.py:287-300)
  def _force_correct_text_reader( text_reader: t.IO[t.Any], encoding: str | None, errors: str | None, force_readable: bool = False, ) -> t.TextIO:
- sym:366 Option.get_help_record (src/click/core.py:3495-3518)
  def get_help_record(self, ctx: Context) -> tuple[str, str] | None:
- sym:90 _make_text_stream (src/click/_compat.py:22-40)
  def _make_text_stream( stream: t.BinaryIO, encoding: str | None, errors: str | None, force_readable: bool = False, force_writable: bool = False, ) -> t.TextIO:
- sym:94 _NonClosingTextIOWrapper.__init__ (src/click/_compat.py:60-72)
  def __init__( self, stream: t.BinaryIO, encoding: str | None, errors: str | None, force_readable: bool = False, force_writable: bool = False, **extra: t.Any, ) -> None:
- sym:681 Path.to_info_dict (src/click/types.py:1136-1145)
  def to_info_dict(self) -> PathInfoDict:
- sym:111 _force_correct_text_stream (src/click/_compat.py:241-284)
  def _force_correct_text_stream( text_stream: t.IO[t.Any], encoding: str | None, errors: str | None, is_binary: t.Callable[[t.IO[t.Any], bool], bool], find_binary: t.Callable[[t.IO[t.Any]], t.BinaryIO | None], force_readable: bool = False, force_writable: bool = False, ) -> t.TextIO:
- sym:422 MissingParameter.__str__ (src/click/exceptions.py:224-229)
  def __str__(self) -> str:
- sym:124 _AtomicFile.name (src/click/_compat.py:463-464)
  def name(self) -> str:
- sym:214 ConsoleStream.name (src/click/_winconsole.py:196-197)
  def name(self) -> str:
- sym:328 Parameter.__init__ (src/click/core.py:2356-2438)
  def __init__( self, param_decls: cabc.Sequence[str] | None = None, type: types.ParamType[t.Any] | t.Any | None = None, required: bool = False, # XXX The default historically embed two concepts: # - the declaration of a Parameter object carrying the default (handy to # arbitrage the default value of coupled Parameters sharing the same # self.name, like flag options), # - and the actual value of the default. # It is confusing and is the source of many issues discussed in: # https://github.com/pallets/click/pull/3030 # In the future, we might think of splitting it in two, not unlike # Option.is_flag and Option.flag_value: we could have something like # Parameter.is_default and Parameter.default_value. default: t.Any | t.Callable[[], t.Any] | None = UNSET, callback: t.Callable[[Context, Parameter, t.Any], t.Any] | None = None, nargs: int | None = None, multiple: bool = False, metavar: str | None = None, expose_value: bool = True, is_eager: bool = False, envvar: str | cabc.Sequence[str] | None = None, shell_complete: t.Callable[ [Context, Parameter, str], list[CompletionItem] | list[str] ] | None = None, deprecated: bool | str = False, help: str | None = None, ) -> None:
- sym:346 Parameter.handle_parse_result (src/click/core.py:2835-2921)
  def handle_parse_result( self, ctx: Context, opts: cabc.Mapping[str, t.Any], args: list[str] ) -> tuple[t.Any, list[str]]:
- sym:586 _NamedTextIOWrapper.name (src/click/testing.py:203-204)
  def name(self) -> str:
- sym:689 Tuple.name (src/click/types.py:1282-1283)
  def name(self) -> str: # type: ignore[override]
- sym:8 AliasedGroup.resolve_command (examples/aliases/aliases.py:70-73)
  def resolve_command(self, ctx, args): # always return the command's name, not the alias
- sym:32 ComplexCLI.get_command (examples/complex/complex/cli.py:40-45)
  def get_command(self, ctx, name):
- sym:57 ship_new (examples/naval/naval.py:22-24)
  def ship_new(name):
- sym:87 __getattr__ (src/click/__init__.py:76-144)
  def __getattr__(name: str) -> object:
- sym:91 is_ascii_encoding (src/click/_compat.py:43-48)
  def is_ascii_encoding(encoding: str) -> bool:
- sym:99 _FixupStream.__getattr__ (src/click/_compat.py:105-106)
  def __getattr__(self, name: str) -> t.Any:
- sym:121 open_stream (src/click/_compat.py:374-452)
  def open_stream( filename: str | os.PathLike[str], mode: str = "r", encoding: str | None = None, errors: str | None = "strict", atomic: bool = False, ) -> tuple[t.IO[t.Any], bool]:
- sym:126 _AtomicFile.__getattr__ (src/click/_compat.py:473-474)
  def __getattr__(self, name: str) -> t.Any:
- sym:162 _PagerWriter.__getattr__ (src/click/_termui_impl.py:419-420)
  def __getattr__(self, name: str) -> t.Any:
- sym:168 _tempfilepager (src/click/_termui_impl.py:636-667)
  def _tempfilepager( cmd_path: Path, cmd_params: list[str], color: bool | None = None ) -> t.Iterator[tuple[t.TextIO, bool | None]]:
- sym:186 Sentinel.__repr__ (src/click/_utils.py:18-19)
  def __repr__(self) -> str:
- sym:189 Sentinel.__reduce_ex__ (src/click/_utils.py:27-28)
  def __reduce_ex__(self, protocol: object) -> tuple[t.Any, ...]:
- sym:190 UNSET (src/click/_utils.py:31-31)
  UNSET: t.Literal[Sentinel.UNSET] = Sentinel.UNSET
- sym:217 ConsoleStream.__getattr__ (src/click/_winconsole.py:212-213)
  def __getattr__(self, name: str) -> t.Any:
- sym:219 ConsoleStream.__repr__ (src/click/_winconsole.py:218-219)
  def __repr__(self) -> str:
- sym:226 _complete_visible_commands (src/click/core.py:64-80)
  def _complete_visible_commands( ctx: Context, incomplete: str ) -> cabc.Iterator[tuple[str, Command]]:
- sym:227 _check_nested_chain (src/click/core.py:83-100)
  def _check_nested_chain( base_command: Group, cmd_name: str, cmd: Command, register: bool = False ) -> None:
- sym:230 _format_deprecated_label (src/click/core.py:128-133)
  def _format_deprecated_label(deprecated: bool | str) -> str:
- sym:231 _format_deprecated_suffix (src/click/core.py:136-142)
  def _format_deprecated_suffix(deprecated: bool | str) -> str:
- sym:233 augment_usage_errors (src/click/core.py:150-165)
  def augment_usage_errors( ctx: Context, param: Parameter | None = None ) -> cabc.Generator[None]:
- sym:235 ParameterSource (src/click/core.py:195-231)
  class ParameterSource(enum.IntEnum):
- sym:236 Context (src/click/core.py:234-982)
  class Context:
- sym:253 Context._default_map_has (src/click/core.py:787-799)
  def _default_map_has(self, name: str | None) -> bool:
- sym:254 Context.lookup_default (src/click/core.py:802-804)
  def lookup_default( self, name: str, call: t.Literal[True] = True ) -> t.Any | None:
- sym:262 Context._make_sub_context (src/click/core.py:867-873)
  def _make_sub_context(self, command: Command) -> Context:
- sym:270 Command.__init__ (src/click/core.py:1061-1103)
  def __init__( self, name: str | None, context_settings: cabc.MutableMapping[str, t.Any] | None = None, callback: t.Callable[..., t.Any] | None = None, params: list[Parameter] | None = None, help: str | None = None, epilog: str | None = None, short_help: str | None = None, options_metavar: str | None = "[OPTIONS]", add_help_option: bool = True, no_args_is_help: bool = False, hidden: bool = False, deprecated: bool | str = False, ) -> None: #: the name the command thinks it has. Upon registering a command #: on a :class:`Group` the group will default the command name #: with this information. You should instead use the #: :class:`Context`\'s :attr:`~Context.info_name` attribute.
- sym:271 Command.to_info_dict (src/click/core.py:1105-1114)
  def to_info_dict(self, ctx: Context) -> dict[str, t.Any]:
- sym:272 Command.__repr__ (src/click/core.py:1116-1117)
  def __repr__(self) -> str:
- sym:277 Command.get_help_option_names (src/click/core.py:1203-1216)
  def get_help_option_names(self, ctx: Context) -> list[str]:
- sym:769 test_parameter_name_not_an_identifier_is_deprecated (tests/test_arguments.py:278-293)
  def test_parameter_name_not_an_identifier_is_deprecated(cls, form, decl):
- sym:1025 test_help_spec (tests/test_options.py:87-98)
  def test_help_spec(param_decls, kwargs, hidden, expected):
- sym:1381 test_path_surrogates (tests/test_types/test_Path.py:91-144)
  def test_path_surrogates(tmp_path, monkeypatch):
- sym:1431 test_parameter_duplication (tests/test_utils/test_sentinel.py:90-97)
  def test_parameter_duplication(duplicate):
- sym:953 test_default_map_source (tests/test_defaults.py:254-272)
  def test_default_map_source(runner, args, default_map, expected_value, expected_source):
- sym:767 test_parameter_name_transform (tests/test_arguments.py:237-241)
  def test_parameter_name_transform(cls, form, decl, expect):
- sym:771 test_parameter_name_is_not_nfkc_normalized (tests/test_arguments.py:330-350)
  def test_parameter_name_is_not_nfkc_normalized(runner, decorator, decl, argv):
- sym:763 test_parameter_name_is_an_identifier_or_deprecated (tests/test_arguments.py:166-189)
  def test_parameter_name_is_an_identifier_or_deprecated(count, expose_value):

---
tokens: 7134/8000
